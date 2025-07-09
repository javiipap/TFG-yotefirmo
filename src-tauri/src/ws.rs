use crate::{
    crypto::Certificate,
    utils::{choose_certificate, notify_verification},
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn start_websocket_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    println!("WebSocket server listening on: {}", addr);

    while let Ok((stream, peer_addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, peer_addr));
    }

    Ok(())
}

#[derive(Clone, Serialize, Deserialize)]
struct WSRequest {
    pub action: String,
    pub payload: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
struct WSResponse {
    pub certificate: Vec<u8>,
    pub public_key: Vec<u8>,
    pub subj: String,
    pub result: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
struct VerificationRequest {
    pub certificate: Vec<u8>,
    pub signature: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
struct WSVerificationResponse {
    pub verified: bool,
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    println!("Nueva conexión de: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error durante el handshake WebSocket");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = match msg {
            Ok(val) => val,
            Err(error) => {
                eprintln!("Error en la conexión con {}: {}", addr, error);
                continue;
            }
        };

        if msg.is_close() || !(msg.is_binary() || msg.is_text()) {
            break;
        }

        let data: WSRequest;
        if msg.is_binary() {
            data = serde_json::from_slice(msg.into_data().to_vec().as_slice()).unwrap();
        } else if msg.is_text() {
            data = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
        } else {
            break;
        }

        let WSRequest { action, payload } = data;

        println!("Mensaje recibido de {}", addr);
        println!("|- Acción: {}", action);

        if action == "verify" {
            let VerificationRequest {
                certificate,
                signature,
                data,
            } = serde_json::from_slice(payload.clone().as_slice()).unwrap();

            let (info, public_key) = Certificate::ReadPublic(&certificate).unwrap();

            let verified =
                match primitives::signatures::rsa_verify(public_key, data, signature.clone()) {
                    Ok(_) => true,
                    Err(e) => {
                        println!("{:?}", e);
                        false
                    }
                };

            notify_verification(info, hex::encode(signature), verified);

            println!("Enviando respuesta");
            let response = Message::text(
                serde_json::to_string(&WSVerificationResponse { verified: verified })
                    .expect("Couldn't encode response"),
            );

            if let Err(e) = write.send(response).await {
                println!("Error al enviar mensaje: {}", e);
                break;
            }

            continue;
        }

        let payload_hash = Sha256::digest(payload.clone()).to_vec();

        let certificate = choose_certificate(&action, &hex::encode(&payload_hash)).unwrap();

        let private_key = certificate
            .extract_private_key()
            .expect("Couldn't extract private key from certificate");

        let result = if action == "decrypt" {
            primitives::signatures::rsa_decrypt(private_key, payload.clone()).unwrap()
        } else if action == "sign" {
            primitives::signatures::rsa_sign(private_key, payload.clone()).unwrap()
        } else {
            panic!("Not implemented");
        };

        println!("Enviando respuesta");
        let response = Message::text(
            serde_json::to_string(&WSResponse {
                certificate: certificate.to_pem().unwrap(),
                public_key: certificate.extract_public_key().unwrap(),
                result: result.clone(),
                subj: certificate.info().unwrap().subj,
            })
            .expect("Couldn't encode response"),
        );

        if let Err(e) = write.send(response).await {
            println!("Error al enviar mensaje: {}", e);
            break;
        }
    }

    println!("Conexión cerrada: {}", addr);
}
