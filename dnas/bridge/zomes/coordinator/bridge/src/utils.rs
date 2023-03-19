use hdk::prelude::*;
use bridge_integrity::*;

//TODO remove - practice writing modules
pub fn get_progenitor_pub_key() -> ExternResult<AgentPubKey> {
    let properties = Properties::try_from(dna_info()?.properties)
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Malformed properties".into())))?;
    Ok(properties.progenitor_dht_address)
}
