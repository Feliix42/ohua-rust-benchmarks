use crate::Packet;
use std::collections::HashMap;

pub struct DecodedPacket {
    pub flow_id: usize,
    pub data: String,
}

pub struct DecoderState {
    pub fragments_map: HashMap<usize, Vec<Packet>>,
}

impl DecoderState {
    pub fn new() -> Self {
        Self {
            fragments_map: HashMap::new(),
        }
    }
}

/// Decodes a packet. If the packet is part of a fragmented flow and the flow is
/// not completely decoded yet, the function will return `None` and place the
/// packet in its internal storage
pub fn decode_packet(packet: Packet, state: &mut DecoderState) -> Option<DecodedPacket> {
    if packet.packets_in_flow != 1 {
        // this is part of a fragmented flow
        let decoded = state
            .fragments_map
            .entry(packet.flow_id)
            .or_insert(Vec::new());

        // insert the current element into the queue
        let idx = decoded
            .iter()
            .position(|p| p.fragment_id > packet.fragment_id)
            .unwrap_or(0);
        decoded.insert(idx, packet);

        // reassemble the flow if all fragments are present
        if decoded.len() == decoded[0].packets_in_flow {
            let flow_id = decoded[0].flow_id;
            let reconstructed_data = decoded
                .drain(..)
                .fold(String::new(), |acc, p| acc + &p.data);

            // TODO: Remove assertion?
            assert!(state.fragments_map.remove(&flow_id).is_some());
            Some(DecodedPacket {
                flow_id,
                data: reconstructed_data,
            })
        } else {
            None
        }
    } else {
        // this is the only packet
        assert!(packet.fragment_id == 0);

        Some(DecodedPacket {
            flow_id: packet.flow_id,
            data: packet.data,
        })
    }
}
