use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use wallet_gen::coin::Coin;
use wallet_gen::ethereum::new_wallet;
use wallet_gen::wallet::Wallet;

fn main() {
    let mut args = env::args();
    let end = args.nth_back(0).unwrap();
    let fill = args.nth_back(0).unwrap().chars().nth(0).unwrap();
    let start = args.nth_back(0).unwrap();

    let best_wallet = Arc::new(Mutex::new((new_wallet(Coin::Ethereum).unwrap(), 0, 0, 0)));
    let mut handles = vec![];

    for _ in 0..8 {
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
