pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

trait Day {
    fn execute(&self);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <day: int>");
        return;
    }

    let day_string = &args[1];
    let day_int: i64 = match day_string.parse::<i64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: day must be an integer");
            return;
        },
    };

    if day_int < 1 || day_int > 25 {
        println!("Error: day must be between 1 and 25");
        return;
    }

    let days: Vec<Box<dyn Day>> = vec![
        Box::new(crate::day01::solution::Day01 {}),
        Box::new(crate::day02::solution::Day02 {}),
        Box::new(crate::day03::solution::Day03 {}),
        Box::new(crate::day04::solution::Day04 {}),
        Box::new(crate::day05::solution::Day05 {}),
    ];

    let day = &days[(day_int - 1) as usize];
    day.execute();
}

