use rand::{self, Rng};
use clap::{Command, Arg};

fn main() {
  let app = Command::new("")
    .arg(Arg::new("min")
      .takes_value(true)
      .allow_hyphen_values(true)
      .help("minimum number")
      .long("min"))
    .arg(Arg::new("max")
      .takes_value(true)
      .allow_hyphen_values(true)
      .help("maximum number")
      .long("max"))
    .get_matches();
  let minimum = app.get_one::<String>("min").expect("No minimum value").parse::<i128>().unwrap();
  let maximum = app.get_one::<String>("max").expect("No maximum value").parse::<i128>().unwrap();

  if minimum >= maximum {
    panic!("max number must be greater than min")
  } else {
    let mut rng = rand::thread_rng();
    let number: i128 = rng.gen_range(minimum..maximum);
    println!("number is {:?}", number);
  }
}
