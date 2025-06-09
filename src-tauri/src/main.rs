// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::runtime::Runtime;
use yoterfirmo_lib::ws::start_websocket_server;

fn main() {
    let runtime = Runtime::new().expect("No se pudo crear el runtime de tokio");

    runtime.spawn(async move {
        if let Err(e) = start_websocket_server(9999).await {
            eprintln!("Error en el servidor WS: {}", e);
        }
    });

    yoterfirmo_lib::run();
}
