use crate::utils::{
    database_types::{Asset, DBItem, DeckList, Note, NoteList, UpdateRecipe, UpdateRecipes, UpdateType, UpdateValues}, 
    outcomes::Outcome, proceed, query::{query_dynamo, ValidQueryTypes}, 
    shared_truth::LOCAL_USER_INFO_KEY, 
    shared_utilities::{store_item_in_local_storage, UserState}, 
    user_types::{user_from_dynamo, UserInfo},
};
use std::str::FromStr;
use leptos::logging::debug_warn;
use super::asset::asset_from_s3;

/// Frontend Imports
#[cfg(feature="hydrate")]
use crate::utils::{
    front_utils::frontend_query_validation,
    cache::{get_notes_from_cache, update_cache, get_user_info_from_cache, clear_cache,},
};

pub async fn retrieve_notes(query: ValidQueryTypes, all_user_decks: DeckList) -> Outcome {
    #[cfg(feature="hydrate")]
    match get_notes_from_cache(&query).await {
        Outcome::ItemsFound(note_list_str) => return Outcome::ItemsFound(note_list_str),
        _ => proceed(),
    }
    #[cfg(feature="hydrate")]
    match frontend_query_validation(&query, all_user_decks) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    }

    let notes_str = match query_dynamo(query).await {
        Ok(outcome) => match outcome {
            Outcome::ItemsFound(string) => string,
            any_other_outcome => return any_other_outcome,
        },
        Err(e) => return Outcome::UnspecifiedQueryFailure(e.to_string()),
    };

    let Ok(notes) = NoteList::from_str(&notes_str) else {return Outcome::UnspecifiedQueryFailure("Notes could not be parsed".to_string())};

    let mut cache_recipes = UpdateRecipes::default();

    for note in notes.iter() {
        cache_recipes.recipes.push(UpdateRecipe {
            update_type: UpdateType::Swap,
            update_key: Note::FULL_NOTE_CACHE_KEY.to_owned(),
            value: UpdateValues::Note(note.clone()),
            update_item: DBItem::Note(note.deck_id, note.note_id),
        })
    }

    #[cfg(feature="hydrate")]
    update_cache(cache_recipes).await;

    Outcome::ItemsFound(notes_str)
}

pub async fn get_user_info(user_state: &UserState) -> UserInfo {
    debug_warn!("get user info called");

    if !!!user_state.is_authenticated() {
        debug_warn!("cannot get user info because user is not authenticated");
        return UserInfo::default();
    };

    #[cfg(feature="hydrate")]
    match get_user_info_from_cache()  {
        Ok(cached_user_info) => return cached_user_info,
        Err(_) => (),
    };

    let user = match user_from_dynamo().await.unwrap_or_default() {
        Outcome::UserFound(user) => {
            #[cfg(feature="hydrate")]
            match store_item_in_local_storage(LOCAL_USER_INFO_KEY, &user.to_string()) {
                Ok(_) => debug_warn!("user info successfully cached"),
                Err(_) => {
                    match clear_cache(LOCAL_USER_INFO_KEY) {
                        Ok(_) => proceed(),
                        Err(_) => proceed(),
                    }
                    debug_warn!("user info could not be cached")
                },
            }
            user
        },
        _any_other_outcome => return UserInfo::default(),
    };
    
    user
}

pub async fn get_asset(asset: Asset) -> Outcome {
    if asset == Asset::default() {
        return Outcome::UnresolvedOutcome;
    };

    let asset = match asset {
        Asset::CachedPFP(asset_str, _) => {
            let Ok(original_asset) = Asset::from_str(&asset_str) else {
                return Outcome::PresignedUrlNotRetrieved(
                    "cached asset could not be converted back to original asset format".to_string(),
                );
            };
            original_asset
        }
        any_other_asset => any_other_asset,
    };

    let outcome = match asset_from_s3(asset).await {
        Ok(outcome) => outcome,
        Err(e) => return Outcome::PresignedUrlNotRetrieved(e.to_string()),
    };

    outcome
}
