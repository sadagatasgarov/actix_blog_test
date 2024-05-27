use actix_session::Session;
use actix_web::{
    error, http, web, Error, HttpResponse, Responder,
};
use serde::Deserialize;
use tera::{Context, Tera};


#[derive(Debug, Deserialize)]
pub struct LoginUser {
    text: String,
    password: String,
}

pub async fn index(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user)= session.get::<String>("user")? {
        ctx.insert("user", &user)
    }

    ctx.insert("ad", "mehmet");
    let a = tmpl
        .render("index.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    //HttpResponse::Ok().body("Hello world!")
    Ok(HttpResponse::Ok().body(a))
}

pub async fn login(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(_)= session.get::<String>("user")? {
        return Ok(redirect("/"))
    }

    // ctx.insert("ad", "mehmet");
    let a = tmpl
         .render("login.html", &ctx)
         .map_err(error::ErrorInternalServerError)?;
     //HttpResponse::Ok().body("Hello world!")
    Ok(HttpResponse::Ok().body(a))
}

pub async fn post_login(
    tmpl: web::Data<Tera>,
    form: web::Form<LoginUser>,
    session: Session
) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let _ = session.insert("user", &form.text);
    // println!("{:?}", *form);
    let a = tmpl
        .render("login.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    //HttpResponse::Ok().body("Hello world!")
    Ok(redirect("/"))
}

fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
    .append_header((http::header::LOCATION, location))
    .finish()
}