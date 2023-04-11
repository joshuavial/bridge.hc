use hdi::prelude::{*, holo_hash::ActionHashB64};
use bridge_hc_types::{Transaction, TransactionRequest};

mod transaction;
mod validation;

#[derive(Serialize, Deserialize)]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Transaction(Transaction),
    TransactionRequest(TransactionRequest),
}

#[hdk_link_types]
pub enum LinkTypes {
    AgentPubKeyToTransactionRequest
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    TransactionRequestReceived {
        transaction_request_hash: ActionHashB64,
        transaction_request: TransactionRequest,
    },
    TransactionRequestAccepted {
        transaction_request_hash: ActionHashB64,
        transaction: (ActionHashB64, Transaction),
    },
    TransactionRequestCancelled {
        transaction_request_hash: ActionHashB64,
    },
    TransactionRequestRejected {
        transaction_request_hash: ActionHashB64,
    },
}
