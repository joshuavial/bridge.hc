use hdk::prelude::*;
use serde::de::DeserializeOwned;

pub fn call_transactions<I, R>(fn_name: String, payload: I) -> ExternResult<R>
where
    I: serde::Serialize + std::fmt::Debug,
    R: serde::Serialize + std::fmt::Debug + DeserializeOwned,
{
    unimplemented!()
    // let response = call(
    //     CallTargetCell::Local,
    //     "transactions".into(),
    //     fn_name.into(),
    //     None,
    //     payload,
    // )?;

    // let result = match response {
    //     ZomeCallResponse::Ok(result) => Ok(result),
    //     _ => Err(wasm_error!(format!(
    //         "Error creating the transaction: {:?}",
    //         response
    //     ))),
    // }?;

    // let transaction_hash: R = result.decode().map_err(|_| {
    //     wasm_error!(WasmErrorInner::Guest(
    //         "Error decoding transaction hash".into()
    //     ))
    // })?;

    // Ok(transaction_hash)
}
