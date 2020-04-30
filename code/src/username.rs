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

pub fn get_all_agents() -> ZomeApiResult<Vec<Username>> {
    let username_anchor = holochain_anchors::anchor("username".into(), "".into())?;

    hdk::utils::get_links_and_load_type(
        &username_anchor,
        LinkMatch::Exactly(USERNAME_LINK_TYPE),
        LinkMatch::Any,
    )
}

pub fn get_username(agent_address: Address) -> ZomeApiResult<Option<String>> {
    let links_result = hdk::get_links(
        &agent_address,
        LinkMatch::Exactly("agent->username"),
        LinkMatch::Any,
    )?;

    match links_result.links().len() {
        0 => Ok(None),
        1 => {
            let username_address = links_result.addresses()[0].clone();

            let username: Username = hdk::utils::get_as_type(username_address)?;

            Ok(Some(username.username))
        }
        _ => Err(ZomeApiError::from(String::from(
            "Agent has more than one username registered",
        ))),
    }
}
