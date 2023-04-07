use bridge_integrity::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::TransactionRequest;

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
