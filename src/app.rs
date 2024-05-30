use actix_session::Session;
use actix_web::{error, http, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email)]
    email: String,
    password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct SigninUser {
    #[validate(email)]
    email: String,
    username: String,
    #[validate(must_match = "password2")]
    password: String,
    password2: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    email: String,
    username: String,
    password: String,
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
    form: web::Form<LoginUser>,
    session: Session,
    conn: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let login_form = form.into_inner();

    if let Ok(_) = login_form.validate() {
        let user: User = sqlx::query_as("select * from users where email = $1")
            .bind(&login_form.email)
            .fetch_one(&**conn)
            .await
            .expect("nese loginde xeta oldu");

        if let Ok(_) = bcrypt::verify(&login_form.password, &user.password) {
            let _ = session.insert("user", &login_form.email);
        }
        return Ok(redirect("/"));
    }
    Ok(redirect("/login"))
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
    let hashed = hash(&form.password, DEFAULT_COST);
    //let valid = verify("hunter2", &hashed)?;

    match form.validate() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            ctx.insert("hata", "val err");
            //println!("{:?}", *form);
            let a = tmpl
                .render("siginin.html", &ctx)
                .map_err(error::ErrorInternalServerError)?;
            return Ok(HttpResponse::Ok().body(a));
        }
    };

    let hashed = hash(&form.password, DEFAULT_COST).unwrap();

    let add_user = sqlx::query("insert into users (username, email, password) values ($1,$2,$3)")
        .bind(&form.username)
        .bind(&form.email)
        .bind(&hashed)
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
