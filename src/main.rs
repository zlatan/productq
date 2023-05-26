mod model;
mod schema;
mod handler;

use std::io;

use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use actix_web::middleware::Logger;

pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("PGSQL_URL").expect("PGSQL_URL is mandatory configuration parameter");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);


    HttpServer::new(move || {
        App::new()
            .wrap(handler::error_handlers())
            .wrap(Logger::default())
            .app_data(handlebars_ref.clone())
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(handler::config)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

