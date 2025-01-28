// // Building on the last exercise, we want all of the threads to complete their
// // work. But this time, the spawned threads need to be in charge of updating a
// // shared value: `JobStatus.jobs_done`

// use std::{sync::Arc, thread, time::Duration};

// struct JobStatus {
//     jobs_done: u32,
// }

// fn main() {
//     // TODO: `Arc` isn't enough if you want a **mutable** shared state.
//     let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

//     let mut handles = Vec::new();
//     for _ in 0..10 {
//         let status_shared = Arc::clone(&status);
//         let handle = thread::spawn(move || {
//             thread::sleep(Duration::from_millis(250));

//             // TODO: You must take an action before you update a shared value.
//             status_shared.lock().unwrap().jobs_done += 1;
//         });
//         handles.push(handle);
//     }

//     // Waiting for all jobs to complete.
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // TODO: Print the value of `JobStatus.jobs_done`.
//     println!("Jobs done: {}", status.lock().unwrap().jobs_done);
// }
use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // Use `Arc<Mutex<T>>` to share mutable state safely across threads.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Lock the mutex to safely update the shared value.
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // Safely access the shared value and print the result.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
