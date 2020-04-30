#![allow(dead_code)]
#![allow(unused_imports)]

use serde_derive::{Deserialize, Serialize};
use holochain_json_derive::DefaultJson;
use hdk::{
    self,
    entry,
    from,
    link,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        // time::Timeout,
        // time::Iso8601,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    api::AGENT_ADDRESS,
    prelude::*,
    holochain_persistence_api::cas::content::Address
};
use std::collections::{
    hash_map::DefaultHasher,
    HashMap
};
use std::hash::{
    Hash, 
    Hasher
};

pub mod handlers;
pub mod validation;
pub mod strings;
use strings::*;
// MAIN MODULE UNDER THE PROFILE CRATE
// contains data structure definitions and implementations, and entry definitions

// Profile Entry
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Profile {
    agent_id: Address,
    username: Username,
    first_name: String, // can be Option<String>
    last_name: String, // can be Option<String>
}
// Username Entry
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Username {
    pub agent_id: Address,
    pub username: String,
}
// Hashed Email
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "snake_case")]
pub struct HashedEmail {
    agent_id: Address,
    email_hash: u64,
}
// Email table
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Email {
    email: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BooleanReturn {
    pub value: bool,
}

// IMPLEMENTATIONS OF STRUCTS
// Profile; new(), entry()
impl Profile {
    // a new() function that will generate a new public profile struct with the given arguments
    pub fn new(username: Username, first_name: String, last_name: String) -> Self {
        Profile {
            agent_id: AGENT_ADDRESS.to_string().into(),
            username,
            first_name,
            last_name,
        }
    }

    pub fn entry(self) -> Entry {
        Entry::App(PROFILE_ENTRY_NAME.into(), self.into())
    }
}

impl Email {
    pub fn new(email: String) -> Self {
        Email {
            email,
        }
    }
}

impl Hash for Email {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

impl HashedEmail {
    pub fn new(email_hash: u64) -> Self {
        HashedEmail {
            agent_id: AGENT_ADDRESS.to_string().into(),
            email_hash: email_hash
        }
    }

    pub fn entry(self) -> Entry {
        Entry::App(HASHED_EMAIL_ENTRY_NAME.into(), self.into())
    }
}

impl Username {
    pub fn new(username: String) -> Self {
        Username {
            agent_id: AGENT_ADDRESS.to_string().into(),
            username,
        }
    }

    pub fn entry(self) -> Entry {
        Entry::App(USERNAME_ENTRY_NAME.into(), self.into())
    }
}

// DEFINITIONS
// Profile
pub fn profile_definition() -> ValidatingEntryType {
    entry!(
        name: PROFILE_ENTRY_NAME,
        description: "this is the profile spec of the user",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Profile>| {
            // match validation_data {
            //     hdk::EntryValidationData::Create{entry, validation_data} => {
            //         if !validation_data.sources().contains(&entry.agent_id) {
            //             return Err("Other agents cannot create a profile for another agent".to_string());
            //         }
            //         validation::validate_profile_create(entry, validation_data)
            //     },
            //     _ => Ok(())
            // }
            Ok(())
        },
        links: [
            from!(
                "%username",
                link_type: USERNAME_PROFILE_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                link_type: AGENT_PROFILE_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}

// Hashed Email
pub fn hashed_email_definition() -> ValidatingEntryType {
    entry!(
        name: HASHED_EMAIL_ENTRY_NAME,
        description: "this is a hash of a registered email",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<HashedEmail>| {
            Ok(())
        },
        links: [
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: HASHED_EMAIL_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                link_type: AGENT_HASHED_EMAIL_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}

// Username
pub fn username_definition() -> ValidatingEntryType {
    entry!(
        name: USERNAME_ENTRY_NAME,
        description: "this is the username of the agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Username>| {
            // match validation_data {
            //     hdk::EntryValidationData::Create{entry, validation_data} => {
            //         if !validation_data.sources().contains(&entry.agent_id) {
            //             return Err("Other agents cannot create a profile for another agent".to_string());
            //         }
            //         validation::validate_profile_create(entry, validation_data)
            //     },
            //     _ => Ok(())
            // }
            Ok(())
        },
        links: [
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: USERNAME_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                link_type: "agent->username",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}