use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use crate::types::UIEnum;
use std::fmt;

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