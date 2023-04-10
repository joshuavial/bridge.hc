use hdk::prelude::{*, holo_hash::*};
use std::fmt;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct AuthorityList {
    pub percentage_for_consensus: u32,
    pub authorities: Vec<(AgentPubKey, String)>,
}

#[hdk_entry_helper]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub spender_pub_key: AgentPubKeyB64,
    pub recipient_pub_key: AgentPubKeyB64,
    pub amount: f64,
}

impl TransactionRequest {
    pub fn get_counterparty(&self) -> ExternResult<AgentPubKeyB64> {
        let my_pub_key: AgentPubKeyB64 = agent_info()?.agent_initial_pubkey.into();

        if my_pub_key.eq(&self.spender_pub_key) {
            Ok(self.recipient_pub_key.clone())
        } else if my_pub_key.eq(&self.recipient_pub_key) {
            Ok(self.spender_pub_key.clone())
        } else {
            Err(wasm_error!(String::from(
                "I don't participate in this TransactionRequest",
            )))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "UIEnum")]
#[serde(into = "UIEnum")]
pub enum TransactionRequestType {
    Send,
    Receive,
}
impl From<UIEnum> for TransactionRequestType {
    fn from(ui_enum: UIEnum) -> Self {
        match ui_enum.0.as_str() {
            "Send" => Self::Send,
            "Receive" => Self::Receive,
            _ => Self::Send,
        }
    }
}
impl From<TransactionRequestType> for UIEnum {
    fn from(request_type: TransactionRequestType) -> Self {
        Self(request_type.to_string())
    }
}
impl fmt::Display for TransactionRequestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub spender: TransactionParty,
    pub recipient: TransactionParty,
    pub amount: f64,
    pub info: SerializedBytes,
}

impl Transaction {
    pub fn try_from_entry(entry: Entry) -> ExternResult<Transaction> {
        match entry {
            Entry::CounterSign(_session_data, entry_bytes) => {
                let transaction = Transaction::try_from(entry_bytes.into_sb())
                    .map_err(|e| wasm_error!(format! {"{:?}", e}))?;
                Ok(transaction)
            }
            _ => Err(wasm_error!(String::from("Malformed entry"))),
        }
    }

    pub fn from_previous_transactions(
        _spender: AgentPubKey,
        _recipient: AgentPubKey,
        _previous_spender_transaction: Option<(ActionHashB64, Transaction)>,
        _previous_recipient_transaction: Option<(ActionHashB64, Transaction)>,
        _amount: f64,
        _info: SerializedBytes,
    ) -> ExternResult<Transaction> {
        unimplemented!()
        // let previous_spender_balance = balance_from_previous_transaction(
        //     spender.clone(),
        //     previous_spender_transaction.clone().map(|(_, t)| t),
        // )?;
        // let previous_recipient_balance = balance_from_previous_transaction(
        //     recipient.clone(),
        //     previous_recipient_transaction.clone().map(|(_, t)| t),
        // )?;

        // let resulting_spender_balance = previous_spender_balance - amount;
        // let resulting_recipient_balance = previous_recipient_balance + amount;

        // let spender = TransactionParty {
        //     agent_pub_key: spender.into(),
        //     previous_transaction_hash: previous_spender_transaction.map(|(h, _)| h),
        //     resulting_balance: resulting_spender_balance,
        // };
        // let recipient = TransactionParty {
        //     agent_pub_key: recipient.into(),
        //     previous_transaction_hash: previous_recipient_transaction.map(|(h, _)| h),
        //     resulting_balance: resulting_recipient_balance,
        // };

        // let transaction = Transaction {
        //     spender,
        //     recipient,
        //     amount,
        //     info,
        // };

        // Ok(transaction)
    }

    pub fn get_party(&self, agent_pub_key: &AgentPubKey) -> ExternResult<TransactionParty> {
        if AgentPubKey::from(self.spender.agent_pub_key.clone()).eq(agent_pub_key) {
            Ok(self.spender.clone())
        } else if AgentPubKey::from(self.recipient.agent_pub_key.clone()).eq(agent_pub_key) {
            Ok(self.recipient.clone())
        } else {
            Err(wasm_error!(String::from(
                "This agent did not participate in the transaction",
            )))
        }
    }

    pub fn get_counterparty(&self) -> ExternResult<TransactionParty> {
        let my_pub_key: AgentPubKeyB64 = agent_info()?.agent_initial_pubkey.into();

        if my_pub_key.eq(&self.spender.agent_pub_key) {
            Ok(self.spender.clone())
        } else if my_pub_key.eq(&self.recipient.agent_pub_key) {
            Ok(self.spender.clone())
        } else {
            Err(wasm_error!(String::from(
                "I don't participate in this Transaction",
            )))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionParty {
    pub agent_pub_key: AgentPubKeyB64,
    pub previous_transaction_hash: Option<ActionHashB64>,
    pub resulting_balance: f64,
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
