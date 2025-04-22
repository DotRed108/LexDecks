
#[cfg(feature = "ssr")]
#[allow(unused_variables)]
#[tokio::main]
async fn main() {
    use axum::{middleware, Router};
    use axum_server::tls_rustls::RustlsConfig;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use lex_decks::{app::*, utils::middleware::auth_middleware};
    use std::sync::Arc;
    use tower_cookies::CookieManagerLayer;
    use tower_governor::{governor::GovernorConfig, GovernorLayer};
    
    println!("server started");
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install");
    #[cfg(debug_assertions)]
    dotenvy::dotenv().unwrap();

    let governor_conf = Arc::new(GovernorConfig::default());
    
    let governor_limiter = governor_conf.limiter().clone();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    println!("rate limiter init");
    let clean_up_governor = tokio::task::spawn(async move {
        loop {
            interval.tick().await;
            governor_limiter.retain_recent();
        }
    });

    clean_up_governor.await.unwrap();
    println!("rate limiter cleanup");

    let key: Vec<u8> = create_ssl_key().as_bytes().into();
    let cert: Vec<u8> = create_ssl_cert().as_bytes().into();

    #[cfg(not(debug_assertions))]
    let config = RustlsConfig::from_pem(cert, key).await.unwrap();

    #[cfg(debug_assertions)]
    let config = RustlsConfig::from_pem_file("./cert.pem", "./key.pem").await.expect("Could not create rustls config");

    println!("ssl cert init");
    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    println!("routes config init");
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(auth_middleware))
        .layer(GovernorLayer {config: governor_conf})
        .with_state(leptos_options);
    
    println!("routes init");
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let std_listener = listener.into_std().unwrap();
    println!("boutta serve");
    axum_server::from_tcp_rustls(std_listener, config).serve(app.into_make_service()).await.unwrap();
    // axum::serve(listener, app.into_make_service())
    //     .await
    //     .unwrap();
}

fn create_ssl_cert() -> String {
    let begin_cert = "-----BEGIN CERTIFICATE-----";
    let middle_cert = std::env::var("SSL_CERT").unwrap_or_default().replace("\n", "").replace(" ", "");
    let end_cert = "-----END CERTIFICATE-----";
    format!("{begin_cert}\n{middle_cert}\n{end_cert}")
}

fn create_ssl_key() -> String {
    let begin_key = "-----BEGIN PRIVATE KEY-----";
    let middle_key = std::env::var("SSL_CERT_PRIVATE_KEY").unwrap_or_default().replace("\n", "").replace(" ", "");
    let end_key = "-----END PRIVATE KEY-----";
    format!("{begin_key}\n{middle_key}\n{end_key}")
}


#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
