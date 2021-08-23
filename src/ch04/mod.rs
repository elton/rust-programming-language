// Ownership Rules
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

pub fn string_owner() {
  let mut s = "hello".to_string();
  s.push_str(", world!");
  println!("{}", s);
}
