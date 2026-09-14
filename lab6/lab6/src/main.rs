use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(500)).await;
    format!("Data task #{}", id)
}

#[tokio::main]
async fn main() {
    println!("Lab 6: Starting Async Execution...");

    let task1 = tokio::spawn(fetch_data(1));
    let task2 = tokio::spawn(fetch_data(2));

    let res1 = task1.await.unwrap();
    let res2 = task2.await.unwrap();

    println!("Fetched: {} and {}", res1, res2);
}
