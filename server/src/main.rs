// Lanch servers

pub mod common;
pub mod central;
pub mod local;
pub mod matchmaking;

#[tokio::main]
async fn main() {
    //Get the server type from the args passed in (default local server)
    let server_type = std::env::args().nth(1).unwrap_or_else(|| "local".to_string());

    match server_type.as_str() {
        "local" => println!("Starting local server"),
        "central" => println!("Starting central server"),
        "matchmaking" => {
            println!("Starting matchmaking server");
            matchmaking::run().await
        },
        _ => println!("Invalid server type. Use: local, central, matchmaking")
    }
}