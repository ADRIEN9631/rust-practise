use std::{env, thread};
use std::process;

use minigrep;
use minigrep::Config;
use std::sync::mpsc;

/// Main function of line keyword query of a file
/// # Examples
///
/// ```no_run
/// cargo run to prac.txt
///
/// ```
fn file_content_searcher() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error : {}", e);
        process::exit(1);
    }
}

fn main() {

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for i in 1..10 {
            tx.send(i);
            println!("Hi from thread handle, number {}", i);
        }
    });


    for received in rx {
        println!("Main thread received {}", received);
    }

}