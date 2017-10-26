#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(dotenv_macros)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![feature(decl_macro)]
#![recursion_limit = "128"]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate postgres;
extern crate rocket;
extern crate serde_json;
extern crate dotenv;
extern crate chrono;
extern crate regex;
extern crate spongedown;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate r2d2_postgres;
extern crate timeago;
extern crate bcrypt;

#[macro_use] mod controller;
#[macro_use] mod handler;
#[macro_use] mod model;
mod utils;

use controller::{home,api,user};

pub fn start() {
    let pool_dsl = model::db::init_pool();
    let pool_pg = model::pg::init_pool();
    rocket::ignite()
        .manage(pool_dsl)
        .manage(pool_pg)
        // .mount("/", routes![home::public,home::index])
        .mount("/user", routes![user::login_post])
        .mount("/api", routes![api::register_post])
        .catch(errors![home::not_found])
        .launch();
}
