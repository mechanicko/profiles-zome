#![allow(dead_code)]
#![allow(unused_imports)]

use hdk::{
    error::ZomeApiResult,
    // holochain_core_types::{
    //     entry::Entry,
    // },
    holochain_persistence_api::cas::content::{
        Address,
    },
    prelude::*,
    AGENT_ADDRESS
};
use holochain_anchors::anchor;
use crate::profile::{
    Profile,
    Username,
    HashedEmail,
    Email,
};
use crate::profile::strings::*; 
use std::collections::hash_map::DefaultHasher;
use std::hash::{
    Hash, 
    Hasher
};


// HANDLER MODULE UNDER THE PROFILE CRATE

// anchor_username()
// attach anchors to newly created usernames
// anchor format: 
//      anchor type: 'USERNAME_ANCHOR_<first character of username>'
//      anchor text: 'USERNAMES_<first character of username>'
fn anchor_username(anchor_type: String, anchor_text: String, username: String) -> ZomeApiResult<Address> {
    let first_letter = username.chars().next().unwrap().to_ascii_lowercase();
    let type_string = format!("{}{}{}", anchor_type, "_", first_letter);
    let text_string = format!("{}{}{}", anchor_text, "_", first_letter);
    anchor(type_string.to_string(), text_string.to_string())
}
fn anchor_hashed_email(anchor_type: String, anchor_text: String) -> ZomeApiResult<Address> {
    anchor(anchor_type.to_string(), anchor_text.to_string())
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn compare_hashes (hash: u64) -> ZomeApiResult<bool> {
    let matches = hdk::get_links(
        &anchor_hashed_email(
            HASHED_EMAIL_ANCHOR_TYPE.to_string(),
            HASHED_EMAIL_ANCHOR_TEXT.to_string(),
        )?,
        LinkMatch::Exactly(HASHED_EMAIL_LINK_TYPE),
        LinkMatch::Any
    )?.addresses().into_iter();
    let mut result = false;
    for address in matches {
        let entry = get_hashed_email(address)?;
        if entry.email_hash == hash {
            result = true;
            break;
        } else {
            ()
        }
    };
    Ok(result)
}

pub fn check_email (email: String) -> ZomeApiResult<bool> {
    let input_email = Email::new(email);
    let input_email_hash = calculate_hash(&input_email);
    compare_hashes(input_email_hash)
}

// check_username()
// argument(s): username
// return value: bool
pub fn check_username(username: String) -> ZomeApiResult<bool> {
    let username_checker = hdk::get_links(
        &anchor_username(
            USERNAME_ANCHOR_TYPE.to_string(), 
            USERNAMES_ANCHOR_TEXT.to_string(),
            username.clone()
        )?, 
        LinkMatch::Exactly(USERNAME_LINK_TYPE), 
        LinkMatch::Exactly(&username)
    )?.addresses().is_empty();
    let mut result = false;
    if username_checker == false {
        result = true;
    }
    Ok(result)
}

pub fn create_profile(
    username: String,
    first_name: String,
    last_name: String,
    email: String
) -> ZomeApiResult<Profile> {
    let new_email = Email::new(email);
    let new_email_hash = calculate_hash(&new_email);
    let new_hashed_email_entry  = HashedEmail::new(new_email_hash.clone()).entry();
    let new_hashed_email_address  = hdk::commit_entry(&new_hashed_email_entry)?;

    hdk::link_entries(
        &AGENT_ADDRESS,                                 // base
        &new_hashed_email_address,                      // target
        AGENT_HASHED_EMAIL_LINK_TYPE,                   // link_type
        "hashed_email"                                  // tag
    )?;

    hdk::link_entries(
        &anchor_hashed_email(
            HASHED_EMAIL_ANCHOR_TYPE.to_string(),
            HASHED_EMAIL_ANCHOR_TEXT.to_string()
        )?, 
        &new_hashed_email_address,                                       
        HASHED_EMAIL_LINK_TYPE,                         
        &new_email_hash.to_string()                        
    )?;

    let new_username = Username::new(username.clone());
    let new_username_entry = new_username.clone().entry();
    let new_username_address = hdk::commit_entry(&new_username_entry)?;

    hdk::link_entries(
        &AGENT_ADDRESS,                             // base
        &new_username_address,                      // target
        AGENT_USERNAME_LINK_TYPE,                   // link_type
        "username"                                  // tag
    )?;

    hdk::link_entries(
        &anchor_username(
            USERNAME_ANCHOR_TYPE.to_string(),
            USERNAMES_ANCHOR_TEXT.to_string(),
            username.clone().to_ascii_lowercase()     // <username input> to concatenate to anchor type and text
        )?,  
        &new_username_address,                                       
        USERNAME_LINK_TYPE,                         
        &username.to_ascii_lowercase()                      
    )?;

    let new_profile = Profile::new(new_username, first_name, last_name);
    let profile_address = hdk::commit_entry(&new_profile.clone().entry())?;
    hdk::link_entries(
        &new_username_address, 
        &profile_address,                                   // profile_address of the entry in the dht
        USERNAME_PROFILE_LINK_TYPE,                         // USERNAME->PROFILE
        "",
    )?;

    hdk::link_entries(
        &AGENT_ADDRESS,                                     // base
        &profile_address,                                   // target
        AGENT_PROFILE_LINK_TYPE,                            // link_type
        "profile"                                    // tag
    )?;
    Ok(new_profile)
}

// get_profile()
// argument(s): Address
// return value: Profile

pub fn get_profile(id: Address) -> ZomeApiResult<Profile> {
    hdk::utils::get_as_type(id)
}

pub fn get_my_profile() -> ZomeApiResult<Vec<Profile>> {
    hdk::get_links(
        &AGENT_ADDRESS, 
        LinkMatch::Exactly(AGENT_PROFILE_LINK_TYPE), 
        LinkMatch::Exactly("profile")
    )?.addresses().into_iter().map(|profile_address| {
        get_profile(profile_address)
    }).collect()
}

fn get_hashed_email(id: Address) -> ZomeApiResult<HashedEmail> {
    hdk::utils::get_as_type(id)
}

// list_public_profiles()
// argument(s): none (can be changed to username)
// return value: Vector of PublicProfile
// pub fn list_public_profiles(username: String) -> ZomeApiResult<Vec<PublicProfile>> {
//     hdk::get_links(
//         &anchor_profile(
//             PUBLIC_PROFILES_ANCHOR_TYPE.to_string(),        // anchor_type: PUBLIC_PROFILE_n
//             PUBLIC_PROFILES_ANCHOR_TEXT.to_string(),        // anchor_text: PUBLIC_PROFILES_n
//             username,
//         )?, 
//         LinkMatch::Exactly(PUBLIC_PROFILE_LINK_TYPE),       // link_type: PUBLIC_PROFILE_LINK
//         LinkMatch::Any                                      // tag: nicko
//     // iterate over the Vec<ZomeApiResult<Entry>> result
//     )?.addresses().into_iter().map(|profile_address| {
//         get_public_profile(profile_address)
//     }).collect()
// }
