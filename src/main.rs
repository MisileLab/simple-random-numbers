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
  let minimum = app.get_one::<i128>("min").expect("No minimum value");
  let maximum = app.get_one::<i128>("max").expect("No maximum value");

  if minimum >= maximum {
    panic!("max number must be greater than min")
  } else {
    let mut rng = rand::thread_rng();
    let number: i128 = rng.gen_range(*minimum..*maximum);
    println!("number is {:?}", number);
  }
}
