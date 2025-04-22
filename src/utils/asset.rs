use leptos::server;
use server_fn::ServerFnError;
use crate::utils::{
    database_types::Asset, outcomes::Outcome,
    auth_client::AuthClient,
};
use std::time::Duration;

/// Server Imports
#[cfg(feature="ssr")]
use crate::utils::{
    dynamo_utils::{setup_client, validate_active_decks_and_user_standing},
    back_utils::{PFP_BUCKET, verify_user_header},
};
#[cfg(feature="ssr")]
use aws_sdk_s3::{presigning::PresigningConfig, Client as S3Client};
#[cfg(feature="ssr")]
use aws_config::{retry::RetryConfig, BehaviorVersion, Region};

#[server(client=AuthClient)]
pub async fn asset_from_s3(asset: Asset) -> Result<Outcome, ServerFnError> {
    let Outcome::VerificationSuccess(email) = verify_user_header().await else {return Ok(Outcome::VerificationFailure)};

    let config = aws_config::defaults(BehaviorVersion::latest()).retry_config(RetryConfig::standard().with_max_attempts(15)).region(Region::new("us-east-2")).load().await;
    let client = S3Client::new(&config);

    let outcome = get_asset_url(&client, &email, asset).await;

    Ok(outcome)
}

#[cfg(feature="ssr")]
async fn get_asset_url(client: &S3Client, email: &str, asset: Asset) -> Outcome {
    let outcome = match asset {
        Asset::PFP(address) => {
            if address.bucket != PFP_BUCKET {
                return Outcome::InvalidRequest;
            }
            get_presigned_url(client, &address.bucket, &address.key, 20).await
        },
        Asset::DeckImage(address) => {
            let Some(split_index) = address.key.find("/") else {return Outcome::InvalidRequest};
            let (deck_id, _file_id) = address.key.split_at(split_index);

            let ddb_client = setup_client().await;

            match validate_active_decks_and_user_standing(&ddb_client, email, deck_id).await {
                Outcome::PermissionGranted(_) => get_presigned_url(client, &address.bucket, &address.key, 20).await,
                any_other_outcome => any_other_outcome,
            }
        },
        _ => return Outcome::InvalidRequest,
    };
    
    outcome
}

#[cfg(feature="ssr")]
async fn get_presigned_url(client: &S3Client, bucket: &str, key: &str, expires_in: u64) -> Outcome {
    let duration = Duration::from_secs(expires_in);

    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(PresigningConfig::expires_in(duration).expect("crazy"))
        .await;

    let url = match presigned_request {
        Ok(request) => request.uri().to_string(),
        Err(e) => return Outcome::PresignedUrlNotRetrieved(e.into_service_error().to_string()),
    };
    
    Outcome::PresignedUrlRetrieved(url)
}