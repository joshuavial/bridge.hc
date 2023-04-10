use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{TransactionRequestType, TransactionRequest};

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    TransactionRequest(TransactionRequest),
}

#[hdk_link_types]
pub enum LinkTypes {
    AgentPubKeyToTransactionRequest
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequestInput {
    pub transaction_request_type: TransactionRequestType,
    pub counterparty_pub_key: AgentPubKeyB64,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
pub struct UIEnum(pub String);