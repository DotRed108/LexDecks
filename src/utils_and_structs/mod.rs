#![allow(dead_code)]

pub mod database_types;
pub mod outcomes;
pub mod queries;
pub mod shared_truth;
pub mod sign_in_lib;
pub mod user_types;
pub mod front_utils;
pub mod ui;
pub mod date_and_time;
#[cfg(feature = "ssr")]
pub mod dynamo_utils;
#[cfg(feature = "ssr")]
pub mod back_utils;
#[cfg(feature = "ssr")]
pub mod middleware;

pub fn proceed() {
    ()
}