use crate::logger::setup_logger;

use anyhow::{Context, Result};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{extract, Router};
use cairo::{prove, verify_proof};
use log::{debug, info};
use std::collections::HashMap;
use std::io::Read;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod cairo;
mod error;
mod logger;

use zip::ZipArchive;

use std::io::Cursor;

fn decompress_zip_data(compressed_data: Vec<u8>) -> std::io::Result<Vec<u8>> {
    let cursor = Cursor::new(compressed_data);
    let mut archive = ZipArchive::new(cursor)?;
    let mut decompressed_data = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        file.read_to_end(&mut decompressed_data)?;
    }

    Ok(decompressed_data)
}

/*
    Receives a multipart request with the following fields
        - trace -> contains a zip file of the bytes of the trace output during the run
        - memory -> contains a zip file of the bytes of the memory output during the run
    Returns a base64 encoded proof (not zipped)
*/
async fn prove_handler(mut multipart: extract::Multipart) -> String {
    info!("Received a prove request");

    let mut map: HashMap<String, Vec<u8>> = HashMap::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let to_vec: Vec<u8> = data
            .bytes()
            .into_iter()
            .collect::<Result<Vec<u8>, _>>()
            .unwrap();

        map.insert(name.clone(), decompress_zip_data(to_vec).unwrap());

        debug!("Length of `{name}` is {} bytes", data.len());
    }

    let trace_data = map.get("trace").unwrap();
    let memory_data = map.get("memory").unwrap();

    let proof = prove(trace_data, memory_data).unwrap();

    String::from(base64::encode(proof))
}

/*
    Receives a multipart request with the following fields
        - proof -> contains a zip file of the bytes of the proof output during the run
    Returns OK if proof is verified, KO otherwise
*/
async fn verify_handler(mut multipart: extract::Multipart) -> Response {
    info!("Received a verify request");

    let mut map: HashMap<String, Vec<u8>> = HashMap::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let to_vec: Vec<u8> = data
            .bytes()
            .into_iter()
            .collect::<Result<Vec<u8>, _>>()
            .unwrap();

        map.insert(name.clone(), decompress_zip_data(to_vec).unwrap());

        debug!("Length of `{name}` is {} bytes", data.len());
    }

    let proof_data = map.get("proof").unwrap();

    match verify_proof(proof_data) {
        Ok(()) => (StatusCode::OK, "OK").into_response(),
        Err(e) => {
            log::error!("{:?}", e);
            (StatusCode::OK, "KO").into_response()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger().context("Setup logger")?;

    info!("Starting Cairo Http!");

    let app = Router::new()
        .route("/prove", post(prove_handler))
        .route("/verify", post(verify_handler));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let tcp_listener = TcpListener::bind(addr).await?;

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .context("Starting http server")
}
