use crate::utils::{ 
    database_types::{DeckId, DeckMeta, Field, Note, NoteList, NoteType}, 
    outcomes::Outcome, 
    proceed, 
    shared_truth::MAX_LEVELS,
    auth_client::AuthClient,
};
use std::{collections::HashMap, str::FromStr};
use leptos::{prelude::ServerFnError, server};
use serde::{Deserialize, Serialize};

pub const QUERY_PARAM_FOR_QUERIES: &str = "QueryType";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ValidQueryTypes {
    #[default] NoQuery,
    NotesById(DeckId, Vec<usize>),
    NotesByLevel(DeckId, Vec<usize>),
    NotesByType(DeckId, Vec<NoteType>),
}


/// Server Imports
#[cfg(feature="ssr")]
use crate::utils::{
    back_utils::{PUBLIC_DECKS_TABLE, verify_user_header},
    dynamo_utils::{convert_attribute_value_to_string, setup_client, validate_if_in_any_decks_and_if_good_standing, DECK_ID_DB_KEY, DECK_META_DB_KEY, LEVEL_DB_KEY, NOTE_ID_DB_KEY, NOTE_TYPE_DB_KEY, REVIEWS_PER_STAGE_DB_KEY, VERSION_DB_KEY},
};
#[cfg(feature="ssr")]
use aws_sdk_dynamodb::{types::AttributeValue, Client as DDBClient};


#[server(client=AuthClient)]
pub async fn query_dynamo(query_type: ValidQueryTypes) -> Result<Outcome, ServerFnError> {
    let Outcome::VerificationSuccess(email) = verify_user_header().await else {return Ok(Outcome::VerificationFailure)};
    
    let outcome = match query_type {
        ValidQueryTypes::NoQuery => Outcome::InvalidRequest,
        query_type => query(query_type, &email).await,
    };

    Ok(outcome)
}

#[cfg(feature="ssr")]
async fn query_by_level(client: &DDBClient, partition_key: String, level: usize) -> Outcome {

    println!("!!!!!!!!!!!!!!!!!!! AHHHHHHHHHHHHHHHHH !!!!!!!!!!!!!!!!!!!!!!!!!! pk: {}, level: {}", partition_key, level);
    let output = match client.query()
    .table_name(PUBLIC_DECKS_TABLE)
    .index_name(format!("{LEVEL_DB_KEY}-index"))
    .consistent_read(false)
    .key_condition_expression("#DeckID = :pk AND #Level = :lvl")
    .expression_attribute_names("#DeckID", DECK_ID_DB_KEY)
    .expression_attribute_names("#Level", LEVEL_DB_KEY)
    .expression_attribute_values(":pk", AttributeValue::S(partition_key))
    .expression_attribute_values(":lvl", AttributeValue::N(level.to_string()))
    .send().await {
        Ok(output) => output,
        Err(e) => return Outcome::UnspecifiedQueryFailure(e.into_service_error().to_string()),
    };

    let mut note_list = NoteList::default();

    for item in output.items() {
        note_list.push(construct_note_from_database_item(&item));
    }

    Outcome::ItemsFound(note_list.to_string())
}

#[cfg(feature="ssr")]
pub fn construct_note_from_database_item(note_attribute_map: &HashMap<String, AttributeValue>) -> Note {
    let mut note = Note::default();
    for (attribute_name, attribute_value) in note_attribute_map {
        let value_as_str = convert_attribute_value_to_string(&attribute_value, "|!|");
        match attribute_name.as_str() {
            NOTE_ID_DB_KEY => note.note_id = u64::from_str(&value_as_str).unwrap(),
            NOTE_TYPE_DB_KEY => note.note_type = NoteType::from_str(&value_as_str).unwrap_or_default(),
            VERSION_DB_KEY => note.version = u64::from_str(&value_as_str).unwrap_or_default(),
            REVIEWS_PER_STAGE_DB_KEY => note.reviews_per_stage = u8::from_str(&value_as_str).unwrap_or_default(),
            LEVEL_DB_KEY => note.level = u32::from_str(&value_as_str).unwrap_or_default(),
            DECK_META_DB_KEY => {
                note.meta = match DeckMeta::from_str(&value_as_str) {
                    Ok(meta) => Some(meta),
                    Err(_) => None,
                }
            },
            DECK_ID_DB_KEY => note.deck_id = DeckId::from_str(&value_as_str).unwrap_or_default(),
            field_name => note.fields.push(Field::new(field_name.to_owned(), Some(value_as_str), None)),
        }
    }
    note
}

#[cfg(feature="ssr")]
async fn _query_by_note_type() -> Outcome {
    todo!()
}
#[cfg(feature="ssr")]
async fn _query_by_note_id() -> Outcome {
    todo!()
}
#[cfg(feature="ssr")]
async fn _query_by_reviews_per_stage() -> Outcome {
    todo!()
}

#[cfg(feature="ssr")]
async fn is_valid_query(client: &DDBClient, query_type: &ValidQueryTypes, email: &str) -> Outcome {
    match query_type {
        ValidQueryTypes::NotesByLevel(deck_id, levels) => {
            match validate_if_in_any_decks_and_if_good_standing(client, email, &deck_id.to_string()).await {
                Outcome::PermissionGranted(_) => proceed(),
                any_other_outcome => return any_other_outcome,
            }
            for level in levels {
                if level > &MAX_LEVELS {return Outcome::InvalidRequest}
            }
        },
        ValidQueryTypes::NotesById(deck_id, _) => {
            match validate_if_in_any_decks_and_if_good_standing(client, email, &deck_id.to_string()).await {
                Outcome::PermissionGranted(_) => proceed(),
                any_other_outcome => return any_other_outcome,
            }
        },
        ValidQueryTypes::NotesByType(deck_id, _) => {
            match validate_if_in_any_decks_and_if_good_standing(client, email, &deck_id.to_string()).await {
                Outcome::PermissionGranted(_) => proceed(),
                any_other_outcome => return any_other_outcome,
            }
        },
        _all_other_queries => return Outcome::InvalidRequest,
    }
    Outcome::PermissionGranted("Likely valid query".to_string())
}

#[cfg(feature="ssr")]
async fn query(query_type: ValidQueryTypes, email: &str) -> Outcome {
    let client = &setup_client().await;

    match is_valid_query(client, &query_type, email).await {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    }

    let outcome = match query_type {
        ValidQueryTypes::NotesByLevel(partition_key, levels) => {
            for level in levels {
                return query_by_level(client, partition_key.to_string(), level).await
            }
            Outcome::IncorrectType
        },
        _any_other_query => Outcome::InvalidRequest,
    };
    outcome
}