use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};

#[actix_web::get("/twitch/{filename:.*}")]
async fn twitch(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(twitch))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
