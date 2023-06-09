use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::TransactionRequestType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequestInput {
    pub transaction_request_type: String, // work out how to serialize enums
    pub counterparty_pub_key: AgentPubKeyB64,
    pub amount: f64,
}
