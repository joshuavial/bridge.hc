use hdk::prelude::*;

use transaction_requests_integrity::*;

mod handlers;
mod utils;

pub use handlers::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // let mut functions = GrantedFunctions::new();
    // functions.insert((zome_info()?.name, "recv_remote_signal".into()));

    // let grant = ZomeCallCapGrant {
    //     access: CapAccess::Unrestricted,
    //     functions,
    //     tag: "".into(),
    // };
    // create_cap_grant(grant)?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}
