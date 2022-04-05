use {{crate_name}}_api::configuration::Settings;
use {{crate_name}}_api::observability::{get_subscriber, init_subscriber};
use {{crate_name}}_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("Cannot load {{project-name | capitalize}} configuration");
    let subscriber = get_subscriber(&settings.application.log_directive);
    init_subscriber(subscriber);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", settings.application.port))
        .expect("Failed to bind port");
    run(listener, &settings)?.await
}
