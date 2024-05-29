use std::{env, str::FromStr};

use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions};
use tera::Tera;

mod app;
use app::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = env::var("DATABASE_URL").expect("Database tapilmadi .env faylinin icinde");
    let conn = sqlx::SqlitePool::connect(&db).await.unwrap();

    HttpServer::new(move || {
        let mut templates = Tera::new("templates/**/*").expect("errors in tera templates");
        templates.autoescape_on(vec!["tera"]);

        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .app_data(web::Data::new(templates))
            .app_data(web::Data::new(conn.clone()))
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/login")
                    .route(web::post().to(post_login))
                    .route(web::get().to(login)),
            )
            .service(
                web::resource("/signin")
                    .route(web::post().to(post_signin))
                    .route(web::get().to(signin)),
            )
            .service(web::resource("/logout").route(web::get().to(logout)))
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//5:46
