#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
// #![feature(const_trait_impl)]
// #![feature(type_alias_impl_trait)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]
#![feature(variant_count)]
#![feature(strict_overflow_ops)]
#![feature(iter_intersperse)]
#![feature(trivial_bounds)]
#![feature(impl_trait_in_assoc_type)]
// #![feature(impl_trait_existential_types)]
// #![feature(option_get_or_insert_default)]
#![feature(let_chains)]

fn main() {
  let current_day = day1;
  dbg!(current_day());
}
use std::{fs::File,
          io::{self, BufRead},
          path::Path};

fn day1() -> io::Result<()> {
  // Define the relative path to your file
  let relative_path = "input/input1";

  // Open the file
  let file_path = Path::new(relative_path);
  let file = File::open(file_path)?;

  // Create a buffered reader
  let reader = io::BufReader::new(file);

  // Read lines and parse them into a vector of arrays of numbers
  let data: Vec<Vec<f64>> = reader.lines()
                                  .filter_map(Result::ok) // Skip lines that failed to read
                                  .map(|line| {
                                    let line_nums =
                                    line.split_whitespace() // Split the line by spaces
                                        .map(|num| num.parse::<i32>()?);

                                  })
                                  .collect();

  // Print the result
  // println!("{:?}", data);

  Ok(data)
}
