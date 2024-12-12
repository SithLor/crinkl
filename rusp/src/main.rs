#[derive(Debug)]
struct Item {
    name: String,
    price: Vec<i64>,
    sell: Vec<i64>,
}

fn fill_arr_random(arr: &mut Vec<f64>, length: f64) {
    //use rayon to fill array with random number
}
use std::sync::{Arc, Mutex};

use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    let price_per_item_max = 300_000_000;
    let price_per_item_min = 0;

    let total_of_item_count = 100_000_000;
    let item_in_game = 400;

    let item_count_per_item = (0..total_of_item_count)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::rngs::StdRng::from_entropy();
            rng.gen_range(0..total_of_item_count / item_in_game)
        })
        .collect::<Vec<i64>>();

    let i: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let arr: Vec<Item> = (0..item_in_game as i64)
        .into_par_iter()
        .map(|_| {
            let mut i: std::sync::MutexGuard<'_, usize> = i.lock().unwrap();
            let item: Item = Item {
                name: rand::thread_rng().gen_range(0..150).to_string(),
                price: (0..item_count_per_item[*i])
                    .map(|_| rand::thread_rng().gen_range(price_per_item_min..price_per_item_max))
                    .collect(),
                sell: (0..item_count_per_item[*i])
                    .map(|_| rand::thread_rng().gen_range(price_per_item_min..price_per_item_max))
                    .collect(),
            };
            *i += 1;
            item
        })
        .collect();

    //size clacs

    let mut size_byte = 0;
    for item in arr.iter() {
        for price in item.price.iter() {
            size_byte += std::mem::size_of_val(price);
        }
        for sell in item.sell.iter() {
            size_byte += std::mem::size_of_val(sell);
        }
        for name in item.name.as_bytes() {
            size_byte += std::mem::size_of_val(name);
        }
    }

    //cache result for 
    //eg file has
    //cache:
    //  struct Item {
    //      name: String,
    //      price: Vec<i64>,
    //      sell: Vec<i64>,
    //  }

    {
        //print all the struct to file
        let mut file = std::fs::File::create("cache").unwrap();
        for item in arr.iter() {
            //informat of struct {name: String, price: Vec<i64>, sell: Vec<i64>}
            let _t = format!("{:?}",item.name);
            let __t = format!("{:?}",item.price);
            let ___t = format!("{:?}",item.sell);
            
        }
    }

    println!("byte:{:?}", size_byte);
    println!("kb:{:?}", size_byte / 1024);
    println!("mb:{:?}", size_byte / 1024 / 1024);

    // Convert to float for gigabyte calculation
    let gb = (size_byte as f64) / 1024.0 / 1024.0 / 1024.0;
    println!("gb:{:?}", gb);
}
