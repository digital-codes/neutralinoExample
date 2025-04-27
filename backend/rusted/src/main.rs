// main.rs

// cargo run -- --local-port 8080 --gui-port 3000

/*
Key Rust Concepts for Newcomers:

    Ownership System:

    Rust's unique memory management ensures safety without garbage collection

    Arc (Atomic Reference Counting) for shared ownership

    Mutex for safe concurrent access

    Async/Await:

    Non-blocking I/O using Tokio runtime

    .await syntax for asynchronous operations

    Pattern Matching:

    Powerful match expressions for request routing

    Destructuring complex data structures

    Type Safety:

    Strong static typing catches errors at compile time

    Result and Option types enforce error handling

    Cargo Ecosystem:

    Dependency management through Cargo.toml

    Easy integration of community crates

This implementation maintains the same functionality while leveraging Rust's safety features and modern async ecosystem. The code is more concise while being equally (if not more) performant due to Rust's zero-cost abstractions.


Summary Table
Step	Command
Install Rust	`curl ...
Create new project	cargo new calendar-server
Build & run (default)	cargo run
Build & run (with args)	cargo run -- --local-port 8080 --gui-port 3000
Build only	cargo build
Run built binary	./target/debug/calendar-server ...

*/


use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    convert::Infallible
};
use clap::Parser;
use hyper::{
    Body, Request, Response, Server, Method, StatusCode,
    service::{make_service_fn, service_fn}, header::HeaderValue
};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

use serde_json::json;


// Data structures for our calendar
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    text: String,
}


// Thread-safe storage using Arc and Mutex
lazy_static! {
    static ref CALENDAR_DB: Arc<Mutex<HashMap<String, Vec<Task>>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    static ref NEXT_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(1));
}

// Command line arguments parsing
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    local_port: u16,
    #[arg(short, long, default_value_t = 3000)]
    gui_port: u16,
}

// Main async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    
    // Create server address
    let addr = ([127, 0, 0, 1], args.local_port).into();
    
    // Create service handler
    let service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    // Start server
    let server = Server::bind(&addr).serve(service);
    println!("Server running on http://localhost:{}", args.local_port);
    println!("GUI server expected on port {}", args.gui_port);

    server.await?;
    Ok(())
}

// Request handler with CORS support
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Handle CORS preflight
    if req.method() == Method::OPTIONS {
        return Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body(Body::empty())
            .unwrap());
    }

    // Route requests
    let path = req.uri().path();
    let method = req.method();
    
    let response = match (method, path) {
        (&Method::GET, "/api/calendar/month") => handle_get_month().await,
        (&Method::POST, "/api/calendar/task") => handle_post_task(req).await,
        _ if method == Method::PUT && path.starts_with("/api/calendar/task/") => 
            handle_put_task(req).await,
        _ if method == Method::DELETE && path.starts_with("/api/calendar/task/") => 
            handle_delete_task(req).await,
        _ => Ok(not_found()),
    }.unwrap_or_else(|e| server_error(e));

    // Add CORS headers to all responses
    Ok(add_cors_headers(response))
}

// Helper function to add CORS headers
fn add_cors_headers(mut response: Response<Body>) -> Response<Body> {
    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert("Access-Control-Allow-Methods", 
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"));
    headers.insert("Access-Control-Allow-Headers", 
        HeaderValue::from_static("Content-Type"));
    response
}

/* === Request Handlers === */

// Get all calendar data

async fn handle_get_month() -> Result<Response<Body>, hyper::Error> {
    let db = CALENDAR_DB.lock().unwrap();

    // Build the "days" array as in the C++ code
    let days: Vec<_> = db.iter().map(|(date, tasks)| {
        json!({
            "date": date,
            "tasks": tasks
        })
    }).collect();

    let result = json!({ "days": days });

    Ok(Response::new(Body::from(result.to_string())))
}



// Create new task
async fn handle_post_task(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    
    let mut data: HashMap<String, String> = serde_json::from_str(&body_str).unwrap();
    
    let mut db = CALENDAR_DB.lock().unwrap();
    let mut next_id = NEXT_ID.lock().unwrap();
    
    let task = Task {
        id: *next_id,
        text: data.remove("text").unwrap(),
    };
    *next_id += 1;
    
    db.entry(data.remove("date").unwrap())
        .or_default()
        .push(task.clone());
    
    Ok(Response::new(Body::from(serde_json::to_string(&task).unwrap())))
}

// Update existing task
async fn handle_put_task(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path().to_string();
    let id = path.split('/').last().unwrap().parse::<u32>().unwrap();
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    
    let data: HashMap<String, String> = serde_json::from_str(&body_str).unwrap();
    
    let mut db = CALENDAR_DB.lock().unwrap();
    let date = data.get("date").unwrap();
    
    if let Some(tasks) = db.get_mut(date) {
        for task in tasks {
            if task.id == id {
                task.text = data.get("text").unwrap().clone();
                return Ok(Response::new(Body::from("OK")));
            }
        }
    }
    
    Ok(Response::new(Body::from("Failed")))
}

// Delete task
async fn handle_delete_task(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path().to_string();
    let id = path.split('/').last().unwrap().parse::<u32>().unwrap();
    
    let mut db = CALENDAR_DB.lock().unwrap();
    
    for tasks in db.values_mut() {
        tasks.retain(|task| task.id != id);
    }
    
    Ok(Response::new(Body::from("OK")))
}

/* === Helper Functions === */

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap()
}

fn server_error(e: hyper::Error) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Server Error: {}", e)))
        .unwrap()
}
