use hdk::prelude::{*, holo_hash::ActionHashB64};

use bridge_hc_types::*;
use crate::utils::records_to_transactions;

use std::collections::BTreeMap;

#[hdk_extern]
pub fn query_my_transactions(_: ()) -> ExternResult<BTreeMap<ActionHashB64, Transaction>> {
    let filter = ChainQueryFilter::new()
        .entry_type(transaction_entry_type()?)
        .include_entries(true);
    let records = query(filter)?;

    records_to_transactions(records)
}

pub(crate) fn transaction_entry_type() -> ExternResult<EntryType> {
    let app_entry = AppEntryDef::new(1.into(), 0.into(), EntryVisibility::Public);
    Ok(EntryType::App(app_entry))
}
