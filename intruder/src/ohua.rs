use crate::*;
use crate::detector::*;
use crate::decoder::*;
use crate::decoder::simple::*;
use std::collections::VecDeque;

/// A 2-stage pipeline parallel algorithm.
pub fn analyze_flow_2(mut packets: VecDeque<Packet>) -> Vec<usize> {
    let mut decoder: Decoder = Decoder::new();
    let mut detector: AttackDetector = AttackDetector::new();

    let packets0: VecDeque<Packet> = id(packets);
    for packet0 in packets0 {
        let packet:Packet = packet0;
        // decode the data (state!) --> decoder.c
        let decoded_flow:Option<DecodedFlow> = decoder.decode_flow(packet);
        // process the output -> run the detector
        detector.detect(decoded_flow);
    }

    detector.get_attacks()
}

/// A 3-stage pipeline parallel algorithm where the heavy compute step
/// benefits from data parallelism.
pub fn analyze_flow_3(mut packets: VecDeque<Packet>) -> Vec<usize> {
    let mut decoder: Decoder = Decoder::new();
    let mut found: Vec<Option<usize>> = Vec::new();

    //let packets0: VecDeque<Packet> = id(packets);
    for packet0 in packets {
        let packet:Packet = packet0;
        // decode the data (state!) --> decoder.c
        let decoded_flow:Option<DecodedFlow> = decoder.decode_flow(packet);
        // 2nd stage: intrusion detection (is stateless and runs data parallel)
        let detected: Option<usize> = bind_detect(decoded_flow); 
        // just collect
        found.push(detected)
    }

     get_attacks(found)
}
