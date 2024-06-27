#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn open(path: String) {
  match open::that(path) {
    Ok(()) => println!("Opened successfully."),
    Err(err) => eprintln!("An error occurred when opening url: {}", err),
  }
}

