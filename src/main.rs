use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use reqwest::blocking::Client;
use std::time::Instant;

#[derive(Clone)]
struct Request {
    url: String,
    number: usize, // Number between 0-1000
}

struct ResultStruct {
    number: usize,
    size: isize, // Size of the image or -1 if an error occurs
}

// Function for OS thread-based API calls and sending results to a channel
fn run_with_threads(requests: Vec<Request>, sender: Sender<ResultStruct>) {
    let client = Client::new();
    let thread_count = 20; // Number of threads to use
    let request_chunks = requests.chunks(requests.len() / thread_count).map(|c| c.to_vec()).collect::<Vec<_>>();

    let handles: Vec<_> = request_chunks.into_iter().map(|chunk| {
        let client_clone = client.clone();
        let sender_clone = sender.clone();
        thread::spawn(move || {
            for request in chunk {
                let number = request.number;
                match client_clone.get(&request.url).send() {
                    Ok(response) => {
                        let size = match response.bytes() {
                            Ok(bytes) => bytes.len() as isize,
                            Err(_) => -1,
                        };
                        sender_clone.send(ResultStruct { number, size }).unwrap();
                    }
                    Err(_) => {
                        sender_clone.send(ResultStruct { number, size: -1 }).unwrap();
                    }
                }
            }
        })
    }).collect();

    // Join all threads
    for handle in handles {
        handle.join().unwrap();
    }
}

// Function to consume result structs from the channel and update the vector
fn consume_results(receiver: &Receiver<ResultStruct>, results: Arc<Mutex<Vec<isize>>>) {
    for result in receiver.iter() {
        let mut results_lock = results.lock().unwrap();
        results_lock[result.number] = result.size;
    }
}

// Main function to execute the thread-based method
fn main() {
    // Generate 1,000 requests
    let requests: Vec<Request> = (0..1000)
        .map(|i| Request {
            url: "https://yral.com/img/android-chrome-384x384.png".to_string(),
            number: i,
        })
        .collect();

    // Initialize a vector with 0s to store image sizes
    let thread_results = Arc::new(Mutex::new(vec![0; 1000]));

    // Create a channel for communication
    let (thread_sender, thread_receiver) = mpsc::channel();

    // Measure time for the thread-based method
    let thread_start = Instant::now();
    println!("Running thread-based method:");
    run_with_threads(requests.clone(), thread_sender);
    let thread_duration = thread_start.elapsed();
    println!("Thread-based method took: {:?}", thread_duration);

    // Consume the results and update the vector
    consume_results(&thread_receiver, Arc::clone(&thread_results));

    let thread_final_results = thread_results.lock().unwrap();
    println!("First 10 image sizes (Thread): {:?}", &thread_final_results[..10]);
}
