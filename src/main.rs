use std::env;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn client_speak_fifo(command: &str, id: Option<&str>) {
    match id {
        Some(id) => println!("Command received: {} with id {}", command, id),
        None => println!("Command received: {}", command),
    }
}

fn fifo_thread(lock: Arc<Mutex<()>>) {
    loop {
        let guard = lock.lock().unwrap();
        println!("fifo_thread");
        drop(guard);
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn clip_thread(lock: Arc<Mutex<()>>) {
    loop {
        let guard = lock.lock().unwrap();
        println!("clip_thread");
        drop(guard);
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn launch_daemon() {
    println!("Starting daemon...");

    // Create a shared mutex lock
    let lock = Arc::new(Mutex::new(()));

    // Spawn two threads that share the mutex lock
    let fifo_lock_clone = Arc::clone(&lock);
    thread::spawn(move || fifo_thread(fifo_lock_clone));
    let clip_lock_clone = Arc::clone(&lock);
    thread::spawn(move || clip_thread(clip_lock_clone));

    // Keep the daemon running indefinitely
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide an argument");
        return;
    }
    match args[1].as_str() {
        "print" => client_speak_fifo("print", None),
        "info" => {
            if args.len() < 3 {
                println!("Please provide an id for the info command");
                return;
            }
            let id = &args[2];
            client_speak_fifo("info", Some(id));
        },
        "copy" => {
            if args.len() < 3 {
                println!("Please provide an id for the copy command");
                return;
            }
            let id = &args[2];
            client_speak_fifo("copy", Some(id));
        },
        "delete" => {
            if args.len() < 3 {
                println!("Please provide an id for the delete command");
                return;
            }
            let id = &args[2];
            client_speak_fifo("delete", Some(id));
        },
        "save" => client_speak_fifo("save", None),
        "daemon" => {
            launch_daemon();
            process::exit(0);
        },
        "help" => println!("Help selected"),
        _ => println!("Invalid argument"),
    }
}
