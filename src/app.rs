use actix_session::Session;
use actix_web::{error, http, web, Error, HttpResponse};
use serde::Deserialize;
use sqlx::SqliteConnection;
use tera::{Context, Tera};

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    text: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct SigninUser {
    email: String,
    username: String,
    password: String,
    password2: String,
}

pub async fn index(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")? {
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
    let ctx = Context::new();

    if let Some(_) = session.get::<String>("user")? {
        return Ok(redirect("/"));
    }

    let a = tmpl
        .render("login.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    //HttpResponse::Ok().body("Hello world!")
    Ok(HttpResponse::Ok().body(a))
}

pub async fn post_login(
    tmpl: web::Data<Tera>,
    form: web::Form<LoginUser>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let ctx = Context::new();
    let _ = session.insert("user", &form.text);
    // println!("{:?}", *form);
    let _ = tmpl
        .render("login.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    //HttpResponse::Ok().body("Hello world!")
    Ok(redirect("/"))
}

pub async fn signin(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(_) = session.get::<String>("user")? {
        return Ok(redirect("/"));
    }
    //let actx = Context::from_value("hata".into()).unwrap();
    let a = tmpl
        .render("siginin.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    //HttpResponse::Ok().body("Hello world!")
    Ok(HttpResponse::Ok().body(a))
}

pub async fn post_signin(
    tmpl: web::Data<Tera>,
    form: web::Form<SigninUser>,
    session: Session,
    conn: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    if &form.password != &form.password2 {
        ctx.insert("hata", "Passwordlar bir biri ile uygun deyil");
        //println!("{:?}", *form);
        let a = tmpl
            .render("siginin.html", &ctx)
            .map_err(error::ErrorInternalServerError)?;
        return Ok(HttpResponse::Ok().body(a));
    }

    let add_user = sqlx::query("insert into users (username, email, password) values ($1,$2,$3)")
        .bind(&form.username)
        .bind(&form.email)
        .bind(&form.password)
        .execute(&**conn)
        .await;

    match add_user {
        Ok(_) => {
            let _ = session.insert("user", &form.username);
            ctx.insert("ugur", "giris ugurlu");

            Ok(redirect("/"))
        }
        Err(_) => {
            ctx.insert("hata", "username databazada movcuddur");
            //println!("{:?}", *form);
             let a = tmpl
            .render("siginin.html", &ctx)
            .map_err(error::ErrorInternalServerError)?;
        return Ok(HttpResponse::Ok().body(a));
        }
    }
}

pub async fn logout(session: Session) -> Result<HttpResponse, Error> {
    session.purge();
    Ok(redirect("/login"))
}

fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, location))
        .finish()
}

