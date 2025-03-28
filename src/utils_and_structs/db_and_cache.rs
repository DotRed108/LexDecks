use super::{user_types::UserInfo, database_types::{Asset, DBItem, DeckList, Note, NoteList, UpdateRecipe, UpdateRecipes, UpdateType, UpdateValues}, outcomes::Outcome, proceed, queries::{ValidQueryTypes, QUERY_PARAM_FOR_QUERIES}, shared_truth::{AUTH_TOKEN_HEADER, LOCAL_AUTH_TOKEN_KEY, LOCAL_USER_INFO_KEY}};
use gloo_net::http::{Method, RequestBuilder};
use indexed_db::Factory;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use std::{io::Error as ioError, str::FromStr, sync::atomic::AtomicBool};
use leptos::{logging::debug_warn, web_sys::window};

const QUERY_DB_LAMBDA_URL: &str = "";
const GET_USER_INFO_LAMBDA_URL: &str = "";
const GET_ASSET_LAMBDA_URL: &str = "";

use super::front_utils::{frontend_query_validation, get_item_from_local_storage, image_cached, s3_url_expired, store_item_in_local_storage, UserState};

const DECKS_DB_NAME: &str = "test";

pub async fn retrieve_notes(query: ValidQueryTypes, token: &str, all_user_decks: DeckList) -> Outcome {
    match get_notes_from_cache(&query).await {
        Outcome::ItemsFound(note_list_str) => return Outcome::ItemsFound(note_list_str),
        _ => proceed(),
    }

    match frontend_query_validation(&query, all_user_decks) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    }
    debug_warn!("query: {}", query.to_string());
    // else 
    let notes_str = match get_notes_from_dynamo(&query, token).await {
        Outcome::ItemsFound(string) => string,
        any_other_outcome => return any_other_outcome,
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

    update_cache(cache_recipes).await;

    Outcome::ItemsFound(notes_str)
}

pub async fn get_notes_from_cache(_query: &ValidQueryTypes) -> Outcome {
    Outcome::ItemsNotFound
}

pub async fn get_notes_from_dynamo(query: &ValidQueryTypes, token: &str) -> Outcome {

    let url = format!("{}?{}={}", QUERY_DB_LAMBDA_URL, QUERY_PARAM_FOR_QUERIES, query.to_string());
    let response = RequestBuilder::new(&url)
        .method(Method::GET)
        .header(AUTH_TOKEN_HEADER, token)
        .send()
        .await;
    

    let response = match response {
        Ok(resp) => resp.text().await,
        Err(e) => return Outcome::UnspecifiedQueryFailure(e.to_string()),
    };

    let Ok(outcome_str) = response else {return Outcome::UnspecifiedQueryFailure("could not find string in response".to_string())};

    let Ok(outcome) = Outcome::from_str(&outcome_str) else {return Outcome::UnspecifiedQueryFailure("outcome could not be parsed".to_string())};

    outcome
}

pub async fn update_notes_cache(cache_recipe: UpdateRecipe) -> Outcome {
    let _update_key = cache_recipe.update_key.as_str();
    let DBItem::Note(deck_id, _) = cache_recipe.update_item else {return Outcome::CacheFailed("Not a note".to_string())};

    let UpdateValues::Note(note) = cache_recipe.value else {return Outcome::CacheFailed("could not find note".to_string())};

    let factory = match Factory::<ioError>::get() {
        Ok(fac) => fac,
        Err(e) => return Outcome::CacheFailed(e.to_string()),
    };

    static DELETE_DB: AtomicBool = AtomicBool::new(false);
    if DELETE_DB.load(std::sync::atomic::Ordering::Relaxed) {
        match factory.delete_database(DECKS_DB_NAME).await {
            Ok(_) => return Outcome::CacheSucceeded,
            Err(_) => return Outcome::CacheFailed("Could not delete db".to_string()),
        }
    }

    let db = match factory.open_latest_version(DECKS_DB_NAME).await {
        Ok(db) => db,
        Err(e) => return Outcome::CacheFailed(e.to_string()),
    };

    let current_stores = db.object_store_names();

    let db = if !!!current_stores.contains(&deck_id.to_string()) {
        debug_warn!("did we make it here {:?}", current_stores);
        let new_version = db.version() + 1;
        db.close();
        match factory.open(DECKS_DB_NAME, new_version, move |evt| async move {
            debug_warn!("did we make it here {}, {}", evt.old_version(), evt.new_version());
            let db = evt.database();
            let store = db.build_object_store(&deck_id.to_string()).auto_increment().key_path(&Note::FIELD_NAMES.note_id).create()?;
            store.build_index(&Note::FIELD_NAMES.level, &Note::FIELD_NAMES.level).create()?;
            store.build_index(&Note::FIELD_NAMES.version, &Note::FIELD_NAMES.version).create()?;
            
            Ok(())
        }).await {
            Ok(db) => db,
            Err(e) => return Outcome::CacheFailed(e.to_string()),
        }
    } else {
        db
    };

    let transaction = db.transaction(&[&deck_id.to_string()]).rw();

    match transaction.run(move |trans| async move {
        let store = trans.object_store(&deck_id.to_string())?;
        let note_json = match note.serialize(&Serializer::json_compatible()) {
            Ok(json) => json,
            Err(_) => return Err(indexed_db::Error::OperationNotSupported),
        };
        store.put(&note_json).await?;
        Ok(())
    }).await {
        Ok(_) => (),
        Err(e) => return Outcome::CacheFailed(e.to_string()),
    }

    db.close();
    Outcome::CacheSucceeded
}

pub async fn update_cache(cache_recipes: UpdateRecipes) {
    for cache_recipe in cache_recipes.recipes {
        match cache_recipe.update_item {
            DBItem::User(_) => match update_user_cache(cache_recipe) {
                Ok(_) => debug_warn!("cache updated?"),
                Err(_) => debug_warn!("could not update user cache"),
            },
            DBItem::Note(_, _) => match update_notes_cache(cache_recipe).await {
                Outcome::CacheSucceeded => debug_warn!("cache updated?"),
                Outcome::CacheFailed(e) => debug_warn!("could not update note cache {}", e.to_string()),
                any_other_outcome => debug_warn!("attempt to cache notes produced the following outcome {}", any_other_outcome.to_string()),
            },
        }
    }
}

fn update_user_cache(cache_recipe: UpdateRecipe) -> Result<(), ()> {
    debug_warn!("update_user_cache called with recipe {}", cache_recipe.to_string());
    let Ok(mut user_info_cache) = get_user_info_from_cache() else {
        return Err(());
    };

    let update_key = cache_recipe.update_key.as_str();

    match update_key {
        UserInfo::ACTIVE_DECKS_CACHE_KEY | UserInfo::OWNED_DECKS_CACHE_KEY | UserInfo::COLAB_DECKS_CACHE_KEY => {
            let UpdateValues::DeckList(deck_list) = cache_recipe.value else {debug_warn!("deck list err");return Err(())};
            let decks: &mut DeckList;
            if update_key == UserInfo::ACTIVE_DECKS_CACHE_KEY {
                decks = &mut user_info_cache.active_decks;
            } else if update_key == UserInfo::OWNED_DECKS_CACHE_KEY {
                decks = &mut user_info_cache.owned_decks;
            } else {
                decks = &mut user_info_cache.colab_decks;
            }

            match cache_recipe.update_type {
                UpdateType::Add => decks.add_decks_wo_dupes(deck_list),
                UpdateType::Subtract => decks.remove_decks(deck_list),
                UpdateType::Swap => *decks = deck_list,
            }
        },
        UserInfo::LAST_LOGIN_CACHE_KEY => {
            let UpdateValues::Unsigned64(new_date) = cache_recipe.value else {debug_warn!("could not parse date"); return Err(())};
            match cache_recipe.update_type {
                UpdateType::Swap => user_info_cache.last_login = new_date,
                UpdateType::Add => user_info_cache.last_login = user_info_cache.last_login + new_date,
                UpdateType::Subtract => user_info_cache.last_login = user_info_cache.last_login - new_date,
            }
        },
        UserInfo::UPLOAD_TOKENS_CACHE_KEY => {
            let UpdateValues::Float64(tokens) = cache_recipe.value else {debug_warn!("could not parse tokens"); return Err(())};
            match cache_recipe.update_type {
                UpdateType::Add => user_info_cache.upload_tokens = user_info_cache.upload_tokens + tokens,
                UpdateType::Subtract => user_info_cache.upload_tokens = user_info_cache.upload_tokens - tokens,
                UpdateType::Swap => user_info_cache.upload_tokens = tokens,
            }
        }
        key => {
            debug_warn!("cannot update user because the key: {key} is an unhandled pattern");
            return Err(());
        }
    }

    return store_item_in_local_storage(LOCAL_USER_INFO_KEY, &user_info_cache.to_string());
}

pub fn get_user_info_from_cache() -> Result<UserInfo, ()> {
    let user_info_as_str = match get_item_from_local_storage(LOCAL_USER_INFO_KEY) {
        Some(cached_user_info) => cached_user_info,
        None => return Err(()),
    };

    let user_info = match UserInfo::from_str(&user_info_as_str) {
        Ok(user_info) => user_info,
        Err(_) => {
            debug_warn!("could not get parse user info from cache...\nremoving user info cache...");
            match clear_cache(LOCAL_USER_INFO_KEY) {
                _ => return Err(()),
            }
        }
    };

    Ok(user_info)
}

pub fn get_user_state_from_cache() -> UserState {
    let stored_token = match get_item_from_local_storage(LOCAL_AUTH_TOKEN_KEY) {
        Some(auth_token) => auth_token,
        None => {
            return {
                debug_warn!("auth token not retrieved from cache user state set to default");
                UserState::default()
            }
        }
    };

    let user_state = UserState::from_stored_token_or_default(&stored_token);

    user_state
}

pub async fn get_user_info(user_state: &UserState) -> UserInfo {
    debug_warn!("get user info called");

    if !user_state.is_authenticated() {
        debug_warn!("cannot get user info because user is not authenticated");
        return UserInfo::default();
    };

    match get_user_info_from_cache()  {
        Ok(cached_user_info) => return cached_user_info,
        Err(_) => (),
    };

    debug_warn!("are we making it here");

    let token = user_state.token();

    let response = RequestBuilder::new(GET_USER_INFO_LAMBDA_URL)
        .method(Method::GET)
        .header(AUTH_TOKEN_HEADER, token)
        .send()
        .await;

    let response_text = match response {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(e) => {debug_warn!("response could not be parsed to text {}", e.to_string()); return UserInfo::default()},
        },
        Err(e) => {debug_warn!("server responded with error {}", e.to_string()); return UserInfo::default()},
    };

    let outcome = match Outcome::from_str(&response_text) {
        Ok(outcome) => outcome,
        Err(_) => {debug_warn!("outcome within response could not be parsed"); return UserInfo::default()},
    };

    let user_info = match outcome {
        Outcome::UserFound(user) => {

            let user_as_str = user.to_string();

            
            match store_item_in_local_storage(LOCAL_USER_INFO_KEY, &user_as_str) {
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
        }
        any_other_outcome => {debug_warn!("user info not found {}", any_other_outcome.to_string()); return UserInfo::default()},
    };
    
    user_info
}

pub fn clear_cache(cache_key: &str) -> Result<(), ()> {
    match window().unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .remove_item(cache_key)
    {
        Ok(_) => {
            debug_warn!("cache with key: {} removed", cache_key);
            return Ok(());
        }
        Err(_) => {
            debug_warn!("cache with key: {} failed to be removed", cache_key);
            return Err(());
        }
    }
}

#[allow(unreachable_code)]
pub async fn cache_and_return_asset(cache_key: &str, asset: Asset, token: &str) -> Option<Asset> {
    if asset == Asset::default() {
        return None;
    }
    let asset_to_cache = match asset {
        Asset::CachedPFP(original_asset_str, url) => {
            let asset: Asset;

            // false true
            if s3_url_expired(&url) && !!!image_cached(&url) {
                let Ok(original_asset) = Asset::from_str(&original_asset_str) else {
                    debug_warn!("Original asset could not be parsed");
                    return None;
                };
                asset = match get_asset(token, original_asset.clone()).await {
                    Outcome::PresignedUrlRetrieved(uri) => {
                        Asset::CachedPFP(original_asset.to_string(), uri)
                    }
                    _any_other_outcome => {
                        debug_warn!("asset could not be retrieved");
                        return None;
                    }
                }
            } else {
                return Some(Asset::CachedPFP(original_asset_str, url));
            }

            asset
        }
        any_other_asset => match get_asset(token, any_other_asset.clone()).await {
            Outcome::PresignedUrlRetrieved(url) => Asset::CachedPFP(any_other_asset.to_string(), url),
            any_other_outcome => {
                debug_warn!("asset could not be retrieved {}", any_other_outcome.to_string());
                return None;
            }
        },
    };

    let original_asset = match asset_to_cache.clone() {
        Asset::CachedPFP(original, _) => match Asset::from_str(&original) {
            Ok(og_asset) => og_asset,
            Err(_) => {
                debug_warn!("og asset could not be parsed");
                return None;
            }
        },
        _ => {
            debug_warn!("attempted to cache uncachable asset");
            return None;
        }
    };

    match original_asset {
        Asset::PFP(_) => {
            let value_to_cache;

            if cache_key == LOCAL_USER_INFO_KEY {
                let mut cached_user_info = match get_user_info_from_cache() {
                    Ok(info) => info,
                    Err(()) => return None,
                };

                cached_user_info.pfp = asset_to_cache.clone();

                value_to_cache = cached_user_info.to_string();
            } else {
                value_to_cache = todo!();
            }

            debug_warn!("this is being hit");
            match store_item_in_local_storage(cache_key, &value_to_cache) {
                Ok(_) => (),
                Err(_) => {
                    debug_warn!("asset could not be cached");
                    return None;
                }
            }
        }
        Asset::DeckImage(_) => {
            debug_warn!("caching deck images is not yet implemented");
            return None;
        }
        _ => {
            debug_warn!("if this error is ever triggered im killing myself");
            return None;
        }
    };

    Some(asset_to_cache)
}

pub async fn get_asset(token: &str, asset: Asset) -> Outcome {
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
    let response = RequestBuilder::new(GET_ASSET_LAMBDA_URL)
        .method(Method::POST)
        .header(AUTH_TOKEN_HEADER, token)
        .body(asset.to_string())
        .expect("method cannot be get if sending body")
        .send()
        .await;

    let response = match response {
        Ok(resp) => resp,
        Err(e) => return Outcome::PresignedUrlNotRetrieved(e.to_string()),
    };

    let outcome = match response.text().await {
        Ok(o) => Outcome::from_str(&o).unwrap_or(Outcome::PresignedUrlNotRetrieved(
            "couldnt convert response to outcome".to_string(),
        )),
        Err(e) => return Outcome::PresignedUrlNotRetrieved(e.to_string()),
    };

    outcome
}