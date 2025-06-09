use crate::utils::choose_certificate;
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
    pub subj: String,
    pub result: Vec<u8>,
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

        let data_hash = Sha256::digest(payload.clone());

        let certificate = choose_certificate(format!("{:x?}", data_hash)).unwrap();

        let private_key = certificate
            .extract_private_key()
            .expect("Couldn't extract private key from certificate");

        let result = if action == "encrypt" {
            primitives::signatures::rsa_encrypt(private_key, payload.clone()).unwrap()
        } else if action == "sign" {
            primitives::signatures::rsa_sign(private_key, payload).unwrap()
        } else {
            panic!("Not implemented");
        };

        let response = Message::text(
            serde_json::to_string(&WSResponse {
                certificate: certificate.to_pem().unwrap(),
                result,
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
