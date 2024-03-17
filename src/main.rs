use rocket::response::status::Custom;
use rocket::serde::json::Json;

use crate::file_search::file_search::search_in_files;
use rocket::http::Status;
use std::path::Path;
pub mod file_search;
#[macro_use]
extern crate rocket;

#[launch]
#[tokio::main]
async fn server() -> _ {
    rocket::build().mount("/", routes![index])
}
const DIR: &'static str = "examples";
#[get("/search/files/<word>")]
async fn index(word: &str) -> Result<Json<Vec<String>>, Custom<String>> {
    let arr = search_in_files(
        Path::new(&format!(
            "{}/{}",
            std::env::current_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            DIR
        )),
        word,
    )
    .await
    .map_err(|e| Custom(Status::ServiceUnavailable, e.to_string()))?;

    Ok(Json(arr))
}
