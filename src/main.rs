use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web::Data,
    App, HttpServer,
};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod routers;
use routers::routes;

#[macro_use]
extern crate log;

#[macro_use]
extern crate mysql;

mod controllers;
mod models;

#[derive(Clone)]
pub struct AppState {
    pub db: mysql::Pool,
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DB_ADDRESS").unwrap();
    let client_domain = env::var("CLIENT_DOMAIN").unwrap();
    let bind_address = env::var("BIND_ADDRESS").unwrap();

    let state = AppState {
        db: mysql::Pool::new(db_url).unwrap(),
    };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .register_data(Data::new(state.clone()))
            .wrap(
                Cors::new()
                    .allowed_origin(&client_domain)
                    .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(bind_address).unwrap()
    };
    server.run().unwrap();
}
