use diesel;
use diesel::prelude::*;
use rocket::request::{self,Form, FlashMessage,FromRequest,Request};
use rocket::response::{Redirect,Flash};
use model::user::{User,NewUser};
use rocket::http::{Cookie, Cookies};
use std::collections::HashMap;
use rocket::outcome::IntoOutcome;
use chrono::prelude::*;
use chrono::{DateTime,Utc};
use model::db::ConnDsl;
use model::pg::ConnPg;
use diesel::pg::PgConnection;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Debug,Serialize)]
pub struct Uid {
    id: i32,
}
#[derive(FromForm,Debug)]
pub struct DataCount {
    count: i32,
}
#[derive(Debug)]
pub struct UserOr(pub String);
#[derive(Debug)]
pub struct UserId(pub i32);

impl<'a, 'r> FromRequest<'a, 'r> for UserOr {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserOr, ()> {
        request.cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserOr(id))
            .or_forward(())
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserId, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserId(id))
            .or_forward(())
    }
}

#[derive(FromForm)]
struct UserRegister {
    email: String,
    username: String,
    password: String,
    password2: String,
}

#[derive(FromForm,Debug)]
struct UserLogin {
    username: String,
    password: String,
}


#[post("/login", data = "<user_form>")]
fn login_post(conn_dsl: ConnDsl, mut cookies: Cookies, user_form: Form<UserLogin>) -> Flash<Redirect> {
    use utils::schema::users::dsl::*;
    let post_user = user_form.get();
    let user_result =  users.filter(&username.eq(&post_user.username)).load::<User>(&*conn_dsl);
    let login_user = match user_result {
        Ok(user_s) => match user_s.first() {
            Some(a_user) => Some(a_user.clone()),
            None => None,
        },
        Err(_) => None,
    };
    match login_user {
        Some(login_user) => {
            match verify(&post_user.password, &login_user.password) {
                Ok(valid) => {
                    cookies.add_private(Cookie::new("username",post_user.username.to_string() ));
                    cookies.add_private(Cookie::new("user_id",login_user.id.to_string() ));
                    Flash::success(Redirect::to("/"), "Successfully logged in")
                },
                Err(_) => Flash::error(Redirect::to("/user/login"), "Incorrect Password"),
            }
        },
        None => Flash::error(Redirect::to("/user/login"), "Incorrect Username"),
    }
}