use std::process::exit;
use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Dice variant to roll, e.g. d20, d6, d4, etc.
    #[arg(short, long, default_value = "d20")]
    variant: String,

    /// Number of times to roll
    #[arg(short, long, default_value_t = 1)]
    times: u8,
}

fn main() {
    let args = Args::parse();

    let max = match args.variant.as_str() {
        "d2" => 2,
        "d4" => 4,
        "d6" => 6,
        "d8" => 8,
        "d10" => 10,
        "d12" => 12,
        "d20" => 20,
        "d100" => 100,
        _ => 0,
    };

    if max == 0 {
        println!("Invalid dice variant");
        exit(1);
    }

    let mut rng = rand::thread_rng();
    let mut total = 0;
    let mut rolls = Vec::new();
    for _ in 0..args.times {
        let roll = rng.gen_range(1..=max);
        total += roll;
        rolls.push(roll);
    }
    for (pos, roll) in rolls.iter().enumerate() {
        println!("#{}: {}", pos + 1, roll);
    }
    println!("= {}", total);
}