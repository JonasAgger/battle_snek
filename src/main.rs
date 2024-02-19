mod logic;
#[allow(dead_code)]
mod requests;
#[allow(dead_code)]
mod responses;
mod snake;

use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tracing::{info, warn};

#[derive(Clone)]
struct AppState {
    hist: Arc<Mutex<hdrhistogram::Histogram<u64>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        hist: Arc::new(Mutex::new(hdrhistogram::Histogram::new(4).unwrap())),
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/start", post(start))
        .route("/end", post(end))
        .route("/move", post(movement))
        .with_state(state);

    // run it

    let addr = std::env::var("SNEK").unwrap_or(String::from("127.0.0.1:3000"));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn index() -> Json<responses::Info> {
    Json(responses::Info {
        apiversion: "1".to_string(),
        author: None,
        color: Some("#b7410e".to_string()),
        head: None,
        tail: None,
        version: Some("1".to_string()),
    })
}

async fn start(State(state): State<AppState>, Json(req): Json<requests::Turn>) -> StatusCode {
    warn!(?req);

    state.hist.lock().unwrap().clear();

    StatusCode::OK
}

async fn end(State(state): State<AppState>, Json(req): Json<requests::Turn>) -> StatusCode {
    warn!(?req);

    state.hist.lock().unwrap().summarize();

    StatusCode::OK
}

async fn movement(
    State(state): State<AppState>,
    Json(req): Json<requests::Turn>,
) -> Json<responses::Move> {
    info!("Calc move!");
    let start = Instant::now();
    // tokio::time::sleep(Duration::from_millis(400)).await;

    let snake_move = logic::get_move(req);
    info!("move: {:?}", snake_move.movement);

    let elapsed = start.elapsed().as_micros() as u64;
    state.hist.lock().unwrap().record(elapsed).unwrap();

    Json(snake_move)
}

trait Summary {
    fn summarize(&self);
    fn dur(&self, quantile: f64) -> Duration;
}

impl Summary for hdrhistogram::Histogram<u64> {
    fn summarize(&self) {
        let hist = self;
        info!("# of samples: {}", hist.len());
        info!("50'th percentile: {:?}", hist.dur(0.5));
        info!("75'th percentile: {:?}", hist.dur(0.75));
        info!("90'th percentile: {:?}", hist.dur(0.9));
        info!("99.9'th percentile: {:?}", hist.dur(0.999));
    }

    fn dur(&self, quantile: f64) -> Duration {
        Duration::from_micros(self.value_at_quantile(quantile))
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
