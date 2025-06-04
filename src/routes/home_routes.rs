use actix_web::web;

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/home")
            .service(handlers::home_handler::hello_name)
            .service(handlers::home_handler::test),
    );
}
