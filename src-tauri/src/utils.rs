use crate::{crypto::Certificate, GLOBAL_STATE};
use std::error::Error;

pub fn choose_certificate(hash: String) -> Result<Certificate, Box<dyn Error>> {
    println!("Establecer el hash");
    let state = GLOBAL_STATE.clone();
    let hash_cvar = &state.hash_cvar;
    let mut hash_ref = state.hash.lock().unwrap();
    *hash_ref = Some(hash);
    hash_cvar.notify_all();

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

    let mut passphrase = state.passphrase.lock().unwrap();
    let passphrase_copy = passphrase.clone().unwrap();
    *passphrase = None;

    let mut certificate = Certificate::new(&certificate_copy);
    certificate.unlock(passphrase_copy).unwrap();

    Ok(certificate)
}
