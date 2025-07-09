use crate::{
    crypto::{Certificate, CertificateInfo},
    GLOBAL_STATE,
};
use std::error::Error;

pub fn choose_certificate(action: &String, hash: &String) -> Result<Certificate, Box<dyn Error>> {
    println!("Establecer el hash");
    let state = GLOBAL_STATE.clone();
    let hash_cvar = &state.hash_cvar;
    let mut hash_ref = state.hash.lock().unwrap();
    let mut action_ref = state.action.lock().unwrap();
    *hash_ref = Some(hash.clone());
    *action_ref = action.clone();
    hash_cvar.notify_all();

    drop(action_ref);
    drop(hash_ref);

    let cert_cvar = &state.cert_cvar;
    let mut certificate = state.certificate.lock().unwrap();

    println!("Esperar al certificado");
    while (*certificate).is_none() {
        certificate = cert_cvar.wait(certificate).unwrap();
    }

    println!("Recibido el certificado");
    let certificate_copy = certificate.clone().unwrap();
    *certificate = None;
    cert_cvar.notify_all();

    let mut passphrase = state.passphrase.lock().unwrap();
    let passphrase_copy = passphrase.clone().unwrap();
    *passphrase = None;

    // Eliminar hash para volver a empezar
    let mut hash = state.hash.lock().unwrap();
    let hash_cvar = &state.hash_cvar;
    *hash = None;
    hash_cvar.notify_all();

    // Crear objeto certificado y desbloquearlo para acceder a la info
    let mut certificate = Certificate::new(&certificate_copy);
    certificate.unlock(passphrase_copy).unwrap();

    Ok(certificate)
}

pub fn notify_verification(cert_info: CertificateInfo, signature: String, status: bool) {
    let state = GLOBAL_STATE.clone();
    let mut action_ref = state.action.lock().unwrap();
    let mut cert_info_ref = state.cert_info.lock().unwrap();
    let mut hash_ref = state.hash.lock().unwrap();
    let hash_cvar = &state.hash_cvar;

    *hash_ref = Some(signature);
    if status {
        *action_ref = String::from("verify-ok");
    } else {
        *action_ref = String::from("verify-err");
    }
    *cert_info_ref = Some(cert_info);

    hash_cvar.notify_all();
}
