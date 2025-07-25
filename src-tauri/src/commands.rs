use tauri::Emitter;

use crate::{
    crypto::{Certificate, CertificateInfo},
    GLOBAL_STATE,
};
use serde_json::json;

#[tauri::command]
pub async fn listen_ws_events(app: tauri::AppHandle) {
    let state = GLOBAL_STATE.clone();

    let mut is_listening = state.is_listening.lock().unwrap();

    if *is_listening {
        return;
    }

    *is_listening = true;
    drop(is_listening);

    loop {
        println!("Escuchando eventos de WS");
        let hash_cvar = &state.hash_cvar;
        let mut hash = state.hash.lock().unwrap();

        while (*hash).is_none() {
            hash = hash_cvar.wait(hash).unwrap();
        }

        let action = state.action.lock().unwrap();

        println!("Notificando la UI");

        if *action == "verify-ok" || *action == "verify-err" {
            let cert_info = state.cert_info.lock().unwrap();

            let payload = json!({
                "action": (*action).clone(),
                "hash": (*hash).clone().unwrap(),
                "cert_info": (*cert_info).clone().unwrap(),
                "success": *action == "verify-ok"
            })
            .to_string();

            *hash = None;

            app.emit("verification-info", payload).unwrap();
        } else {
            let payload = json!({
                "action": (*action).clone(),
                "hash": (*hash).clone().unwrap()
            })
            .to_string();

            app.emit("select-cert", payload).unwrap();
        }

        // Esperar a que se cargue el certificado
        while (*hash).is_some() {
            hash = hash_cvar.wait(hash).unwrap();
        }
    }
}

#[tauri::command]
pub async fn update_certificate(certificate_value: Vec<u8>) -> Option<CertificateInfo> {
    println!("Invocado comando update_certificate desde UI");

    let state = GLOBAL_STATE.clone();

    let cond_var = &state.cert_cvar;
    let mut certificate_state = state.certificate.lock().unwrap();

    let certificate = Certificate::new(&certificate_value);
    *certificate_state = Some(certificate_value);

    if certificate
        .passphrase_required()
        .expect("Couldn't process certificate")
    {
        return None;
    }

    cond_var.notify_all();

    match certificate.info() {
        Ok(info) => Some(info),
        Err(e) => {
            eprintln!("{:?}", e);
            None
        }
    }
}

#[tauri::command]
pub fn update_passphrase(passphrase_value: String) {
    println!("Invocado comando update_passphrase desde UI");

    let state = GLOBAL_STATE.clone();

    let cond_var = &state.cert_cvar;
    let mut passphrase = state.passphrase.lock().unwrap();

    *passphrase = Some(passphrase_value);

    cond_var.notify_all();
}
