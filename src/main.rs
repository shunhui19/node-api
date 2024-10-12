use std::sync::LazyLock;

use routers::router::get_routers;
use tokio::net::TcpListener;
use tracing::info;

mod common;
mod configs;
mod controllers;
mod middlewares;
mod routers;

use common::log::Logger;
use configs::config::Config;

static CONFIG: LazyLock<Config> = LazyLock::new(Config::init_config);

#[tokio::main]
async fn main() {
    Logger::init();

    let listener = TcpListener::bind((CONFIG.server.local_ip, CONFIG.server.local_port))
        .await
        .unwrap();
    info!("server starting at {:?}", listener.local_addr().unwrap());
    axum::serve(listener, get_routers()).await.unwrap();
}
