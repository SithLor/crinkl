#![feature(portable_simd)]

use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use rand_core::SeedableRng;
use rand_pcg::Pcg64Mcg;
use rayon::prelude::*;
use std::simd::u64x4;
use std::time::Instant;

static mut SIZE_BYTE: usize = 0;
#[derive(Debug)]
struct Item {
    name: String,
    price: Vec<i64>,
    sell: Vec<i64>,
}
fn calculate_size(arr: &Vec<Item>) -> usize {
    arr.iter()
        .map(|item| {
            item.name.len() * std::mem::size_of::<u8>()
                + item.price.len() * std::mem::size_of::<i64>()
                + item.sell.len() * std::mem::size_of::<i64>()
        })
        .sum()
}
fn _calculate_size(arr: &Vec<Item>) -> usize {
    let mut size_byte = 0;
    for item in arr.iter() {
        size_byte += item.name.len() * std::mem::size_of::<u8>();
        size_byte += item.price.len() * std::mem::size_of::<i64>();
        size_byte += item.sell.len() * std::mem::size_of::<i64>();
    }
    size_byte
}

fn _fast_pure_rust_simd(t: usize) {
    let price_per_item_max = 300_000_000u64;
    let price_per_item_min = 0u64;
    let item_in_game = 400usize;

    let arr: Vec<Item> = (0..item_in_game)
        .into_par_iter()
        .map_init(
            || Pcg64Mcg::from_entropy(),
            |rng, _| {
                let name = rng.gen_range(0..150).to_string();

                let mut price = Vec::with_capacity(t);
                let mut sell = Vec::with_capacity(t);

                let price_dist = Uniform::new(price_per_item_min, price_per_item_max);
                let sell_dist = Uniform::new(price_per_item_min, price_per_item_max);

                // SIMD lane count
                const LANES: usize = 4;
                let chunks = t / LANES;
                let remainder = t % LANES;

                // Generate price data using SIMD
                for _ in 0..chunks {
                    let values = [
                        price_dist.sample(rng),
                        price_dist.sample(rng),
                        price_dist.sample(rng),
                        price_dist.sample(rng),
                    ];
                    let simd_vals = u64x4::from_array(values);
                    price.extend_from_slice(&[
                        simd_vals[0] as i64,
                        simd_vals[1] as i64,
                        simd_vals[2] as i64,
                        simd_vals[3] as i64,
                    ]);
                }
                // Handle remainder
                for _ in 0..remainder {
                    price.push(price_dist.sample(rng) as i64);
                }

                // Generate sell data using SIMD
                for _ in 0..chunks {
                    let values = [
                        sell_dist.sample(rng),
                        sell_dist.sample(rng),
                        sell_dist.sample(rng),
                        sell_dist.sample(rng),
                    ];
                    let simd_vals = u64x4::from_array(values);
                    sell.extend_from_slice(&[
                        simd_vals[0] as i64,
                        simd_vals[1] as i64,
                        simd_vals[2] as i64,
                        simd_vals[3] as i64,
                    ]);
                }
                // Handle remainder
                for _ in 0..remainder {
                    sell.push(sell_dist.sample(rng) as i64);
                }

                Item { name, price, sell }
            },
        )
        .collect();

    let size_byte = calculate_size(&arr);
    //set static
    unsafe {
        SIZE_BYTE = size_byte;
    }
}

fn _fast_pure_rust(t: usize) {
    let price_per_item_max = 300_000_000i64;
    let price_per_item_min = 0i64;
    let item_in_game = 400usize;

    let arr: Vec<Item> = (0..item_in_game)
        .into_par_iter()
        .map_init(
            || Pcg64Mcg::from_entropy(),
            |rng: &mut rand_pcg::Mcg128Xsl64, _| {
                let name: String = rng.gen_range(0..150).to_string();

                let price_dist: Uniform<i64> = Uniform::new(price_per_item_min, price_per_item_max);
                let sell_dist: Uniform<i64> = Uniform::new(price_per_item_min, price_per_item_max);

                let price: Vec<i64> = rng.sample_iter(&price_dist).take(t).collect();
                let sell: Vec<i64> = rng.sample_iter(&sell_dist).take(t).collect();

                Item { name, price, sell }
            },
        )
        .collect();

    let size_byte = calculate_size(&arr);
    //set static
    unsafe {
        SIZE_BYTE = size_byte;
    }
}

fn _pure_rust(t: usize) {
    let price_per_item_max = 300_000_000;
    let price_per_item_min = 0;
    let item_in_game = 400;

    let arr: Vec<Item> = (0..item_in_game)
        .into_par_iter()
        .map(|_| {
            let mut r: rand::prelude::ThreadRng = rand::thread_rng();
            Item {
                name: r.gen_range(0..150).to_string(),
                price: {
                    let mut price_vec = Vec::with_capacity(t);
                    for _ in 0..t {
                        price_vec.push(r.gen_range(price_per_item_min..price_per_item_max));
                    }
                    price_vec
                },
                sell: {
                    let mut sell_vec = Vec::with_capacity(t);
                    for _ in 0..t {
                        sell_vec.push(r.gen_range(price_per_item_min..price_per_item_max));
                    }
                    sell_vec
                },
            }
        })
        .collect();

    let size_byte = calculate_size(&arr);

    unsafe {
        SIZE_BYTE = size_byte;
    }
    // std::thread::sleep(std::time::Duration::from_secs(10));
}

//bench mark stuff

fn test(size: usize) {
    let t = size;

    println!("{}:", t);
    let now: Instant = Instant::now();
    _fast_pure_rust_simd(t);
    println!("  _fast_pure_rust_simd({}):{:?}", t, now.elapsed());

    //perf timer
    let now: Instant = Instant::now();
    _pure_rust(t);
    println!("  _pure_rust({}):{:?}", t, now.elapsed());

    let now = Instant::now();
    _fast_pure_rust(t);
    println!("  _fast_pure_rust({}):{:?}", t, now.elapsed());

    //print memory usage
    unsafe {
        println!("    gb:{:?}", SIZE_BYTE as f64 / 1024.0 / 1024.0 / 1024.0);
        println!("    mb:{:?}", SIZE_BYTE as f64 / 1024.0 / 1024.0);
        println!("    kb:{:?}", SIZE_BYTE as f64 / 1024.0);
    }
    //wait for os to release memory
    std::thread::sleep(std::time::Duration::from_secs(5));
}
fn welcome() {
    // Print time and date
    println!("this what it does genrate (400*x*(0 to 300_000_000 random ))*8 bytes of data");
}
fn main() {
    welcome();

    test(500);
    test(1000);
    test(2000);
    test(4000);
    test(8000);
    test(16000);
    test(32000);
    test(64000);
    test(128000);
    test(256000);
    test(512000);
    test(1024000);
    test(1124000);
    test(1224000);
    test(1324000);
    test(1424000);
    test(1524000);
    test(1624000);
    test(1724000);
    test(1824000);
    test(1924000);
    test(2024000);
    test(2124000);
    test(2224000);
    test(2324000);
    test(2424000);
    test(2524000);

    //test(1024000);
    //test(2048000);
    //test(4096000);
    //test(8192000);

    println!("wait 30 seconds for os to release memory");
}
