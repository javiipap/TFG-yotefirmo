use commands::{listen_ws_events, update_certificate, update_passphrase};
use std::sync::{Arc, LazyLock};
use std::sync::{Condvar, Mutex};

mod commands;
pub mod crypto;
pub mod error;
pub mod utils;
pub mod ws;

#[derive(Default)]
pub struct AppData {
    pub certificate: Mutex<Option<Vec<u8>>>,
    pub cond_var: Condvar,
}

#[derive(Default)]
pub struct GlobalState {
    pub certificate: Mutex<Option<Vec<u8>>>,
    pub cert_cvar: Condvar,
    pub hash: Mutex<Option<String>>,
    pub hash_cvar: Condvar,
    pub is_listening: Mutex<bool>,
    pub passphrase: Mutex<Option<String>>,
    pub pass_cvar: Condvar,
    pub evt_cvar: Condvar,
}

pub static GLOBAL_STATE: LazyLock<Arc<GlobalState>> =
    std::sync::LazyLock::new(|| Arc::new(GlobalState::default()));

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            update_certificate,
            listen_ws_events,
            update_passphrase,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
