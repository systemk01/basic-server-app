use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, post, web};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;
mod routes;
mod utils;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    dotenv::dotenv().ok();
    env_logger::init();

    let port = *utils::constants::PORT;
    let address = utils::constants::ADDRESS.clone();
    let database_url = utils::constants::Database_URL.clone();
    println!("listening on {}:{}", address, port);

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
