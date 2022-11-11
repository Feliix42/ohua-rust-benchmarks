use crate::segments::Segments;
use crate::Nucleotide;
use std::collections::VecDeque;
use std::ops::Range;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use stm::{atomically, det_atomically, dtm, freeze, DTMHandle, TVar};
use stm_datastructures::THashSet;

#[derive(Clone, Debug)]
pub struct SequencerItem {
    pub segment: Arc<Vec<Nucleotide>>,
    pub links: TVar<LinkInfo>,
}

#[derive(Clone, Copy, Debug)]
pub struct LinkInfo {
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub overlap_with_prev: usize,
}

impl Default for LinkInfo {
    fn default() -> Self {
        Self {
            prev: None,
            next: None,
            overlap_with_prev: 0,
        }
    }
}

impl From<Vec<Nucleotide>> for SequencerItem {
    fn from(nucleotide_sequence: Vec<Nucleotide>) -> Self {
        SequencerItem {
            segment: Arc::new(nucleotide_sequence),
            links: TVar::new(LinkInfo::default()),
        }
    }
}

/// Splits the input vector into evenly sized vectors for `split_size` workers.
fn split_evenly(
    mut to_split: Vec<Vec<Nucleotide>>,
    split_size: usize,
) -> Vec<Vec<Vec<Nucleotide>>> {
    let l = to_split.len() / split_size;
    let mut rest = to_split.len() % split_size;

    let mut splitted = Vec::new();

    for t_num in 0..split_size {
        splitted.push(Vec::with_capacity(l));
        if rest > 0 {
            splitted[t_num] = to_split.split_off(to_split.len() - l - 1);
            rest -= 1;
        } else {
            if to_split.len() <= l {
                splitted[t_num] = to_split.split_off(0);
            } else {
                splitted[t_num] = to_split.split_off(to_split.len() - l);
            }
        }
    }

    splitted
}

pub fn deduplicate(segments: Segments, threadcount: usize) -> VecDeque<SequencerItem> {
    // Step 1: deduplicate all segments by placing them in a hashmap
    let tmp: Arc<THashSet<Vec<Nucleotide>>> = Arc::new(THashSet::new(segments.orig_gene_length));
    let rest = segments.contents.len() % threadcount;
    let to_dedup = split_evenly(segments.contents, threadcount);

    let (done_sx, done_rx): (Vec<Sender<()>>, Vec<Receiver<()>>) =
        (0..threadcount).map(|_| std::sync::mpsc::channel()).unzip();
    let (dtm_sx, dtm_rxs): (Vec<Sender<DTMHandle>>, Vec<Receiver<DTMHandle>>) =
        (0..threadcount).map(|_| std::sync::mpsc::channel()).unzip();
    let bounds = to_dedup.last().unwrap().len();

    let mut handles = Vec::new();
    for (items, (dtm_rx, done)) in to_dedup.into_iter().zip(dtm_rxs.into_iter().zip(done_sx.into_iter())) {
        let local_tmp = tmp.clone();
        handles.push(thread::spawn(move || {
            for item in items {
                done.send(()).unwrap();
                let handle = dtm_rx.recv().unwrap();
                // the following `clone` operation on `item` is absolutely necessary, since
                // the `Fn` closure may be called numerous times in case of a retry,
                // requiring the cloning of the variable.
                det_atomically(handle, |trans| local_tmp.insert(trans, item.clone()));
            }
        }));
    }

    handles.push(thread::spawn(move || {
        let mut threadcount = threadcount;
        let mut i = 0;
        loop {
            // to avoid handing out too many tokens we must do this:
            if rest != 0 && i == bounds {
                // should the last round of items be unequal to the threadcount, the last round
                // will have fewer items
                threadcount = rest;
            }

            for idx in 0..threadcount {
                if let Err(_) = done_rx[idx].recv() {
                    return;
                }
            }

            let mut dtm = dtm();
            for idx in 0..threadcount {
                dtm_sx[idx].send(dtm.register()).unwrap();
            }
            freeze(dtm);

            i += 1;
        }
    }));

    handles.into_iter().for_each(|h| h.join().unwrap());

    // now unpack the Arc and get the values
    let hash_set = match Arc::try_unwrap(tmp) {
        Ok(content) => content,
        Err(_) => panic!("Unexpectedly failed to unpack arc"),
    };
    atomically(|trans| hash_set.as_vec(trans))
        .into_iter()
        .map(SequencerItem::from)
        .collect()
}

fn run_coordinator(
    lower_range: usize,
    upper_range: usize,
    in_chan: Vec<Receiver<()>>,
    out_chan: Vec<Sender<DTMHandle>>,
) {
    thread::spawn(move || {
        let threadcount = in_chan.len();
        // not all threads have equal amounts of work. The last thread will have less work items
        // sometimes, thus requiring this distinction between lower & upper range

        loop {
            for i in 0..upper_range {
                let num = if i < lower_range {
                    threadcount
                } else {
                    threadcount - 1
                };

                // wait for the signal
                for idx in 0..num {
                    // terminate the coordinator when the channel is disconnected
                    if let Err(_) = in_chan[idx].recv() {
                        return;
                    }
                }

                let mut dtm = dtm();
                for idx in 0..num {
                    out_chan[idx].send(dtm.register()).unwrap();
                }
                freeze(dtm);
            }
        }
    });
}

pub fn run_sequencer(
    unique_segments: Arc<VecDeque<SequencerItem>>,
    segment_length: usize,
    iteration_ranges: Vec<Range<usize>>,
) {
    let threadcount = iteration_ranges.len();
    let mut handles = Vec::with_capacity(threadcount);

    let lower = iteration_ranges.last().unwrap().len();
    let upper = iteration_ranges.first().unwrap().len();

    // TODO: construct the 2 vecs containing the senders/receivers, use them in the threads and
    // adjust the atomically call
    let (coord_in, thread_rx): (Vec<Sender<DTMHandle>>, Vec<Receiver<DTMHandle>>) =
        (0..threadcount).map(|_| std::sync::mpsc::channel()).unzip();
    let (thread_sx, coord_out): (Vec<Sender<()>>, Vec<Receiver<()>>) =
        (0..threadcount).map(|_| std::sync::mpsc::channel()).unzip();

    run_coordinator(lower, upper, coord_out, coord_in);
    let channels: Vec<_> = thread_rx.into_iter().zip(thread_sx.into_iter()).collect();

    // spawn threads to work in parallel
    for (iteration_range, (getter, done_chan)) in
        iteration_ranges.into_iter().zip(channels.into_iter())
    {
        //let iteration_range = rng.clone();
        let segments = unique_segments.clone();
        handles.push(thread::spawn(move || {
            // Step 2: go through the prefixes and suffixes of the genomes in descending size and stitch the genome back together
            for match_length in (1..segment_length).rev() {
                /*
                 * - loop through possible subsegment lengths
                 * - loop through all segments once (x)
                 * - loop through all segments again (y) and match against the starts and ends of x and y, stitch together on match
                 */

                for idx in iteration_range.clone() {
                    // communication with the DTM handle thread
                    done_chan.send(()).unwrap();
                    let handle = getter.recv().unwrap();

                    det_atomically(handle, |trans| {
                        let cur_seg = &segments[idx];
                        let mut cur_links = cur_seg.links.read(trans)?;

                        // only continue if the current segment is not linked already
                        if cur_links.prev.is_none() {
                            let slice = &cur_seg.segment[0..match_length];

                            // go over all items in Vec and test whether we can append our
                            // `cur_seg` to the item. If so, stop
                            'inner: for it in 0..segments.len() {
                                // skip the element itself -- this might be unnecessary but we want to avoid
                                // breaking the matching algorithm by linking an element to itself
                                if idx == it {
                                    continue;
                                }
                                // skip the current item when it already has an appended segment
                                if segments[it]
                                    .links
                                    .read_ref_atomic()
                                    .downcast::<LinkInfo>()
                                    .unwrap()
                                    .next
                                    .is_some()
                                {
                                    continue;
                                }

                                let cur_slice = &segments[it].segment
                                    [(segment_length - match_length)..segment_length];
                                if slice == cur_slice {
                                    // link both items together
                                    //segments[it].next.write(trans, Some(idx))?;
                                    segments[it].links.modify(trans, |mut l| {
                                        l.next = Some(idx);
                                        l
                                    })?;
                                    cur_links.prev = Some(it); //.write(trans, Some(it))?;
                                    cur_links.overlap_with_prev = match_length; //.write(trans, match_length)?;
                                    cur_seg.links.write(trans, cur_links)?;
                                    break 'inner;
                                }
                            }
                        }
                        Ok(())
                    });
                }
            }
        }));
    }

    // wait for threads to finish
    handles.into_iter().for_each(|h| h.join().unwrap());
}

pub fn reconstruct(unique_segments: Arc<VecDeque<SequencerItem>>) -> Vec<Nucleotide> {
    if cfg!(feature = "verify") {
        // TMP test
        println!("[TEST] checking segment links");
        atomically(|trans| {
            let mut forward_links = 0;
            let mut backward_links = 0;
            for item in unique_segments.iter() {
                if item.links.read(trans)?.next.is_none() {
                    forward_links += 1;
                }
                if item.links.read(trans)?.prev.is_none() {
                    backward_links += 1;
                }
            }
            assert_eq!(forward_links, 1);
            assert_eq!(backward_links, 1);
            Ok(())
        });
    }

    // Step 3 link together sequence
    atomically(|trans| {
        // find first element
        let mut cur = unique_segments
            .iter()
            .find(|seg| seg.links.read_atomic().prev.is_none())
            .unwrap();

        let mut reconstructed_sequence = Vec::new();

        loop {
            let link_info = cur.links.read(trans)?;

            reconstructed_sequence.extend_from_slice(&cur.segment[link_info.overlap_with_prev..]);

            if let Some(next_idx) = link_info.next {
                // move to the next value -> have to assign to another let binding first to drop `val` due to ownership issues
                cur = &unique_segments[next_idx];
            } else {
                break;
            }
        }

        Ok(reconstructed_sequence)
    })
}
