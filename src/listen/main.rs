use warp::Filter;
use std::convert::Infallible;
use bytes::{Bytes, BytesMut};
use serde_json::Value;
use web3::types::Transaction;

async fn print_request_body(body: String) -> Result<impl warp::Reply, Infallible> {
    println!("Request body: {}", body);

    // Parse the JSON request body.
    let json_body: Vec<Value> = match serde_json::from_str(&body) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to parse request body as JSON: {:?}", e);
            return Ok(warp::reply::with_status(
							"Failed to parse request body as JSON",
							warp::http::StatusCode::BAD_REQUEST,
					));
        }
    };

    // Deserialize the JSON request body into a web3::types::Transaction object.
    for transaction_value in json_body {
        if let Ok(transaction) = serde_json::from_value::<Transaction>(transaction_value) {
            println!("Parsed transaction: {:?}", transaction);
        } else {
            eprintln!("Failed to deserialize transaction");
            return Ok(warp::reply::with_status(
							"Failed to deserialize transaction",
							warp::http::StatusCode::BAD_REQUEST,
					));
        }
    }

    Ok(warp::reply::with_status(
        "Request body printed",
        warp::http::StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() {
    let post_handler = warp::post()
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::bytes())
        .map(|bytes: Bytes| bytes.to_vec())
        .and_then(|bytes: Vec<u8>| async move {
            let body = String::from_utf8_lossy(&bytes).into_owned();
            print_request_body(body).await
        });

    let routes = post_handler;

    println!("Server started at http://localhost:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
