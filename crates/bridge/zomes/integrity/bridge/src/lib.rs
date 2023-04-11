pub mod authority_list;
pub use authority_list::*;

pub mod properties;
pub use properties::*;

mod validation;

use bridge_hc_types::{ AuthorityList };
use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    AuthorityList(AuthorityList)
}

#[hdk_link_types]
pub enum LinkTypes {
    AgentPubKeyToAuthorityList
}