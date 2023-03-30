use std::collections::BTreeMap;

use bridge_integrity::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use transaction_requests_integrity::{call_transactions, CreateTransactionRequestInput};

use crate::{utils::build_transaction, TransactionRequest, TransactionRequestType};

#[hdk_extern]
pub fn create_transaction_request(
    input: CreateTransactionRequestInput,
) -> ExternResult<(ActionHashB64, TransactionRequest)> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    if AgentPubKey::from(input.counterparty_pub_key.clone()).eq(&my_pub_key) {
        return Err(wasm_error!(String::from(
            "An agent cannot create an offer to themselves",
        )));
    }

    let transaction_request = match input.transaction_request_type {
        TransactionRequestType::Send => TransactionRequest {
            spender_pub_key: AgentPubKeyB64::from(my_pub_key.clone()),
            recipient_pub_key: input.counterparty_pub_key.clone(),
            amount: input.amount,
        },
        TransactionRequestType::Receive => TransactionRequest {
            spender_pub_key: input.counterparty_pub_key.clone(),
            recipient_pub_key: AgentPubKeyB64::from(my_pub_key.clone()),
            amount: input.amount,
        },
    };

    // let header_hash = create_entry(&transaction_request)?;

    // create_link(
    //     EntryHash::from(my_pub_key),
    //     header_hash.clone().retype(hash_type::Entry),
    //     HdkLinkType::Any,
    //     (),
    // )?;
    // create_link(
    //     EntryHash::from(AgentPubKey::from(transaction_request.get_counterparty()?)),
    //     header_hash.clone().retype(hash_type::Entry),
    //     HdkLinkType::Any,
    //     (),
    // )?;

    // Ok((header_hash.into(), transaction_request))
    unimplemented!()
}

#[hdk_extern]
pub fn cancel_transaction_request(_transaction_request_hash: ActionHashB64) -> ExternResult<()> {
    Ok(())
}

#[hdk_extern]
pub fn reject_transaction_request(_transaction_request_hash: ActionHashB64) -> ExternResult<()> {
    Ok(())
}

#[hdk_extern]
pub fn clear_transaction_request(_transaction_request_hash: ActionHashB64) -> ExternResult<()> {
    Ok(())
}

#[hdk_extern]
pub fn accept_transaction_request(
    transaction_request_hash: ActionHashB64,
) -> ExternResult<(ActionHashB64, Transaction)> {
    // let transaction_request_element = get(
    //     ActionHash::from(transaction_request_hash.clone()),
    //     GetOptions::default(),
    // )?
    // .ok_or(wasm_error!(String::from(
    //     "Couldn't get transaction request",
    // )))?;

    // let transaction_request: TransactionRequest = transaction_request_element
    //     .entry()
    //     .to_app_option()?
    //     .ok_or(wasm_error!(String::from(
    //         "Malformed transaction request",
    //     )))?;
    // let counterparty = transaction_request.get_counterparty()?;

    // let counterparty_chain_top = get_chain_top(counterparty.into())?;

    // let transaction = build_transaction(transaction_request_element)?;

    // let result: (ActionHashB64, Transaction) = call_transactions(
    //     "attempt_create_transaction".into(),
    //     AttemptCreateTransactionInput {
    //         transaction,
    //         counterparty_chain_top: counterparty_chain_top.into(),
    //     },
    // )?;

    // Ok(result)
    unimplemented!()
}

// #[hdk_extern(infallible)]
// fn post_commit(headers: Vec<SignedHeaderHashed>) {
//     let my_transactions = query_my_transactions().unwrap();

//     let my_new_transactions: Vec<SignedHeaderHashed> = headers
//         .into_iter()
//         .filter(|h| my_transactions.contains_key(&ActionHashB64::from(h.header_address().clone())))
//         .collect();

//     if my_new_transactions.len() > 0 {
//         let get_inputs = my_new_transactions
//             .into_iter()
//             .map(|h| GetInput::new(h.header_address().clone().into(), Default::default()))
//             .collect();

//         let elements = HDK.with(|hdk| hdk.borrow().get(get_inputs)).unwrap();

//         let transactions_i_created: Vec<_> = elements
//             .into_iter()
//             .filter_map(|el| el)
//             .filter_map(|el| el.entry().as_option().map(|e| e.clone()))
//             .filter(|entry| match entry {
//                 Entry::CounterSign(session_data, _entry_bytes) => {
//                     let state = session_data
//                         .agent_state_for_agent(&agent_info().unwrap().agent_initial_pubkey)
//                         .unwrap();
//                     state.agent_index().to_owned() == 0
//                 }
//                 _ => false,
//             })
//             .collect();

//         if transactions_i_created.len() > 0 {
//             let result = call_remote(
//                 agent_info().unwrap().agent_initial_pubkey,
//                 zome_info().unwrap().name,
//                 "clean_transaction_requests".into(),
//                 None,
//                 (),
//             );

//             match result.clone() {
//                 Ok(ZomeCallResponse::Ok(_)) => {}
//                 _ => error!(
//                     "Error trying to clean the transaction requests {:?}",
//                     result,
//                 ),
//             };
//         }
//     }
// }

// #[hdk_extern]
// pub fn clean_transaction_requests(_: ()) -> ExternResult<()> {
//     let my_pub_key = agent_info()?.agent_initial_pubkey;
//     let links = get_links(my_pub_key.into(), None)?;

//     let my_transactions = query_my_transactions()?;

//     for (transaction_hash, transaction) in my_transactions {
//         let info = transaction.info.clone();

//         let transaction_request_hash = HeaderHash::try_from(info)?;

//         if let Some(link) = links.iter().find(|link| {
//             link.target
//                 .clone()
//                 .retype(hash_type::Header)
//                 .eq(&transaction_request_hash)
//         }) {
//             delete_link(link.create_link_hash.clone())?;

//             create_link(
//                 transaction_request_hash.clone().retype(hash_type::Entry),
//                 ActionHash::from(transaction_hash).retype(hash_type::Entry),
//                 HdkLinkType::Any,
//                 (),
//             )?;
//         }
//     }

//     Ok(())
// }

fn query_my_transactions() -> ExternResult<BTreeMap<ActionHashB64, Transaction>> {
    unimplemented!()
    // let response = call_remote(
    //     agent_info().unwrap().agent_initial_pubkey,
    //     "transactions".into(),
    //     "query_my_transactions".into(),
    //     None,
    //     (),
    // )?;

    // let result = match response {
    //     ZomeCallResponse::Ok(result) => Ok(result),
    //     _ => Err(wasm_error!(format!(
    //         "Error querying my transactions: {:?}",
    //         response
    //     ))),
    // }?;

    // let my_transactions: BTreeMap<ActionHashB64, Transaction> = result
    //     .decode()
    //     .map_err(|_| wasm_error!(WasmErrorInner::Guest("Error decoding transactions".into())))?;

    // Ok(my_transactions)
}

// #[hdk_extern]
// pub fn get_my_transaction_requests(
//     _: (),
// ) -> ExternResult<BTreeMap<ActionHashB64, TransactionRequest>> {
//     let my_pub_key = agent_info()?.agent_initial_pubkey;
//     let links = get_links(my_pub_key.into(), None)?;

//     let get_inputs = links
//         .into_iter()
//         .map(|link| {
//             GetInput::new(
//                 link.target.retype(hash_type::Header).into(),
//                 GetOptions::default(),
//             )
//         })
//         .collect();

//     let elements = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

//     let transaction_requests = elements
//         .into_iter()
//         .filter_map(|el| el)
//         .map(|el| {
//             let header_hash = ActionHashB64::from(el.header_address().clone());

//             let transaction_request: TransactionRequest =
//                 el.entry()
//                     .to_app_option()?
//                     .ok_or(wasm_error!(String::from(
//                         "Malformed transaction request",
//                     )))?;

//             Ok((header_hash, transaction_request))
//         })
//         .collect::<ExternResult<BTreeMap<ActionHashB64, TransactionRequest>>>()?;

//     Ok(transaction_requests)
// }

fn get_chain_top(agent_pub_key: AgentPubKey) -> ExternResult<ActionHash> {
    let activity = get_agent_activity(
        agent_pub_key,
        ChainQueryFilter::new(),
        ActivityRequest::Full,
    )?;

    let highest_observed = activity.highest_observed.ok_or(wasm_error!(String::from(
        "Counterparty highest observed was empty",
    )))?;

    if highest_observed.hash.len() != 1 {
        return Err(wasm_error!(String::from(
            "Counterparty highest observed was more than one",
        )));
    }

    Ok(highest_observed.hash[0].clone())
}
