use std::str::FromStr;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use crate::utils_and_structs::{database_types::{DBItem, DeckId, DeckList, Note, UpdateRecipe, UpdateRecipes, UpdateType, UpdateValues}, user_types::{PartialUserInfo, Standing, UserInfo}, outcomes::Outcome, proceed, shared_truth::DECK_LIMIT};
use serde_dynamo::{aws_sdk_dynamodb_1::to_attribute_value, from_item};
use crate::utils_and_structs::back_utils::{PUBLIC_DECKS_TABLE, USERS_TABLE, is_in_active_decks};

// User DB keys
pub const PHONE_NUMBER_DB_KEY: &str = UserInfo::FIELD_NAMES.phone;
pub const EMAIL_DB_KEY: &str = UserInfo::FIELD_NAMES.email;
pub const SIGN_UP_DATE_DB_KEY: &str = UserInfo::FIELD_NAMES.sign_up_date;
pub const USERNAME_DB_KEY: &str = UserInfo::FIELD_NAMES.lex_name;
pub const USER_TYPE_DB_KEY: &str = UserInfo::FIELD_NAMES.user_type;
pub const UPLOAD_TOKENS_DB_KEY: &str = UserInfo::FIELD_NAMES.upload_tokens;
pub const RANK_DB_KEY: &str = UserInfo::FIELD_NAMES.lex_rank;
pub const PFP_DB_KEY: &str = UserInfo::FIELD_NAMES.pfp;
pub const STANDING_DB_KEY: &str = UserInfo::FIELD_NAMES.standing;
pub const ACTIVE_DECKS_DB_KEY: &str = UserInfo::FIELD_NAMES.active_decks;
pub const OWNED_DECKS_DB_KEY: &str = UserInfo::FIELD_NAMES.owned_decks;
pub const COLAB_DECKS_DB_KEY: &str = UserInfo::FIELD_NAMES.colab_decks;
pub const SETTINGS_DB_KEY: &str = UserInfo::FIELD_NAMES.settings;
pub const LAST_LOGIN_DB_KEY: &str = UserInfo::FIELD_NAMES.last_login;

// Note DB keys
pub const DECK_ID_DB_KEY: &str = Note::FIELD_NAMES.deck_id;
pub const NOTE_ID_DB_KEY: &str = Note::FIELD_NAMES.note_id;
pub const LEVEL_DB_KEY: &str = Note::FIELD_NAMES.level;
pub const NOTE_TYPE_DB_KEY: &str = Note::FIELD_NAMES.note_type;
pub const REVIEWS_PER_STAGE_DB_KEY: &str = Note::FIELD_NAMES.reviews_per_stage;
pub const DECK_META_DB_KEY: &str = Note::FIELD_NAMES.meta;
pub const VERSION_DB_KEY: &str = Note::FIELD_NAMES.version;

pub async fn setup_client() -> Client {
    let config = aws_config::defaults(BehaviorVersion::latest()).region(Region::new("us-east-2")).load().await;
    Client::new(&config)
}

pub async fn get_user(client: &Client, email: &str, projection_expression: Option<&str>) -> Outcome {
    let pk = AttributeValue::S(email.to_string());
    let get_item_builder = client.get_item().table_name(USERS_TABLE).key(EMAIL_DB_KEY, pk);
    
    let get_item_result = match projection_expression {
        Some(expression) => get_item_builder.projection_expression(expression).send().await,
        None => get_item_builder.send().await,
    };

    let option_item = match get_item_result {
        Ok(output) => output.item,
        Err(_) => return Outcome::UserNotFound,
    };

    let item = match option_item {
        Some(map) => map,
        None => return Outcome::UserNotFound,
    };

    let user = match projection_expression {
        Some(_) => {
            let partial_user: PartialUserInfo = match from_item(item) {
                Ok(partial_user) => partial_user,
                Err(_) => return Outcome::IncorrectType,
            };

            UserInfo::from(partial_user)
        },
        None => match from_item(item) {
            Ok(user) => user,
            Err(_) => return Outcome::IncorrectType
        },
    };

    Outcome::UserFound(user)
}

pub fn permission_if_good_standing(user: &UserInfo) -> Outcome {
    match user.standing {
        Standing::WUser => return Outcome::PermissionGranted("User is not suspended".to_string()),
        Standing::Suspended(date) => return Outcome::UserSuspended(date),
    }
}

pub fn permission_if_premium_user(user: &UserInfo) -> Outcome {
    if user.user_type.is_premium_user() {
        Outcome::PermissionGranted("user is premium".to_string())
    } else {Outcome::UserDoesNotHavePermission}
}

pub fn is_over_deck_limit(decks: &Vec<String>) -> bool {
    if decks.len() >= DECK_LIMIT {
        return true;
    } else {
        return false;
    };
}

pub fn permission_if_in_deck_list(deck: &str, deck_list_attribute: &AttributeValue) -> Outcome {
    let active_decks = match deck_list_attribute {
        AttributeValue::Ss(vector) => vector,
        _ => return Outcome::IncorrectType,
    };

    if is_in_active_decks(active_decks, &deck.to_string()) {
        let Some(deck_list) = DeckList::from_str_vec(active_decks) else {return Outcome::VerificationFailure};
        return Outcome::PermissionGranted(deck_list.to_string())
    } else {
        return Outcome::UserDoesNotHavePermission
    }
}

pub fn permission_if_under_active_deck_limit(user: &UserInfo) -> Outcome {
    if user.active_decks.len() < DECK_LIMIT {
        Outcome::PermissionGranted("".to_string())
    } else {Outcome::UserDoesNotHavePermission}
}

pub fn permission_if_under_owned_deck_limit(user: &UserInfo) -> Outcome {
    if user.owned_decks.len() < DECK_LIMIT {
        Outcome::PermissionGranted("".to_string())
    } else {Outcome::UserDoesNotHavePermission}
}

pub fn permission_if_under_colab_deck_limit(user: &UserInfo) -> Outcome {
    if user.colab_decks.len() < DECK_LIMIT {
        Outcome::PermissionGranted("".to_string())
    } else {Outcome::UserDoesNotHavePermission}
}

pub fn permission_if_enough_tokens(user: &UserInfo, cost: f64) -> Outcome {
    if user.upload_tokens >= cost {
        return Outcome::PermissionGranted(user.upload_tokens.to_string());
    } else {
        return Outcome::NotEnoughUploadTokens(user.upload_tokens);
    }
}

pub fn permission_if_in_active_decks(user: &UserInfo, deck: DeckId) -> Outcome {
    if user.active_decks.contains(&deck) {
        Outcome::PermissionGranted("Deck is in Active Decks".to_string())
    } else {Outcome::UserDoesNotHavePermission}
}

pub fn permission_if_in_owned_decks(user: &UserInfo, deck: DeckId) -> Outcome {
    if user.owned_decks.contains(&deck) {
        Outcome::PermissionGranted("Deck is in Owned Decks".to_string())
    } else {Outcome::UserDoesNotHavePermission}
}

pub fn permission_if_in_colab_decks(user: &UserInfo, deck: DeckId) -> Outcome {
    if user.colab_decks.contains(&deck) {
        Outcome::PermissionGranted("Deck is in Colab Decks".to_string())
    } else {Outcome::UserDoesNotHavePermission}
}

pub async fn validate_active_decks_and_user_standing(client: &Client, email: &str, deck_id: &str) -> Outcome {
    let deck_id = match DeckId::from_str(deck_id) {
        Ok(id) => id,
        Err(_) => return Outcome::IncorrectType,
    };

    let attributes_to_get = [STANDING_DB_KEY, ACTIVE_DECKS_DB_KEY];

    let user = match get_user(client, email, Some(&attributes_to_get.join(","))).await {
        Outcome::UserFound(user) => user,
        any_other_outcome => return any_other_outcome,
    };

    match permission_if_good_standing(&user) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    };

    permission_if_in_active_decks(&user, deck_id)
}

pub async fn validate_if_in_any_decks_and_if_good_standing(client: &Client, email: &str, deck_id: &str) -> Outcome {
    let Ok(deck_id) = DeckId::from_str(deck_id) else {return Outcome::IncorrectType};
    let attributes_to_get = [STANDING_DB_KEY, ACTIVE_DECKS_DB_KEY, OWNED_DECKS_DB_KEY, COLAB_DECKS_DB_KEY];

    let user = match get_user(client, email, Some(&attributes_to_get.join(","))).await {
        Outcome::UserFound(user) => user,
        any_other_outcome => return any_other_outcome,
    };

    match permission_if_good_standing(&user) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    };

    match permission_if_in_active_decks(&user, deck_id) {
        Outcome::PermissionGranted(_) => return Outcome::PermissionGranted("".to_string()),
        _any_other_outcome => proceed(),
    }

    match permission_if_in_colab_decks(&user, deck_id) {
        Outcome::PermissionGranted(_) => return Outcome::PermissionGranted("".to_string()),
        _any_other_outcome => proceed(),
    }

    permission_if_in_owned_decks(&user, deck_id)
}

pub async fn validate_user_type_user_standing_upload_tokens_and_deck_limits(client: &Client, email: &str, estimated_token_cost: f64) -> Outcome {
    let attributes_to_get = [STANDING_DB_KEY, UPLOAD_TOKENS_DB_KEY, USER_TYPE_DB_KEY, ACTIVE_DECKS_DB_KEY, OWNED_DECKS_DB_KEY];

    let user = match get_user(client, email, Some(&attributes_to_get.join(","))).await {
        Outcome::UserFound(user) => user,
        any_other_outcome => return any_other_outcome,
    };

    match permission_if_under_active_deck_limit(&user) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    };

    match permission_if_under_owned_deck_limit(&user) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    };

    match permission_if_good_standing(&user) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    };

    match permission_if_premium_user(&user) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return any_other_outcome,
    }

    return permission_if_enough_tokens(&user, estimated_token_cost);
}

pub async fn validate_user_standing(client: &Client, email: &str) -> Outcome {
    let user = match get_user(client, email, Some(STANDING_DB_KEY)).await {
        Outcome::UserFound(user) => user,
        any_other_outcome => return any_other_outcome,
    };

    permission_if_good_standing(&user)
}

pub async fn validate_user_existence(client: &Client, email: &str) -> Outcome {
    println!("email: {email}");
    let outcome = get_user(client, email, Some(STANDING_DB_KEY)).await;

    println!("{}", outcome.to_string());
    return outcome;
}

pub async fn validate_user_and_return_rank(client: &Client, email: &str) -> Outcome {
    let attributes_to_get = [STANDING_DB_KEY, RANK_DB_KEY];

    let user = match get_user(client, email, Some(&attributes_to_get.join(","))).await {
        Outcome::UserFound(user) => user,
        any_other_outcome => {println!("{}", any_other_outcome.to_string()); return any_other_outcome},
    };

    match permission_if_good_standing(&user) {
        Outcome::PermissionGranted(_) => return Outcome::PermissionGrantedReturnUser(user),
        any_other_outcome => return any_other_outcome
    }
}

// email (String)   active_decks   colab_decks   last_login   name   owned_decks   pfp   phone   rank   settings   sign_up_date   standing   upload_tokens   user_type

pub async fn add_deck_to_user_active_decks_and_owned_decks(client: Client, email: &str, deck_id: &str) -> Outcome {
    let email = AttributeValue::S(email.to_string());
    let Ok(id) = DeckId::from_str(deck_id) else {return Outcome::DeckCouldNotBeProcessed("Could not parse deck id".to_owned())};
    let deck = AttributeValue::Ss(vec![id.to_string()]);
    match client.update_item().table_name(USERS_TABLE).key(EMAIL_DB_KEY, email)
    .update_expression("ADD #ActiveDecks :deck, ADD #OwnedDecks :deck")
    .expression_attribute_names("#OwnedDecks", OWNED_DECKS_DB_KEY)
    .expression_attribute_names("#ActiveDecks", ACTIVE_DECKS_DB_KEY)
    .expression_attribute_values("deck", deck).send().await {
        Ok(_) => (),
        Err(e) => return Outcome::DeckCouldNotBeProcessed(e.into_service_error().to_string())
    };

    Outcome::UnresolvedOutcome
}

pub async fn update_item(client: &Client, update_recipes: Vec<UpdateRecipe>) -> Outcome {
    for recipe_list in sort_recipes_by_update_item(&update_recipes) {
        let mut update_request = match &recipe_list[0].update_item {
            DBItem::User(email) => {
                let Ok(email) = to_attribute_value(email) else {return Outcome::IncorrectType}; 
                client.update_item().table_name(USERS_TABLE).key(EMAIL_DB_KEY, email)
            },
            DBItem::Note(deck_id, note_id) => {
                let Ok(deck_id) = to_attribute_value(deck_id) else {return Outcome::IncorrectType}; 
                let Ok(note_id) = to_attribute_value(note_id) else {return Outcome::IncorrectType};
                
                client.update_item().table_name(PUBLIC_DECKS_TABLE).key(DECK_ID_DB_KEY, deck_id).key(NOTE_ID_DB_KEY, note_id)
            },
        };

        let mut expression_lists = Vec::new();
        let mut set_expressions = Vec::new();
        let mut add_expressions = Vec::new();
        let mut remove_expressions = Vec::new();
        let mut delete_expressions = Vec::new();
        let mut cache_recipe_list = UpdateRecipes {recipes: Vec::new()};

        for recipe in recipe_list {
            let key = recipe.update_key.clone();

            let update_expression;
            let value_param;

            let Ok(attribute_value) = convert_to_attr_val(&recipe.value) else {return Outcome::IncorrectType};
            println!("{:?}", attribute_value);

            match &attribute_value {
                AttributeValue::N(_) => {
                    value_param = match &key[..] {
                        UPLOAD_TOKENS_DB_KEY => ":tokens",
                        RANK_DB_KEY => ":newrank",
                        LAST_LOGIN_DB_KEY => ":newdate",
                        _ => "",
                    };
                    update_expression = match recipe.update_type {
                        UpdateType::Add => format!("SET #{key} = #{key} + {value_param}"),
                        UpdateType::Subtract => format!("SET #{key} = #{key} - {value_param}"),
                        UpdateType::Swap => format!("SET #{key} = {value_param}"),
                    };
                },
                AttributeValue::S(_) => {
                    value_param = match &key[..] {
                        PHONE_NUMBER_DB_KEY => ":whodis",
                        USERNAME_DB_KEY => ":newname",
                        USER_TYPE_DB_KEY => ":newtype",
                        PFP_DB_KEY => ":newface",
                        STANDING_DB_KEY => ":banornot",
                        _ => "",
                    };
                    update_expression = match recipe.update_type {
                        UpdateType::Swap => format!("SET #{key} = {value_param}"), 
                        _ => {eprintln!("this is where the error is"); return Outcome::InvalidRequest},
                    };
                },
                AttributeValue::L(_) => {
                    value_param = match &key[..] {
                        ACTIVE_DECKS_DB_KEY => ":newactivedecks",
                        OWNED_DECKS_DB_KEY => ":newowneddecks",
                        COLAB_DECKS_DB_KEY => ":newcolabdecks",
                        _ => "",
                    };
                    update_expression = match recipe.update_type {
                        UpdateType::Add => format!("SET #{key} = list_append(#{key}, {value_param})"),
                        UpdateType::Subtract => format!("DELETE #{key} {value_param}"),
                        UpdateType::Swap => format!("SET #{key} = {value_param}"), 
                    };
                },
                _ => return Outcome::UpdateUserFailure("unexpected attribute value".to_owned()),
            }

            update_request = update_request.expression_attribute_names(format!("#{key}"), key);
            if value_param.is_empty() {
                eprintln!("!!!!!!SOMETING WRONG!! PARAM VALUWU EMPTY");
            } else {
                update_request = update_request.expression_attribute_values(value_param, attribute_value.clone());
            }

            if update_expression.starts_with("SET") {
                println!("set expression added");
                set_expressions.push(update_expression);
            } else if update_expression.starts_with("ADD") {
                println!("add expression added");
                add_expressions.push(update_expression);
            } else if update_expression.starts_with("REMOVE") {
                println!("remove expression added");
                remove_expressions.push(update_expression);
            } else if update_expression.starts_with("DELETE") {
                println!("delete expression added");
                delete_expressions.push(update_expression);
            } else {
                println!("expression had {}", Outcome::IncorrectType.to_string());
                return Outcome::IncorrectType;
            }

            cache_recipe_list.recipes.push(recipe);
        }

        expression_lists.push(set_expressions);
        expression_lists.push(add_expressions);
        expression_lists.push(remove_expressions);
        expression_lists.push(delete_expressions);

        let mut all_finalized_expressions = Vec::new();
        
        for expression_list in expression_lists {

            let mut staging_expression = String::new();

            for (i, update_expression) in expression_list.iter().enumerate() {
                if i == 0 {
                    staging_expression.push_str(&update_expression);
                } else {
                    let Some(end_of_first_word) = update_expression.find(" ") else {println!("could not find first word"); return Outcome::InvalidRequest};
                    let modified_update_expression = format!(",{}", &update_expression[end_of_first_word..]);
                    staging_expression.push_str(&modified_update_expression);
                }
            }
            println!("{:?}", staging_expression);
            if !!!staging_expression.is_empty() {
                all_finalized_expressions.push(staging_expression);
            }
        }

        let final_update_expression = all_finalized_expressions.join(" ");

        println!("{:?}", &final_update_expression);

        update_request = update_request.update_expression(final_update_expression);

        match update_request.send().await {
            Ok(_) => return Outcome::DatabaseUpdateSuccess(cache_recipe_list),
            Err(e) => return Outcome::UpdateUserFailure(e.into_service_error().to_string()),
        };
    }
    Outcome::UnresolvedOutcome
}

pub fn convert_attribute_value_to_string(attribute_value: &AttributeValue, list_separator: &str) -> String {
    match attribute_value {
        AttributeValue::Bool(data) => data.to_string(),
        AttributeValue::N(data) => data.to_owned(),
        AttributeValue::Ns(data) => {
            let mut list_as_str = String::new();
            for number in data {
                list_as_str.push_str(number);
                list_as_str.push_str(list_separator);
            }
            list_as_str.pop();
            list_as_str.pop();
            list_as_str.pop();
            list_as_str
        },
        AttributeValue::S(data) => data.to_owned(),
        AttributeValue::Ss(data) => {
            let mut list_as_str = String::new();
            for string in data {
                list_as_str.push_str(string);
                list_as_str.push_str(list_separator);
            }
            list_as_str.pop();
            list_as_str.pop();
            list_as_str.pop();
            list_as_str
        },
        _ => "".to_string(),
    }
}

pub fn sort_recipes_by_update_item(update_recipes: &Vec<UpdateRecipe>) -> Vec<Vec<UpdateRecipe>> {
    let mut sorted_update_recipe_sets = Vec::new();
    
    let mut unsorted_recipes = update_recipes.clone();
    loop {
        let (sorted, unsorted) = return_sorted_and_remaining_recipes(unsorted_recipes);
        sorted_update_recipe_sets.push(sorted);

        if unsorted.len() < 1 {
            break;
        } else {
            unsorted_recipes = unsorted;
        }
    }
    
    sorted_update_recipe_sets
}

fn return_sorted_and_remaining_recipes(unsorted_recipes: Vec<UpdateRecipe>) -> (Vec<UpdateRecipe>, Vec<UpdateRecipe>) {
    let mut sorted_recipes = Vec::new();
    let mut new_unsorted_recipes = Vec::new();

    let current_sorting_item = unsorted_recipes[0].update_item.clone();
    for recipe in unsorted_recipes {
        if recipe.update_item == current_sorting_item {
            sorted_recipes.push(recipe.clone());
        } else {
            new_unsorted_recipes.push(recipe);
        }
    }

    (sorted_recipes, new_unsorted_recipes)
}

pub fn convert_to_attr_val(update_value: &UpdateValues) -> Result<AttributeValue, serde_dynamo::Error> {
    match update_value {
        UpdateValues::Float64(number) => to_attribute_value(number),
        UpdateValues::String(string) => to_attribute_value(string),
        UpdateValues::DeckId(deck_id) => to_attribute_value(deck_id),
        UpdateValues::UserInfo(user) => to_attribute_value(user),
        UpdateValues::Note(note) => to_attribute_value(note),
        UpdateValues::DeckList(deck_list) => to_attribute_value(deck_list),
        UpdateValues::Unsigned64(number) => to_attribute_value(number),
    }
}
