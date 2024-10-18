use std::sync::LazyLock;

use routers::router::get_routers;
use sqlx::PgPool;
use tokio::{net::TcpListener, sync::OnceCell};
use tracing::info;

mod common;
mod configs;
mod controllers;
mod middlewares;
mod models;
mod routers;

use common::log::Logger;
use configs::config::Config;

static CONFIG: LazyLock<Config> = LazyLock::new(Config::init_config);
static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    Logger::init(CONFIG.server.log_file_name.clone());

    let listener = TcpListener::bind((CONFIG.server.local_ip, CONFIG.server.local_port))
        .await
        .unwrap();
    info!("server starting at {:?}", listener.local_addr().unwrap());
    axum::serve(listener, get_routers()).await.unwrap();
}
