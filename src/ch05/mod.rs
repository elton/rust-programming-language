// Copyright 2021 Elton Zheng
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

struct Rectangle {
  width: u32,
  height: u32,
}

// Everything within this impl block will be associated with the Rectangle type
impl Rectangle {
  // first parameter is always self
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

pub fn calc_area() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );
}