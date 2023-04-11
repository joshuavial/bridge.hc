use hdk::prelude::*;
use crate::holo_hash::{AgentPubKeyB64, AnyDhtHashB64};
use serde::de::DeserializeOwned;

use bridge_hc_types::{Transaction, TransactionRequest};

pub fn call_transactions<I, R>(fn_name: String, payload: I) -> ExternResult<R>
where
    I: serde::Serialize + std::fmt::Debug,
    R: serde::Serialize + std::fmt::Debug + DeserializeOwned,
{
    let response = call(
        CallTargetCell::Local,
        ZomeName::from("transactions".to_string()),
        fn_name.into(),
        None,
        payload,
    )?;

    let result = match response {
        ZomeCallResponse::Ok(result) => Ok(result),
        _ => Err(wasm_error!(format!(
            "Error creating the transaction: {:?}",
            response
        ))),
    }?;

    let transaction_hash: R = result.decode().map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Error decoding transaction hash".into()
        ))
    })?;

    Ok(transaction_hash)
}


pub fn build_transaction(transaction_request_record: Record) -> ExternResult<Transaction> {
    let transaction_request: TransactionRequest = transaction_request_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(String::from("Malformed transaction_request",)))?;

    let spender = transaction_request.spender_pub_key.clone();
    let recipient = transaction_request.recipient_pub_key.clone();

    let spender_latest_transaction = get_latest_transaction_for_agent(spender.into())?;
    let recipient_latest_transaction = get_latest_transaction_for_agent(recipient.into())?;

    let info = SerializedBytes::try_from(transaction_request_record.action_address())
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Error getting transaction request record info".to_string())))?;

    let transaction = Transaction::from_previous_transactions(
        transaction_request.spender_pub_key.into(),
        transaction_request.recipient_pub_key.into(),
        spender_latest_transaction,
        recipient_latest_transaction,
        transaction_request.amount,
        info,
    )
    .map_err(|_| wasm_error!(WasmErrorInner::Guest("Malformed transaction".into())))?;
    Ok(transaction)
}

fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<Option<(AnyDhtHashB64, Transaction)>> {
    call_transactions("get_latest_transaction_for_agent".into(), agent_pub_key)
}
