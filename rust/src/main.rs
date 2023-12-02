use std::env;
use std::fs;

mod day1;
// Add solution modules here

fn main() {
    let mut solutions: Vec<Box<dyn Fn(&str) -> String>> = Vec::new();
    solutions.push(Box::new(|_| { "Days of the month start with 1!".to_string() }));
    solutions.push(Box::new(day1::puzzle));
    // Add solution functions here

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_all(solutions);
    } else {
        let day_result: Result<usize, <usize as std::str::FromStr>::Err> = args[1].parse();
        day_result.map_or_else(|e| {
            println!("'{}' is not a valid day of the month: {}", args[1], e);
        }, |day| {
            if day > solutions.len() {
                println!("Day {} hasn't been solved yet!", day);
            } else {
                let input = load_data(day);
                solutions[day](&input);
            }
        })
    }
}

fn run_all(solutions: Vec<Box<dyn Fn(&str) -> String>>) {
    println!("No day specified, running all all days...\n");
    let mut solution_iter = solutions.iter();
    solution_iter.next(); // Skip day 0
    for (index, solution) in solution_iter.enumerate() {
        println!("Running day {} solution...", index + 1);
        let input = load_data(index + 1);
        let answer = solution(&input);
        println!("Day {} answer: {}", index + 1, answer);
        println!("");
    }
}

fn load_data(day: usize) -> String {
    let filename = format!("../data/day-{}-input.txt", day);
    println!("Loading input from '{}'", filename);
    let input = fs::read_to_string(filename).unwrap();
    input
}
