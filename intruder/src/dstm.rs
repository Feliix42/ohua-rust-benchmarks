use crate::decoder::stm_decoder::{decode_packet, StmDecoderState};
use crate::detector::{detect, DetectorResult};
use crate::*;
use std::thread::{self, JoinHandle};
use ::stm::{atomically, det_atomically, dtm, freeze, DTMHandle};

// So, the above(????) version does not work because the implemented algorithm
// for deterministic STM proceeds in rounds. That is, it needs to finish
// a round first in order to re-execute the retries.
// Here is the deadlock:
// Tx1 on thread 1 gets a retry which means it will try to redo its computation in the next round.
// In order to do so it waits for the first round to be done.
// But this in turn blocks Tx2 which would normally run after Tx1.
// We are left with putting Tx1 and Tx2 into a single transaction.
// Otherwise, I would not know how pause Tx1 and continue with Tx2 *on the same* thread. This would
// require quite a different programming model because for that the library. Would need to schedule
// the transactions across a pool of threads.

fn analyze_stream(
    packet: Packet,
    decoder_state: StmDecoderState,
    handle: DTMHandle,
) -> Option<usize> {
    let decoder_result = det_atomically(handle, |trans| {
        decode_packet(&packet, &decoder_state, trans)
        //let mut ds = Vec::new();
        //for p in packets {
        //let decoder_result = decode_packet(&p, &decoder_state, trans);
        //match decoder_result {
        //Ok(d) => ds.push(d),
        //Err(e) => return Err(e) // error type cast
        //}
        //}
        //Ok(ds)
    });

    //for decoder_result in decoder_results {
    if let Some(decoded_flow) = decoder_result {
        // process the output -> run the detector
        if detect(&decoded_flow.data) == DetectorResult::SignatureMatch {
            //found_attacks.push(decoded_flow.flow_id);
            return Some(decoded_flow.flow_id);
        }
    }

    None
    //}

    //found_attacks
}

pub fn run_eval(packets: Vec<Packet>, threadcount: usize) -> Vec<usize> {
    let mut found_attacks = Vec::new();
    let decoder_state = StmDecoderState::new(threadcount);

    // TODO(feliix42): This could be improved by processing multiple elements in a single Tx. But
    // that would already be an optimization.
    for chunk in packets.chunks(threadcount) {
        // create DTM handles
        let mut dtm = dtm();
        let work: Vec<(Packet, DTMHandle)> = chunk
            .into_iter()
            .map(std::borrow::ToOwned::to_owned)
            .map(|item| (item, dtm.register()))
            .collect();
        freeze(dtm);

        // spawn threads
        let mut threads = Vec::with_capacity(threadcount);
        for item in work {
            let ds = decoder_state.clone();
            let (packet, handle) = item;
            threads.push(thread::spawn(move || analyze_stream(packet, ds, handle)));
        }

        // collect work
        found_attacks.extend(
            threads
                .into_iter()
                .map(JoinHandle::join)
                .map(Result::unwrap)
                .filter(Option::is_some)
                .map(Option::unwrap),
        );
    }

    // State verification
    assert!(atomically(|trans3| decoder_state
        .fragments_map
        .is_empty(trans3)));

    found_attacks
}
