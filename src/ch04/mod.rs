// Ownership Rules
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

pub fn string_owner() {
  let mut s = "hello".to_string();
  s.push_str(", world!");
  println!("{}", s);
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

pub fn find_1st_word() {
  let s = String::from("Hello world!");
  let word = first_word(&s);

  println!("the first word is : {}", word);
}
