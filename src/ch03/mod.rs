use std::io;

pub fn basic_type() {
  // variables are immutable. use let to declare a variable.
  let x = 5;
  println!("The value of x is {}", x);

  // Constants can be declared in any scope.
  // Use all uppercase with underscores between words to declare a constant.
  // constant must specify a type.

  const MAX_POINT: u32 = 100_000;
  println!("The constant variable:MAX_POINT = {}", MAX_POINT);

  // Shadow
  let a = 1;
  let a = a + 1;
  let a = a * 2;
  println!("A is {}", a); // should be 4.

  let characters = "ABCD";
  let characters = characters.len();
  println!("The length of characters is {}", characters);

  // Data type

  let f = 3.0; // f64
  let i = 32i32;
  let t = true;
  let c = '🥰';
  println!("f:{},i:{},t:{},c:{}", f, i, t, c);
}

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

pub fn compound_type() {
  // Tuple
  let tup: (u32, f64, u8) = (345, 4.332, 24);
  let (x, y, z) = tup;
  println!("The tuple is ({:?})", tup);
  println!("x = {}, y = {}, z = {}", x, y, z);

  // Array
  let array = [0, 1, 2, 3];
  println!("Array[0]: {}", array[0]);
}

pub fn function() {
  // Function
  println!("The value of the function returned is {}", plus_one(5));
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

pub fn loops() {
  // Loops
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value of a array is {}", element);
  }

  for num in (1..5).rev() {
    println!("{}!", num);
  }
}
