use crate::decoder::stm_decoder::{decode_packet, StmDecoderState};
use crate::detector::{detect, DetectorResult};
use crate::*;
use std::collections::VecDeque;
use std::thread;
use stm::atomically; //, TVar};

/// Function that analyzes the incoming packet stream. The "benchmark" itself.
/// Everything inside this function is being timed.
///
/// Returns a Vec of flow IDs that contained an attack for later check
fn analyze_stream(mut packets: VecDeque<Packet>, decoder_state: StmDecoderState) -> Vec<usize> {
    let mut found_attacks = Vec::new();

    // NOTE: This is a deviation from the original code where packets where
    // retrieved individually. Unfortunately, this made up for 96% of all
    // processing time, hence the simplification
    // loop {
    //     let packet = atomically(|trans| {
    //         let mut v = packets.read(trans)?;
    //         if v.len() == 0 {
    //             Ok(None)
    //         } else {
    //             let r = v.pop_front().unwrap();
    //             packets.write(trans, v)?;
    //             Ok(Some(r))
    //         }
    //     });

    for p in packets {
        // if let Some(p) = packet {
        // do the algorithm
        let decoder_result = atomically(|trans2| decode_packet(&p, &decoder_state, trans2));
        if let Some(decoded_flow) = decoder_result {
            // process the output -> run the detector
            if detect(&decoded_flow.data) == DetectorResult::SignatureMatch {
                found_attacks.push(decoded_flow.flow_id);
            }
        }
        // } else {
        //     break;
        // }
    }

    found_attacks
}

fn partition_input_vec(mut packets: VecDeque<Packet>, threadcount: usize) -> Vec<VecDeque<Packet>> {
    let l = packets.len() / threadcount;
    let mut rest = packets.len() % threadcount;

    let mut partitioned = vec![VecDeque::with_capacity(l); threadcount];

    for t_num in 0..threadcount {
        if rest > 0 {
            partitioned[t_num] = packets.split_off(packets.len() - l - 1);
            rest -= 1;
        } else {
            if packets.len() <= l {
                partitioned[t_num] = packets.split_off(0);
            } else {
                partitioned[t_num] = packets.split_off(packets.len() - l);
            }
        }
    }

    partitioned
}

fn run_eval(packets: VecDeque<Packet>, threadcount: usize) -> Vec<usize> {
    let mut found_attacks = Vec::new();
    let decoder_state = StmDecoderState::new(threadcount);

    let mut handles = Vec::with_capacity(threadcount);
    let mut inputs = partition_input_vec(packets, threadcount);

    for packets in inputs {
        let ds = decoder_state.clone();
        handles.push(thread::spawn(move || analyze_stream(packets, ds)));
    }

    for handle in handles {
        let mut attacks = handle.join().unwrap();
        found_attacks.append(&mut attacks);
    }

    // State verification
    let fmap = decoder_state.fragments_map.get_contents();
    if !fmap.is_empty() {
        println!("{:#?}", fmap);
    }
    assert!(atomically(|trans3| decoder_state
        .fragments_map
        .is_empty(trans3)));

    found_attacks
}
