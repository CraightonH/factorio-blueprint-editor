use hyper::service::service_fn;
use hyper_staticfile::Static;
use hyper_util::rt::TokioIo;
use std::path::Path;
use std::path::PathBuf;
use tokio::net::TcpListener;

mod setup;

#[macro_use]
extern crate lazy_static;

static FACTORIO_VERSION: &str = "2.0.73";

lazy_static! {
    static ref DATA_DIR: PathBuf = PathBuf::from("./data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); // Make .env optional

    // Use FACTORIO_BASE_DIR if set (in Docker), otherwise download to ./data
    let base_factorio_dir = std::env::var("FACTORIO_BASE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let factorio_dir_name = match std::env::consts::OS {
                "linux" => "factorio",
                "windows" => &format!("Factorio_{FACTORIO_VERSION}"),
                _ => panic!("unsupported OS"),
            };
            DATA_DIR.join(factorio_dir_name)
        });

    let output_dir = DATA_DIR.join("output");

    // Only download if FACTORIO_BASE_DIR is not set (not in Docker)
    if std::env::var("FACTORIO_BASE_DIR").is_err() {
        setup::download_factorio(&DATA_DIR, &base_factorio_dir, FACTORIO_VERSION).await?;
    }
    
    setup::extract(&output_dir, &base_factorio_dir).await?;

    let static_ = Static::new(Path::new("data/output/"));

    let listener = TcpListener::bind(std::net::SocketAddr::from(([127, 0, 0, 1], 8081))).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let static_ = static_.clone();
        tokio::spawn(async move {
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, service_fn(|req| static_.clone().serve(req)))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }
}
