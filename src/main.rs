// crates needed axum and tokio full , tower i.e cargo add tower-http -F fs , reqwest 
// Header Map used for auth bw diff layers 
use axum::{response::{Html, IntoResponse}, routing::get, Router, http::StatusCode};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/static", get(static_handler))
        .fallback_service(ServeDir::new("web")); // fallback is an svc that can map try and match anything that didnt match the existing 

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await 
        .unwrap();

    println!("listening on {}" listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn static_holder() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html("<h1>Hello to the World</h1>"))
}


// 2 ->> HeaderMap ->> Header Map used for auth bw diff layers
use std::time::Duration;

use axum::{http::{header, HeaderMap}, response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
   let app = Router::new().route("/", get(header_handler));

   let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
       .await 
       .unwrap();

    tokio::spawn(make_request());

    println!("listening on {}" listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


async fn make_request() {
    // pause to let the server start up 
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Making a req to the server 
    let response = reqwest::Client::new()
        .get("/http://localhost:3001/")
        .header("z-req-id", "1234")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", response); 
}

async fn header_handler(
    headers: HeaderMap
) -> Html<String> {
    if let Some(header) = headers.get("z-req-id") {
        Html(format!("z-req-id {}", header.to_str().unwrap()))
    } else {
        Html("z-req-id not found".to_string())
    }
}


// 4 ----> Selectively applying layers 
use axum::{response::Html, routing::get, Router};

#[tokio::main]

async fn main() {
    let other = Router::new().route("/other", get(handler2));
    let app = Router::new().route("/", get(handler)).merge(other);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello Wolrd</h1>")
}

async fn handler2() -> Html<&'static str> {
    Html("<h1>Hello Wolrd 2</h1>")
}

// 5 -----> Router Layers, Crates needed, tokio axum and compression i.e cargo add tower_http -F compression-full  
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tower_http::compression::CompressionLayer; 

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("war_and_peace.txt");
    Html(WAR_AND_PEACE)
}


// Mod 3 ->> Minimal , Tracing 
// tracing is the crate we needed to emit events whereas a subscriber is the crate we needed to receive and do something with them
// crates needed ---> axum tokio reqwest tracing tracing-subscriber    
// RUST_LOG=debug cargo run i.e tracing::debug!("Born to Serve Hello World!");

use axum::{response::Html, routing::get, Router};
use tracing::info; // using the tracing crate 

#[tokio::main]
async fn main() {
    // setting up default tracing 
    tracing_subscriber::fmt::init();
    info!("Starting Server"); // emitting events into the system 

    let app = Router::new().route("/", get(handler)); 

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    info!("Born to Serve Hello World!"); // gives us the info of emitted events 
    tracing::error!("Born to Serve Hello World!"); // gives the output with ERROR RED 
    Html("<h1>Hello Wolrd 2</h1>")
 }

// Mod 3.1 Logging AxumTower 
// cargo add tower_http -F trace 
// RUST_LOG=debug cargo run i.e tracing::debug!("Born to Serve Hello World!");
use axum::{response::Html, routing::get, Router};
use tracing::info;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting Server"); // emitting events into the system 

    let app = Router::new()
       .route("/", get(handler))
       .layer(TraceLayer::new_for_http()); // constructor to generate a default http tracer 

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    // info!("Born to Serve Hello World!"); // gives us the info of emitted events 
    tracing::debug!("Born to Serve Hello World!"); // gives the output with ERROR RED 
    Html("<h1>Hello Wolrd 2</h1>")
  }

// // Mod 3.2 Timing Spans 
// use axum::{body::Body, http::{request, uri, Request}, response::Html, routing::get, Router};
// use tower_http::trace::TraceLayer;
// use tracing::{info, instrument}; 
// use tracing_subscriber::fmt::format::FmtSpan;

// #[tokio::main]

// async fn main() {
//     // setting up tracing 
//     let subscriber = tracing_subscriber::fmt()
//     // using more compact abbreviated log format 
//     .compact()
//     // display src code file paths 
//     .with_file(true)
//     // display src code file nmbrs
//     .with_line_number(true)
//     // displaying the thread id on which an event is recorded on 
//     .with_thread_ids(true)
//     // dont display the events target (module path)
//     .with_target(false)
//     // include per-span timings 
//     .with_span_events(FmtSpan::CLOSE) // span represents the time it takes for an operation to take place, when span starts it starts the timer 
//     // build the subscriber 
//     .finish();

//     // setting subsriber as default 
//     tracing::subscriber::set_global_default(subscriber).unwrap(); // overwrite the existing setup 

//     info!("Starting the server");

//     let app = Router::new()
//         .route("/", get(handler))
//         .layer(
//             TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
//                 let req_id = uuid::Uuid::new_v4(); // crate -> cargo add uuid -F v4 
//                 tracing::span!(
//                     tracing::Level::INFO,
//                     "request",
//                     method = tracing::field::display(request.method()),
//                     uri = tracing::field::display(uri.display),
//                     version = tracing::field::debug(request.version),
//                     req_id = tracing::field::display(req_id)
//                 )
//             }),
//         );

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
//         .await
//         .unwrap();

//     info!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }

// #[instrument] // creates a span when handler runs 
// async fn handler() ->  Html<&'static str> {
//     info!("Serving Hello World");
//     Html("<h1>Hello worldyy</h1>")
// }