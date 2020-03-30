use super::DecodedPacket;
use crate::Packet;
use stm::{StmResult, Transaction};
use stm_datastructures::THashMap;

#[derive(Clone)]
pub struct StmDecoderState {
    pub fragments_map: THashMap<usize, Vec<Packet>>,
}

impl StmDecoderState {
    pub fn new(bucket_no: usize) -> Self {
        Self {
            fragments_map: THashMap::new(bucket_no),
        }
    }
}

/// Decodes a packet. If the packet is part of a fragmented flow and the flow is
/// not completely decoded yet, the function will return `None` and place the
/// packet in its internal storage
pub fn decode_packet(
    packet: &Packet,
    state: &StmDecoderState,
    transaction: &mut Transaction,
) -> StmResult<Option<DecodedPacket>> {
    if packet.packets_in_flow != 1 {
        // get the matching TVar (== bucket) and read it
        let bucket = state.fragments_map.get_bucket(&packet.flow_id);
        let mut frags = bucket.read(transaction)?;
        // this is part of a fragmented flow
        let decoded = frags.entry(packet.flow_id).or_insert(Vec::new());

        // insert the current element into the queue
        let idx = decoded
            .iter()
            .position(|p| packet.fragment_id < p.fragment_id)
            .unwrap_or(decoded.len());
        decoded.insert(idx, packet.clone());

        // reassemble the flow if all fragments are present
        if decoded.len() == decoded[0].packets_in_flow {
            let flow_id = decoded[0].flow_id;
            let reconstructed_data = decoded
                .drain(..)
                .fold(String::new(), |acc, p| acc + &p.data);

            // remove the flow from the hashmap & write that back
            assert!(frags.remove(&flow_id).is_some());
            bucket.write(transaction, frags)?;

            Ok(Some(DecodedPacket {
                flow_id,
                data: reconstructed_data,
            }))
        } else {
            bucket.write(transaction, frags)?;
            Ok(None)
        }
    } else {
        // this is the only packet
        assert!(packet.fragment_id == 0);

        Ok(Some(DecodedPacket {
            flow_id: packet.flow_id,
            data: packet.data.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Packet;
    use stm::atomically;

    #[test]
    fn basic_decoding() {
        let inp1 = Packet {
            flow_id: 42,
            fragment_id: 1,
            packets_in_flow: 3,
            length: 1,
            data: "w".into(),
        };
        let inp2 = Packet {
            flow_id: 42,
            fragment_id: 0,
            packets_in_flow: 3,
            length: 1,
            data: "t".into(),
        };
        let inp3 = Packet {
            flow_id: 42,
            fragment_id: 2,
            packets_in_flow: 3,
            length: 1,
            data: "o".into(),
        };

        let dec = StmDecoderState::new();

        assert_eq!(atomically(|trans| decode_packet(&inp1, &dec, trans)), None);
        assert_eq!(atomically(|trans| decode_packet(&inp2, &dec, trans)), None);
        assert_eq!(
            atomically(|trans| decode_packet(&inp3, &dec, trans)),
            Some(DecodedPacket {
                flow_id: 42,
                data: "two".into()
            })
        );
    }
}
