use rand::{self, Rng};
use clap::Parser;

/// Simple program to generate random numbers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Minimum number
    #[clap(long)]
    min: i128,

    /// Max number
    #[clap(long)]
    max: i128
}

fn main() {
  let args = Args::parse();

  let minimum = args.min;
  let maximum = args.max;

  if minimum >= maximum {
    panic!("max number must be greater than min")
  } else {
    let mut rng = rand::thread_rng();
    let number: i128 = rng.gen_range(minimum..maximum);
    println!("number is {:?}", number);
  }
}
