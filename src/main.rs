use std::{fs, sync::mpsc, thread};

mod args;
use args::Args;

mod qr;

fn main() {
    let Args { path } = args::Args::get_args();

    let (tx, rx) = mpsc::channel();

    for entry in fs::read_dir(path).unwrap() {
        let tx = tx.clone();

        thread::spawn(move || {
            let content = qr::extract(entry.unwrap().path());
            tx.send(content).unwrap();
        });
    }

    // Drop the last sender to stop `rx` waiting for message.
    // The program will not complete if we comment this out.
    // **All** `tx` needs to be dropped for `rx` to have `Err`.
    drop(tx);

    let mut contents = Vec::new();
    for content in rx {
        contents.extend_from_slice(&content);
    }

    println!("{contents:#?}");
}
