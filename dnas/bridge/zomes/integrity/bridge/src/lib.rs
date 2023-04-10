pub mod authority_list;
pub use authority_list::*;

pub mod properties;
pub use properties::*;

pub mod transaction;
pub use transaction::*;

mod validation;

use types::*;
use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    AuthorityList(AuthorityList),
    Transaction(Transaction),
    TransactionRequest(TransactionRequest),
}