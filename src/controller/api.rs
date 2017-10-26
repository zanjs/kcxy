use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json, Value};
use rocket::request::{self,Form, FlashMessage,FromRequest,Request};
use rocket::response::{Redirect,Flash};
use model::db::ConnDsl;
use bcrypt::{DEFAULT_COST, hash, verify};
use model::user::{User,NewUser};
use chrono::prelude::*;
use chrono::{DateTime,Utc};

#[derive(Debug, Serialize, Deserialize)]
struct UserRegister {
    email: String,
    username: String,
    password: String,
}

#[post("/user/register", format = "application/json", data = "<post_user>")]
fn register_post(conn_dsl: ConnDsl, post_user: Json< UserRegister>) -> Redirect {
    use utils::schema::users;

    let hash_password = match hash(&post_user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!()
    };
    let new_user = NewUser {
        email: &post_user.email,
        username: &post_user.username,
        password: &hash_password,
        created_at: Utc::now(),
    };
    diesel::insert(&new_user).into(users::table).execute(&*conn_dsl).expect("User is  Exist!");
    Redirect::to("/")
}
