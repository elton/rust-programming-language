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

// you can put any kind of data inside an enum variant: strings, numeric types, or structs
#[derive(Debug)]
enum IpAddr {
  V4(String),
  V6(String),
}

impl IpAddr {
  fn call(&self) {
    println!("Home IP IpAddr:{:?}", self);
  }
}

pub fn ip_address() {
  let home = IpAddr::V4(String::from("127.0.0.1"));
  home.call();

  let loopback = IpAddr::V6(String::from("::1"));
  loopback.call();
}
