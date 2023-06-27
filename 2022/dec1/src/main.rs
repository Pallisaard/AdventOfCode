use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).expect("Can't open file");
    let filestream = BufReader::new(file);
    let lines = filestream.lines();

    let mut meals: Vec<i32> = vec![];
    let mut current_meal = 0;

    for line in lines {
        if line.as_ref().unwrap().is_empty() {
            meals.push(current_meal);
            current_meal = 0;
        } else {
            current_meal += line.unwrap().parse::<i32>().unwrap();
        }
    }

    let max_calories = meals.iter().max().unwrap();
    println!("Max calories: {}", max_calories);

    let mut heap = BinaryHeap::new();

    heap.push(Reverse(meals[0]));
    heap.push(Reverse(meals[1]));
    heap.push(Reverse(meals[2]));

    for meal in meals.iter().skip(3) {
        let Reverse(current_min) = heap.peek().unwrap();
        if meal > current_min {
            heap.pop();
            heap.push(Reverse(*meal));
        }
    }

    let top_3_meals_sum = heap.iter().map(|Reverse(x)| x).sum::<i32>();
    println!("Top 3 meals sum: {}", top_3_meals_sum);
}
