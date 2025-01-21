use actix_web::{dev::Server, App, HttpServer};
use snafu::{ResultExt, Whatever};

use crate::{config::UserConfig, routes::test_route};


pub fn get_server(service_config: &UserConfig) -> Result<Server, Whatever>{
    let server = HttpServer::new(move || {
        App::new()
            .service(test_route)
    })
    .bind((service_config.app.host.clone(), service_config.app.port))
    .whatever_context("Failed to bind to port")?
    .run();

    Ok(server)
}
