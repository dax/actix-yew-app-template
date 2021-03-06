use actix_files as fs;
use actix_web::{dev::Server, http, middleware, web, App, HttpServer};
use configuration::Settings;
use core::time::Duration;
use std::net::TcpListener;
use tracing::info;
use tracing_actix_web::TracingLogger;

pub mod configuration;
pub mod {{crate_name}};
pub mod observability;
pub mod routes;

pub fn run(listener: TcpListener, settings: &Settings) -> Result<Server, std::io::Error> {
    let api_path = settings.application.api_path.clone();
    let front_base_url = settings.application.front_base_url.clone();
    let static_path = settings.application.static_path.clone();
    let static_dir = settings
        .application
        .static_dir
        .clone()
        .unwrap_or_else(|| ".".to_string());

    let server = HttpServer::new(move || {
        info!(
            "Mounting API on {}",
            if api_path.is_empty() { "/" } else { &api_path }
        );
        let api_scope = web::scope(&api_path)
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", front_base_url.as_bytes()))
                    .add((
                        "Access-Control-Allow-Methods",
                        "POST, GET, OPTIONS".as_bytes(),
                    ))
                    .add(("Access-Control-Allow-Headers", "content-type".as_bytes())),
            )
            .route("/hello", web::get().to(routes::hello))
            .route(
                "/TODO*",
                web::method(http::Method::OPTIONS).to(routes::option_wildcard),
            );

        let mut app = App::new()
            .wrap(TracingLogger::default())
            .wrap(middleware::Compress::default())
            .route("/ping", web::get().to(routes::ping))
            .service(api_scope);
        if let Some(path) = &static_path {
            info!(
                "Mounting static files on {}",
                if path.is_empty() { "/" } else { &path }
            );
            let static_scope = fs::Files::new(path, &static_dir)
                .use_last_modified(true)
                .index_file("index.html");
            app = app.service(static_scope);
        }
        app
    })
    .keep_alive(http::KeepAlive::Timeout(Duration::from_secs(60)))
    .shutdown_timeout(60)
    .listen(listener)?;

    Ok(server.run())
}
