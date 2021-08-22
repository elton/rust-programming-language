use std::io;

pub fn print_array() {
  let a = [1, 2, 3, 4, 5];
  println!("please enter an array index.");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let index: usize = input.trim().parse().expect("Please type a number!");
  let element = a[index];
  println!("The element at index {} is {}", index, element);
}
