const secound_in_year: f64 = 60.0 * 60.0 * 24.0 * 365.0;
const minute_in_year: f64 = 60.0 * 24.0 * 365.0;
const hour_in_year: f64 = 24.0 * 365.0;
const day_in_year: f64 = 365.0;

const secound_in_day: f64 = 60.0 * 60.0 * 24.0;
const minute_in_day: f64 = 60.0 * 24.0;
const hour_in_day: f64 = 24.0;

const secound_in_hour: f64 = 60.0 * 60.0;
const minute_in_hour: f64 = 60.0;

#[derive(Debug)]
struct Item {
    name: String,
    price: Vec<i64>,
    sell: Vec<i64>,
}

fn fill_arr_random(arr: &mut Vec<f64>, length: f64) {
    //use rayon to fill array with random number
    use rand::prelude::*;
    use rayon::prelude::*;
}
fn main() {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let amount = 5.0 * minute_in_year;
    let mut arr: Vec<Item> = Vec::new();
    for _ in 0..400 as i64 {
        arr.push(Item {
            name: rand::thread_rng().gen_range(0..150).to_string(),
            price: (0..52_000).map(|_| rng.gen_range(7_000_000..15_000_000)).collect(),
            sell: (0..52_000).map(|_| rng.gen_range(7_000_000..15_000_000)).collect(),
        });
    }
    //get size of the price in kb and byte for price
    let size: usize = std::mem::size_of_val(&arr);

    println!("{:?}", size);
}
