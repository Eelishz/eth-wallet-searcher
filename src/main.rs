use clap::Parser;
use clap_derive::Parser;
use std::sync::{Arc, Mutex};
use std::thread;
use wallet_gen::coin::Coin;
use wallet_gen::ethereum::new_wallet;
use wallet_gen::wallet::Wallet;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Match address start
    #[arg(long)]
    start: Option<String>,

    /// Match any position
    #[arg(long)]
    fill: Option<char>,

    /// Match wallet end
    #[arg(long)]
    end: Option<String>,

    /// How many threads to use
    #[arg(long)]
    threads: Option<u16>,
}

fn main() {
    let cli = Cli::parse();

    let start = match cli.start {
        Some(s) => s,
        None => "".to_string(),
    };

    let fill = match cli.fill {
        Some(s) => s,
        None => ' ',
    };

    let end = match cli.end {
        Some(s) => s,
        None => "".to_string(),
    };

    let threads = match cli.threads {
        Some(t) => t,
        None => 1,
    };

    let best_wallet = Arc::new(Mutex::new((new_wallet(Coin::Ethereum).unwrap(), 0, 0, 0)));
    let mut handles = vec![];

    for _ in 0..threads {
        let best_wallet_clone = Arc::clone(&best_wallet);
        let start = start.clone();
        let end = end.clone();

        handles.push(thread::spawn(move || {
            search_worker(&start, fill, &end, best_wallet_clone)
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn search_worker(
    start: &String,
    fill: char,
    end: &String,
    best_wallet: Arc<Mutex<(Wallet, u8, u8, u8)>>,
) {
    loop {
        let wallet = new_wallet(Coin::Ethereum).unwrap();
        let wallet_addr = wallet.address.clone();

        let mut start_matches = 0;
        let mut end_matches = 0;
        let mut fill_matches = 0;

        for (a, b) in start.chars().zip(wallet_addr.chars()) {
            if a == b {
                start_matches += 1;
            } else {
                break;
            }
        }

        for (a, b) in end.chars().rev().zip(wallet_addr.chars().rev()) {
            if a == b {
                end_matches += 1;
            } else {
                break;
            }
        }

        for c in wallet_addr.chars() {
            if c == fill {
                fill_matches += 1;
            }
        }

        {
            let mut guard = best_wallet.lock().unwrap();
            if start_matches > guard.1
                || (start_matches == guard.1 && fill_matches == guard.2 && end_matches > guard.3)
                || (start_matches == guard.1 && end_matches == guard.3 && fill_matches > guard.2)
            {
                guard.0 = wallet.clone();
                guard.1 = start_matches;
                guard.2 = fill_matches;
                guard.3 = end_matches;
                println!("{wallet:?}");
            }
        }
    }
}
