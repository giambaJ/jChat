use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};

rotenv_codegen::dotenv_module!(visibility = "pub(crate)");

// User follows reference: https://dev.twitch.tv/docs/api/reference#get-users-follows

#[actix_web::get("/twitch/{filename:.*}")]
async fn twitch(req: HttpRequest) -> Result<NamedFile> {
    let base_path = std::env::current_dir().unwrap();
    let path = {
        let query = req.match_info().query("filename");

        if query.is_empty() {
            "index.html"
        } else {
            query
        }
    };

    let qualified_path = base_path.join("chat").join(path);

    Ok(NamedFile::open(qualified_path)?)
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
