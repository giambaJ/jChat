use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
