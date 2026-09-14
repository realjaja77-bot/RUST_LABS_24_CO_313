use std::fs::{self};
use std::io::{self};
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// --- 6.4 Exercise C ---
#[derive(Debug)]
enum WorkResult {
    Sum(u64),
    Error(String),
}

fn worker(id: usize, data: Vec<u64>, tx: mpsc::Sender<WorkResult>) {
    let sum: u64 = data.iter().sum();
    // TODO 3: Send WorkResult::Error if chunk sum > 30000
    if sum > 30000 {
        tx.send(WorkResult::Error(format!("Worker {} sum {} exceeded threshold 30000", id, sum))).unwrap();
    } else {
        tx.send(WorkResult::Sum(sum)).unwrap();
    }
}

// --- 6.5 Exercise D: TODO 4 ---
fn list_rs_files_recursive(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                list_rs_files_recursive(&path)?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                println!("  Found Rust file: {:?}", path);
            }
        }
    }
    Ok(())
}

// --- 6.6 Exercise E ---
async fn fetch_data(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("Data from source {}", id)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("--- 6.2 Exercise A: Thread Chunk Sum ---");
    // TODO 1: 4 threads calculating quarters of 1..=1000
    let chunks = vec![1..=250, 251..=500, 501..=750, 751..=1000];
    let mut handles = vec![];
    for range in chunks {
        handles.push(thread::spawn(move || range.fold(0u64, |acc, x| acc + x)));
    }
    let total: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("Sum of 1..=1000 across 4 threads: {}\n", total);

    println!("--- 6.3 Exercise B: Mutex Refactoring ---");
    // TODO 2: Refactored thread accumulation with single Mutex lock
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];
    let start_time = Instant::now();

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut local_sum = 0u64;
            for _ in 0..1000 {
                local_sum += 1;
            }
            let mut num = c.lock().unwrap();
            *num += local_sum;
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("Optimized final counter: {} (Elapsed: {:?})\n", *counter.lock().unwrap(), start_time.elapsed());

    println!("--- 6.4 Exercise C: Channel Error Handling ---");
    let (tx, rx) = mpsc::channel();
    let dataset: Vec<Vec<u64>> = (0..4)
        .map(|i| (i * 250 + 1..=(i + 1) * 250).collect())
        .collect();

    for (id, chunk) in dataset.into_iter().enumerate() {
        let tx_clone = tx.clone();
        thread::spawn(move || worker(id, chunk, tx_clone));
    }
    drop(tx);

    for result in rx {
        match result {
            WorkResult::Sum(s) => println!("Received Sum: {}", s),
            WorkResult::Error(err) => println!("Received Error: {}", err),
        }
    }

    println!("\n--- 6.5 Exercise D: File I/O & Recursion ---");
    println!("Listing local .rs files:");
    list_rs_files_recursive(Path::new("."))?;

    println!("\n--- 6.6 Exercise E: Tokio Join ---");
    // TODO 5: Using tokio::join! to await futures simultaneously
    let (res1, res2, res3, res4) = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
        fetch_data(4)
    );
    println!("Joined outputs:\n  {}\n  {}\n  {}\n  {}", res1, res2, res3, res4);

    Ok(())
}