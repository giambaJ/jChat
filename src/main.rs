use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{web, App, HttpServer};

    println!("Hello, world!");

    Ok(())
}
