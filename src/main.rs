
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::path::PathBuf;
    use std::env;

    use axum::Router;
    use axum_server::tls_rustls::RustlsConfig;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use lex_decks::app::*;
    
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install");
    // let cert = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cert.pem");
    // let key = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("key.pem");

    let cert = env::var("SSL_CERT").unwrap().as_bytes().into();
    let key = env::var("SSL_CERT_PRIVATE_KEY").unwrap().as_bytes().into();

    let config = RustlsConfig::from_pem(cert, key).await.unwrap();

    // let config = RustlsConfig::from_pem_file(cert, key).await.expect("Could not create rustls config");

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let std_listener = listener.into_std().unwrap();
    axum_server::from_tcp_rustls(std_listener, config).serve(app.into_make_service()).await.unwrap();
    // axum::serve(listener, app.into_make_service())
    //     .await
    //     .unwrap();
}


#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
