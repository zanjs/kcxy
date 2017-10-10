use utils::schema::users;
use chrono::{DateTime,Utc,NaiveDateTime};

#[derive(Clone,Debug,Serialize,Identifiable,Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub created_at: DateTime<Utc>,
}
