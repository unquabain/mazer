use mazer::server::Server;
use tracing_subscriber;
use tracing;

fn setup_logging() {
    let _ = tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
        .pretty()
        .finish()
    ).map_err(|_err| eprintln!("Unable to set global default subscriber"));
}

fn main() {
    setup_logging();
    Server::new("0.0.0.0:8080".to_string()).serve();
}
