use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::{Json, Value};
use std::io;

#[derive(Serialize, Deserialize)]
pub struct Message {
    contents: String,
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("webfront/dist/index.html")
}

#[get("/<file..>",rank = 9)]
pub fn public(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("webfront/dist").join(file)).ok()
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}