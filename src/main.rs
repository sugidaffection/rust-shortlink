#[cfg(any(feature = "ssr", feature = "backend"))]
use shortlink::errors::AppError;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> Result<(), AppError> {
    use actix_files::Files;
    use actix_session::{storage, SessionMiddleware};
    use actix_web::*;
    use dotenvy::dotenv;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use shortlink::app::App;
    use shortlink::database::run_db_migrations;
    use shortlink::{api, prelude::*};
    use std::env;

    dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    run_db_migrations().await?;

    let db_pool = create_pool(None);

    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let key = cookie::Key::from(
            env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set")
                .as_bytes(),
        );

        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(
                SessionMiddleware::builder(storage::CookieSessionStore::default(), key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(cookie::SameSite::Strict)
                    .cookie_secure(true)
                    .build(),
            )
            .configure(api::routes::routes_config)
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .leptos_routes_with_context(routes, {
                let db_pool = db_pool.clone();
                move || {
                    provide_context(db_pool.clone());
                }
            }, 
            {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en" data-theme="business">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)
    .map_err(|e| {
        eprintln!("Failed to bind to {}: {}", addr, e);
        AppError::BindError(e.to_string()) // Define `BindError`
    })?
    .run()
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr", feature = "backend")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::prelude::mount_to_body;
    use shortlink::app::App;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[cfg(feature = "backend")]
#[actix_web::main]
async fn main() -> Result<(), AppError> {
    use actix_session::{storage, SessionMiddleware};
    use actix_web::*;
    use dotenvy::dotenv;
    use shortlink::database::run_db_migrations;
    use shortlink::{api, prelude::*};
    use std::env;

    dotenv().map_err(AppError::EnvLoadError)?;

    run_db_migrations().await?;

    let db_pool = create_pool(None);

    let addr = "localhost:3000";

    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let key = cookie::Key::from(
            env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set")
                .as_bytes(),
        );

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(
                SessionMiddleware::builder(storage::CookieSessionStore::default(), key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(cookie::SameSite::Strict)
                    .cookie_secure(true)
                    .build(),
            )
            .configure(api::routes::routes_config)
    })
    .bind(&addr)
    .map_err(|e| {
        eprintln!("Failed to bind to {}: {}", addr, e);
        AppError::BindError(e.to_string()) // Define `BindError`
    })?
    .run()
    .await?;

    Ok(())
}