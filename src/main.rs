use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    middleware::Logger, web, App, HttpServer,
};
use dotenv::dotenv;
use tera::Tera;


mod app;
use app::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let mut templates = Tera::new("templates/**/*").expect("errors in tera templates");
        templates.autoescape_on(vec!["tera"]);

        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0;32]).secure(false))
            .app_data(web::Data::new(templates))
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/login")
                    .route(web::post().to(post_login))
                    .route(web::get().to(login)),
            )
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
