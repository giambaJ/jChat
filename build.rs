use std::io::Write;

rotenv_codegen::dotenv_module!(visibility = "pub");

fn main() {
    let pwd = std::env::current_dir().unwrap();

    let v2chat_path = pwd.join("chat").join("v2");

    let creds_output = format!(
        "
        const client_id = {client_id};
        const credentials = {api_token};
        ",
        client_id = dotenv_vars::TWITCH_CLIENT_ID,
        api_token = dotenv_vars::TWITCH_AUTH_TOKEN
    );

    let creds_path = v2chat_path.join("credentials.js");

    if creds_path.exists() {
        std::fs::remove_file(&creds_path).unwrap();
    } else {
        let mut file = std::fs::File::create(&creds_path).unwrap();
        file.write_all(creds_output.as_bytes()).unwrap();
    }
}
