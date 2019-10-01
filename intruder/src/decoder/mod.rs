#[cfg(not(feature = "transactional"))]
pub mod simple;

#[cfg(feature = "transactional")]
pub mod stm_decoder;

#[derive(PartialEq, Debug)]
pub struct DecodedPacket {
    pub flow_id: usize,
    pub data: String,
}
