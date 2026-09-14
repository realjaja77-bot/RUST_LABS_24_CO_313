use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

// Multithreading & Shared State with Arc<Mutex<T>>
fn multithreading_demo() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Counter (Threads): {}", *counter.lock().unwrap());
}

// Async task helper
async fn fetch_data(id: u32, delay_ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    format!("Data payload from task #{}", id)
}

#[tokio::main]
async fn main() {
    println!("=== 6.1 Multithreading Demo ===");
    multithreading_demo();

    println!("\n=== 6.2 Async / Await Demo ===");
    let task1 = fetch_data(1, 300);
    let task2 = fetch_data(2, 100);

    let (res1, res2) = tokio::join!(task1, task2);
    println!("Received: {}", res1);
    println!("Received: {}", res2);

    println!("\n=== 6.3 Tokio MPSC Channel Demo ===");
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("Hello asynchronously through MPSC!").await.unwrap();
    });

    if let Some(message) = rx.recv().await {
        println!("Channel received: {}", message);
    }
}