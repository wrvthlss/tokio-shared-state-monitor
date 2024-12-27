# Tokio Shared State Monitor

This repository contains a Rust-based application that demonstrates the use of asynchronous programming with the Tokio runtime. The application simulates two data sources producing messages, processes these messages concurrently using Tokio's `select!` macro, and maintains a shared state to track the counts of successful and error messages.

---

## Features

- **Asynchronous Concurrency**: Uses `tokio::spawn` to handle multiple tasks simultaneously.
- **Shared State Management**: Tracks success and error counts using a thread-safe `Arc<Mutex<>>` combination.
- **Channel Communication**: Simulates real-time message passing between tasks using `tokio::sync::mpsc` channels.
- **Periodic Logging**: Logs the shared state every 5 seconds to provide a snapshot of activity.
- **Docker Support**: Runs seamlessly in a Docker container for consistent deployment across environments.

---

## How It Works

### Shared State
The application maintains a `SharedState` struct with the following fields:
- `success_count`: Tracks the number of successful messages.
- `error_count`: Tracks the number of error messages.

The state is wrapped in:
- A `Mutex` to ensure mutual exclusion, allowing safe concurrent access.
- An `Arc` to enable shared ownership of the state across tasks.

### Simulated Data Sources
Two simulated data sources (`Source1` and `Source2`) send periodic messages to their respective channels:
- Messages are randomly determined to be either a "success" or an "error".
- A success occurs with a 70% probability, while an error occurs with a 30% probability.

### Tokio `select!` Macro
The `select!` macro processes messages from both sources concurrently:
- Messages are received via `tokio::sync::mpsc` channels.
- The shared state is updated based on the type of message.

### Logging
A separate task logs the current state every 5 seconds to provide visibility into the application's operation.

---

## Code Overview

### Dependencies
- **Tokio**: The asynchronous runtime for executing tasks.
- **Rand**: For generating random messages.


### Key Functions
1. **`simulate_source`**:
    - Simulates a data source that sends messages to a channel periodically.

2. **Tokio `select!` Loop**:
    - Processes messages from both sources and updates the shared state.

3. **Logging Task**:
    - Periodically logs the shared state.

---

## Running Locally

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).
- Add the `tokio` and `rand` crates to your project:
  ```bash
  cargo add tokio --features full
  cargo add rand
- Clone the repository:
  ```bash
  git clone https://github.com/your-username/tokio-shared-state-monitor.git
  cd tokio-shared-state-monitor
- Run the application
  ```bash
  cargo run
  
Observe the logs in your console, which include:
- Messages received from `Source1` and `Source2`.
- Periodic snapshots of the shared state.

---

## Running with Docker
### Build the Docker image
```bash
   docker build -t tokio-shared-state-monitor .
```
### Run the application
```bash
   docker run --rm tokio-shared-state-monitor
```

### Example Output
```bash
  [*] Received from Source1: Source1: success
  [*] Received from Source2: Source2: error encountered
  [*] Current State: SharedState { success_count: 1, error_count: 1 }
  [*] Received from Source1: Source1: success
  [*] Received from Source2: Source2: success
  [*] Current State: SharedState { success_count: 3, error_count: 1 }
```
---

## Possible Future Enhancements
- Add more data sources to simulate a larger system.
- Introduce error handling for scenarios like channel disconnection.
- Extend the shared state to track additional metrics, such as message latency.
