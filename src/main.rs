use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
#[macro_use]
extern crate rocket;
mod paste_id;

// In a real application, these would be retrieved dynamically from a config.
// Also, host and port are defined in Rocket.toml
// Either get those values and use them in HOST or maybe seperate config file?
const ID_LENGTH: usize = 5;
const HOST: Absolute<'static> = uri!("http://localhost:5432");

// Eventually we will want to return actual json.
#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = paste_id::PasteId::new(ID_LENGTH);
    match paste.open(128.kibibytes()).into_file(id.file_path()).await {
        Ok(_) => {
            // Not typesafe I don't think
            // also will have to change once we use a config file
            let uri = format!("{}/{}", HOST, id);
            Ok(uri)
        },
        Err(e) => Ok(format!("Error: {}", e)),
    }
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload])
}
