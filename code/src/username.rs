use crate::profile::{strings::USERNAME_LINK_TYPE, Username};
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

/** Temporary Guillem solution **/

pub fn set_username(username: String) -> ZomeApiResult<()> {
    let username = Username::new(username);

    let username_anchor = holochain_anchors::anchor("username".into(), "".into())?;

    let username_entry = hdk::commit_entry(&username.entry())?;

    hdk::link_entries(
        &AGENT_ADDRESS.clone(),
        &username_entry,
        "agent->username",
        "",
    )?;
    hdk::link_entries(&username_anchor, &username_entry, USERNAME_LINK_TYPE, "")?;

    Ok(())
}