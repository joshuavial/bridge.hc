use hdi::prelude::*;
use serde::{Deserialize, Serialize};
use web3::types::Address;

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
pub struct Properties {
    pub progenitor_eth_address: Address,
    pub percentage_for_consensus: u32,
}
