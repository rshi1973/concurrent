use anyhow::{anyhow, Result};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const NUM_PRODUCERS: u32 = 3;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: u32,
    value: i32,
}

fn main() -> Result<()> {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Create producer threads
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    // Create consumer thread
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consume {:?}:", msg);
        }

        println!("Consumer stopping");

        42
    });

    let secret = consumer
        .join()
        .map_err(|_| anyhow!("Consumer thread panicked"))?;

    println!("Secret: {}", secret);

    Ok(())
}

fn producer(idx: u32, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<i32>() % 100;
        tx.send(Msg::new(idx, value))?;

        // Sleep for a random time between 0 and 2000 ms
        let sleep_time = rand::random::<u64>() % 2000;
        thread::sleep(Duration::from_millis(sleep_time));

        // Random chance to stop the producer
        if rand::random::<f64>() < 0.1 {
            println!("Producer {} stopping", idx);
            return Ok(());
        }
    }
}

impl Msg {
    fn new(id: u32, value: i32) -> Msg {
        Msg { id, value }
    }
}
