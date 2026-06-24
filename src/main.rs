use pkg::app;

#[tokio::main]

async fn main() -> Result<(), std::io::Error> {
    app::BasicServer::run().await
}
