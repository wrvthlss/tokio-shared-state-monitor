use std::sync::Arc;
use tokio::time::{self, Duration};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[derive(Debug)]
struct SharedState {
    success_count: u64,
    error_count: u64
}

// Simulates data source by sending messages periodically.
async fn simulate_source(tx: mpsc::Sender<String>, name: &str) {
    let mut interval = time::interval(Duration::from_secs(2));

    loop {
        interval.tick().await;
        let msg = if rand::random::<f32>() < 0.3 {
            // Simulate error.
            format!("{}: error encountered", name)
        } else {
            format!("{}: success", name)
        };

        if tx.send(msg).await.is_err() {
            println!("{} channel closed", name);
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    // Create shared state wrapped in Arc and Mutex.
    let state = Arc::new(Mutex::new(SharedState {
        success_count: 0,
        error_count: 0,
    }));

    // Create channels for simulated data sources.
    let (source1_tx, mut source1_rx) = mpsc::channel::<String>(10);
    let (source2_tx, mut source2_rx) = mpsc::channel::<String>(10);

    // Spawn tasks to simulate data sources.
    tokio::spawn(simulate_source(source1_tx, "Source1"));
    tokio::spawn(simulate_source(source2_tx, "Source2"));

    // Task to monitor and log state.
    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;
            let state = state_clone.lock().await;
            println!("[*] Current State: {:?}", *state);
        }
    });

    // Use select! to process messages from multiple sources.
    loop {
        tokio::select! {
            Some(msg) = source1_rx.recv() => {
               println!("[*] Received from Source1: \t{}", msg);
                let mut state = state.lock().await;

                if msg.contains("error") {
                    state.error_count += 1;
                } else {
                    state.success_count += 1;
                }
            }
            Some(msg) = source2_rx.recv() => {
                println!("Received from Source2: \t{}", msg);
                let mut state = state.lock().await;

                if msg.contains("error") {
                    state.error_count += 1;
                } else {
                    state.success_count += 1;
                }

            }
        }
    }
}