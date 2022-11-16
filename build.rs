rotenv_codegen::dotenv_module!(visibility = "pub");

fn main() {
    let pwd = std::env::current_dir().unwrap();

    println!("Starting from {}", pwd.display());
}
