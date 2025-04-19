use super::database_types::DeckList;
use super::date_and_time::current_time_in_seconds;
use super::outcomes::Outcome;
use super::queries::ValidQueryTypes;
use super::shared_truth::{MAX_LEVELS, S3_CREATION_DATE_URL_PARAM, S3_EXPIRATION_URL_PARAM};
use leptos::logging::debug_warn;
use web_sys::{self, window, Element, HtmlImageElement};

pub fn clear_element_classes_and_add_new(element: Element, class: String) {
    let classes = element.class_name();
    if !classes.is_empty() {
        let classes_as_strs = classes.split(" ");
        for class_str in classes_as_strs {
            let _ = element.class_list().remove_1(class_str);
        }
    }
    let _ = element.class_list().add_1(&class);
}

pub fn expiration_date_from_s3_url(url: &str) -> Option<u64> {
    /************* S3 URL EXAMPLE ***************/
    // https://lexpfpbucket.s3.us-east-2.amazonaws.com/default1.avif
    // ?x-id=GetObject&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=
    // AKIAZI2LJCXORLLPSQWQ%2F20240509%2Fus-east-2%2Fs3%2Faws4_request&
    // X-Amz-Date=20240509T053932Z&X-Amz-Expires=20&X-Amz-SignedHeaders=host&
    // X-Amz-Signature=8b62b651a8600f7c4fa3568523e490e1016fb03e11ea42068483e3a9d66be4db
    /************ END EXAMPLE ******************/
    let Some(creation_date_index) = url.find(S3_CREATION_DATE_URL_PARAM) else {
        debug_warn!("no creation date found in url");
        return None;
    };
    let (_, midget_url) = url.split_at(creation_date_index + S3_CREATION_DATE_URL_PARAM.len());

    let Some(end_of_creation_date_index) = midget_url.find('&') else {
        debug_warn!("could not find end of creation date in url");
        return None;
    };
    let (creation_date, _) = midget_url.split_at(end_of_creation_date_index);

    let Some(expiration_index) = url.find(S3_EXPIRATION_URL_PARAM) else {
        debug_warn!("no expiration found in url");
        return None;
    };
    let (_, midget_url) = url.split_at(expiration_index + S3_EXPIRATION_URL_PARAM.len());

    let Some(end_of_expiration_index) = midget_url.find('&') else {
        debug_warn!("could not find end of expiration in url");
        return None;
    };
    let (expiration, _) = midget_url.split_at(end_of_expiration_index);

    let formatted_date = format!(
        "{}-{}-{}T{}:{}:{}Z",
        &creation_date[..4],
        &creation_date[4..6],
        &creation_date[6..8],
        &creation_date[9..11],
        &creation_date[11..13],
        &creation_date[13..15]
    );

    let creation_date = web_sys::js_sys::Date::parse(&formatted_date);

    if creation_date == 0.0 {
        debug_warn!("could not parse creation date string to javascript date object");
        return None;
    };

    let expiration: f64 = match expiration.parse() {
        Ok(num) => num,
        Err(_) => {
            debug_warn!("could not parse expiration string as number");
            return None;
        }
    };
    let expiration_date = (creation_date / 1000.0) + expiration;

    Some(expiration_date as u64) // time from unix epoch in seconds
}

pub fn s3_url_expired(url: &str) -> bool {
    let Some(expiration_date) = expiration_date_from_s3_url(url) else {
        return true;
    };

    let current_time = current_time_in_seconds();

    current_time > expiration_date
}

pub fn image_cached(url: &str) -> bool {
    let image = match HtmlImageElement::new() {
        Ok(image) => image,
        Err(_) => {
            debug_warn!("could not test url");
            return false;
        }
    };

    image.set_src(url);

    let is_cached = image.complete() || image.natural_width() > 0;

    if !!!is_cached {
        debug_warn!("image is not cached")
    }

    is_cached
}

pub fn frontend_query_validation(query: &ValidQueryTypes, valid_decks: DeckList) -> Outcome {    
    match query {
        ValidQueryTypes::NotesByLevel(deck_id, levels) => {
            debug_warn!("current deck id {}", deck_id.to_string());
            if !!!valid_decks.contains(&deck_id) {return Outcome::UserDoesNotHavePermission};

            for level in levels {
                if level > &MAX_LEVELS {return Outcome::InvalidRequest}
            }
        },
        ValidQueryTypes::NotesById(deck_id, _) => {
            if !!!valid_decks.contains(&deck_id) {return Outcome::UserDoesNotHavePermission};
        },
        ValidQueryTypes::NotesByType(deck_id, _) => {
            if !!!valid_decks.contains(&deck_id) {return Outcome::UserDoesNotHavePermission};
        }
        _ => return Outcome::InvalidRequest,
    }
    Outcome::PermissionGranted("Query Likely Valid".to_string())
}

pub fn get_cookie_value_client(cookie_name: &str) -> Option<String> {
    use leptos::web_sys::wasm_bindgen::JsCast;
    let document = window()?.document()?;
    let html_document = document.dyn_into::<leptos::web_sys::HtmlDocument>().ok()?;
    let cookies = html_document.cookie().ok()?;

    let value = cookies
        .split(';')
        .map(|c| c.trim())
        .find_map(|c| c.strip_prefix(&format!("{}=", cookie_name)))
        .map(|s| s.to_string());

    return value;
}