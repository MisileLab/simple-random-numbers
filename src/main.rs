use rand::{self, Rng};

fn main() {
  let minimum = std::env::args().nth(1).expect("no min number").parse::<i128>().expect("no number");
  let maximum = std::env::args().nth(2).expect("no max number").parse::<i128>().expect("no number");
  if minimum >= maximum {
    panic!("max number must be greater than min")
  } else {
    let mut rng = rand::thread_rng();
    let number: i128 = rng.gen_range(minimum..maximum);
    println!("number is {:?}", number);
  }
}
