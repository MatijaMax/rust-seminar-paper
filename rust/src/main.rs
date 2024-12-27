use rand::rngs::{OsRng, StdRng};
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;

// FICTIONAL API ENDPOINT
async fn simulated_api_call(request_id: usize) -> Result<String, String> {
    let mut rng = StdRng::from_rng(OsRng).expect("Failed to create RNG");
    let delay = rng.gen_range(100..500);
    sleep(Duration::from_millis(delay)).await; // NETWORK DELAY
    
    if rng.gen_bool(0.2) {
        return Err(format!("Request {} failed", request_id)); // 20% FAIL CHANCE
    }
    
    Ok(format!("Response for request {}", request_id))
}

// REQUEST HANDLER
async fn handle_request(semaphore: Arc<Semaphore>, request_id: usize) -> String {
    let mut retry_count = 0;

    loop {
        let _permit = semaphore.acquire().await.unwrap();

        match simulated_api_call(request_id).await {
            Ok(response) => {
                println!("Request {} succeeded: {}", request_id, response);
                return response;
            }
            Err(err) => {
                retry_count += 1;
                println!(
                    "Request {} retry({}) failed: {}",
                    request_id, retry_count, err
                );
                sleep(Duration::from_secs(2u64.pow(retry_count as u32))).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let num_requests = 20;
    let semaphore = Arc::new(Semaphore::new(5)); // MAX 5

    // TASK SPAWNER
    let tasks: Vec<_> = (1..=num_requests)
        .map(|i| {
            let semaphore = semaphore.clone();
            tokio::spawn(async move { handle_request(semaphore, i).await })
        })
        .collect();

    // RESULTS
    let results: Vec<_> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|res| res.unwrap())
        .collect();

    println!("All requests completed.");
    for result in results {
        println!("{}", result);
    }
}