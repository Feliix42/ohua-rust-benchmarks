use crate::Packet;
use crate::detector::*;
use crate::decoder::simple::*;
use std::collections::VecDeque;

/// Function that analyzes the incoming packet stream. The "benchmark" itself.
/// Everything inside this function is being timed.
///
/// Returns a Vec of flow IDs that contained an attack for later check
fn analyze_stream(mut packets: VecDeque<Packet>) -> Vec<usize> {
    let mut found_attacks = Vec::new();
    let mut decoder = Decoder::new();

    for packet in packets.drain(..) {
        // decode the data (state!) --> decoder.c
        if let Some(decoded_flow) = decoder.decode_flow(packet) {
            // process the output -> run the detector
            if detect(&decoded_flow.data) == DetectorResult::SignatureMatch {
                found_attacks.push(decoded_flow.flow_id);
            }
        }
    }

    assert!(decoder.fragments_map.is_empty());

    found_attacks
}
