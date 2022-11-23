use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Credentials {
    client_id: String,
    client_secret: String,
    user_id: String,
    auth_token: String,
}

fn main() {
    println!("cargo:rerun-if-changed=credentials.toml");

    let pwd = std::env::current_dir().unwrap();

    let creds: Credentials = {
        let creds_path = pwd.join("credentials.toml");

        if !creds_path.exists() {
            panic!("credentials.toml file does not exist");
        }

        let mut creds_file = std::fs::File::open(creds_path).unwrap();

        let mut creds = String::new();

        creds_file.read_to_string(&mut creds).unwrap();

        toml::from_str(&creds).unwrap()
    };

    let v2chat_path = pwd.join("chat").join("v2");

    let creds_output = format!(
        r#"
const client_id = "{client_id}";
const credentials = "{api_token}";
"#,
        client_id = creds.client_id,
        api_token = creds.auth_token
    );

    let creds_path = v2chat_path.join("credentials.js");

    if creds_path.exists() {
        std::fs::remove_file(&creds_path).unwrap();
    }

    let mut file = std::fs::File::create(&creds_path).unwrap();
    file.write_all(creds_output.as_bytes()).unwrap();

    // Make credentionals accessible within the program
    println!("cargo:rustc-env=TWITCH_CLIENT_ID={}", creds.client_id);
    println!(
        "cargo:rustc-env=TWITCH_CLIENT_SECRET={}",
        creds.client_secret
    );
    println!("cargo:rustc-env=TWITCH_USER_ID={}", creds.user_id);
    println!("cargo:rustc-env=TWITCH_AUTH_TOKEN={}", creds.auth_token);
}
