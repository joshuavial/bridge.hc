use hdk::prelude::*;
use crate::holo_hash::{AgentPubKeyB64, AnyDhtHashB64};
use serde::de::DeserializeOwned;

use types::Transaction;

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


pub fn build_transaction(transaction_request_element: Record) -> ExternResult<Transaction> {
    // let transaction_request: TransactionRequest = transaction_request_element
    //     .entry()
    //     .to_app_option()?
    //     .ok_or(wasm_error!(String::from("Malformed transaction_request",)))?;

    // let spender = transaction_request.spender_pub_key.clone();
    // let recipient = transaction_request.recipient_pub_key.clone();

    // let spender_latest_transaction = get_latest_transaction_for_agent(spender.into())?;
    // let recipient_latest_transaction = get_latest_transaction_for_agent(recipient.into())?;

    unimplemented!()
    // let transaction = Transaction::from_previous_transactions(
    //     transaction_request.spender_pub_key.into(),
    //     transaction_request.recipient_pub_key.into(),
    //     spender_latest_transaction,
    //     recipient_latest_transaction,
    //     transaction_request.amount,
    //     SerializedBytes::try_from(transaction_request_element.header_address())?,
    // )
    // .map_err(|_| wasm_error!(WasmErrorInner::Guest("Malformed transaction".into())))?;
    // Ok(transaction)
}

fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<Option<(AnyDhtHashB64, Transaction)>> {
    call_transactions("get_latest_transaction_for_agent".into(), agent_pub_key)
}
