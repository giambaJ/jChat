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
}
