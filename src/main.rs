use actix_files::Files;
use actix_web::{
    error, http, middleware::Logger, web, App, Error, HttpResponse, HttpServer, Responder
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
struct LoginUser {
    text: String,
    password: String,
}

async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    // let mut ctx = Context::new();

    // ctx.insert("ad", "mehmet");
    // let a = tmpl
    //     .render("index.html", &ctx)
    //     .map_err(error::ErrorInternalServerError)?;
    HttpResponse::Ok().body("Hello world!")
    // Ok(HttpResponse::Ok().body(a))
}

async fn login(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    ctx.insert("ad", "mehmet");
    let a = tmpl
        .render("login.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    //HttpResponse::Ok().body("Hello world!")
    Ok(HttpResponse::Ok().body(a))
}

async fn post_login(tmpl: web::Data<Tera>, form: web::Form<LoginUser>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    println!("{:?}", *form);
    let a = tmpl
        .render("login.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    //HttpResponse::Ok().body("Hello world!")
    Ok(HttpResponse::Found().append_header((http::header::LOCATION, "/")).finish())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let mut templates = Tera::new("templates/**/*").expect("errors in tera templates");
        templates.autoescape_on(vec!["tera"]);

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(templates))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/login").route(web::get().to(login)))
            .service(web::resource("/login").route(web::post().to(post_login)))
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
