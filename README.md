# OS Thread-based Web Scraper
This implementation uses native OS threads to handle HTTP requests, employing reqwest::blocking for blocking I/O. It distributes tasks across multiple threads for parallel processing.

## How It Works
- The run_with_threads function spawns OS threads to process chunks of requests.
- Each thread makes blocking HTTP requests and sends the results via an mpsc channel.
- The main function waits for all threads to complete and collects results.

Change the thread count in codebase to change the total fetch time, more threads lesser the time 
