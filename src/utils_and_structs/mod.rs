#![allow(dead_code)]

pub mod database_types;
pub mod outcomes;
pub mod queries;
pub mod shared_truth;
pub mod sign_in_lib;
pub mod user_types;
pub mod ui;
pub mod date_and_time;
pub mod shared_utilities;
#[cfg(feature = "ssr")]
pub mod dynamo_utils;
#[cfg(feature = "ssr")]
pub mod back_utils;
#[cfg(feature = "ssr")]
pub mod middleware;
#[cfg(feature = "ssr")]
pub mod email_template;
#[cfg(feature = "hydrate")]
pub mod front_utils;

pub fn proceed() {
    ()
}