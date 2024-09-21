use routers::router::get_routers;
use tokio::net::TcpListener;
use tracing::info;

mod common;
mod controllers;
mod routers;

use common::log::Logger;

#[tokio::main]
async fn main() {
    Logger::init(None);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("server starting at {:?}", listener.local_addr().unwrap());
    axum::serve(listener, get_routers()).await.unwrap();
}
