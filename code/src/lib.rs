#![feature(proc_macro_hygiene)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::profile::{BooleanReturn, HashedEmail, Profile};
use hdk::{
    entry_definition::ValidatingEntryType, error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address,
};
use hdk_proc_macros::zome;
use serde_derive::{Deserialize, Serialize};
pub mod profile;
pub mod username;

// MAIN FILE FOR THE PROFILE ZOME
// contains calls to entry definitions and functions

// Crate              Modules
// profile __________ mod
//            |______ handlers
//            |______ validation
//            |______ strings

#[zome]
mod profile_zome {

    use crate::{profile::Username, username as username_mod};

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        // this is where you can actually have some validations for agents who want to join this network.
        // Since this is a public DHT wehere anyone can join, we might not have much of validation here. Let's see.
        Ok(())
    }

    // ENTRY DEFINITIONS
    #[entry_def]
    fn profile_def() -> ValidatingEntryType {
        profile::profile_definition()
    }

    #[entry_def]
    fn hashed_email_def() -> ValidatingEntryType {
        profile::hashed_email_definition()
    }

    #[entry_def]
    fn username_def() -> ValidatingEntryType {
        profile::username_definition()
    }

    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    // FRONTEND FUNCTIONS
    #[zome_fn("hc_public")]
    fn is_email_registered(email: String) -> ZomeApiResult<BooleanReturn> {
        let result = profile::handlers::check_email(email)?;
        Ok(BooleanReturn { value: result })
    }

    #[zome_fn("hc_public")]
    fn is_username_registered(username: String) -> ZomeApiResult<BooleanReturn> {
        let result = profile::handlers::check_username(username)?;
        Ok(BooleanReturn { value: result })
    }

    #[zome_fn("hc_public")]
    fn create_profile(
        username: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> ZomeApiResult<Profile> {
        profile::handlers::create_profile(username, first_name, last_name, email)
    }

    /** Temporary Guillem solution **/
    #[zome_fn("hc_public")]
    fn set_username(username: String) -> ZomeApiResult<Address> {
        username_mod::set_username(username)?;

        Ok(hdk::AGENT_ADDRESS.clone())
    }

    #[zome_fn("hc_public")]
    fn get_all_agents() -> ZomeApiResult<Vec<Username>> {
        username_mod::get_all_agents()
    }

    #[zome_fn("hc_public")]
    fn get_username(agent_address: Address) -> ZomeApiResult<Option<String>> {
        username_mod::get_username(agent_address)
    }

    #[zome_fn("hc_public")]
    fn get_my_address() -> ZomeApiResult<Address> {
        Ok(hdk::AGENT_ADDRESS.clone())
    }
}
