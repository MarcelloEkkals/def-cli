use def_cli::defender::Api;
use std::env;
#[tokio::main(flavor="current_thread")]
async fn main() {

    let args: Vec<String> = env::args().collect();
    let app = Api::new();
    let flag_file = String::from("-f");
    let current_flag = match args.get(1) {
        None => {
            println!("Usage: command not found!\nCorrect usage: -f for files.");
            std::process::exit(1);
        },
        Some(what) => what.to_string(),
    };
    let file = match args.get(2) {
        None => {
            println!("Insert file path after the -f flags");
            std::process::exit(1);
        }
        Some(what) => what.to_string(),
    };

    if current_flag == flag_file {
        app.scan(&file).await
    } else { 
        println!("Usage: command not found!\nCorrect usage: -f for files.");
    }

}
