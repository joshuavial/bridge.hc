use hdi::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
pub struct Properties {
    pub progenitor_eth_address: String,
    pub progenitor_dht_address: AgentPubKey,
    pub percentage_for_consensus: u32,
}
impl Properties {
    pub fn new() -> ExternResult<Self> {
        Properties::try_from(dna_info()?.properties)
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Malformed properties".into()))
            })
    }
}
