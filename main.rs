// ====================================================================
// Imports
// ====================================================================
use grep::grep;
use constructors::GrepArgsConstructor;
mod grep;
use r3bl_rs_utils::utils::{style_error, with};
//TUI (Text User Interface) framework
use std::env::args;
mod constructors;
use std::error::Error;
use std::process::exit;

fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> { 
  // returns () if successful, Box<dyn Error> if failure
  // dyn Error - trait that represents any type that can be used as an Error
  // Box heap allocates the error - makes sure it has a fixed size
  grep(GrepArgsConstructor::parse(args)?)?;
  Ok(()) // returns () if successful
}

// ====================================================================
// Main
// ====================================================================
fn main() {
  let args = args().collect::<Vec<String>>();

  // |it| - a closure on it
  // the result of run is closed in it - used to implement inline functions
  with(run(args), |it| match it {
    Ok(()) => exit(0),
    Err(err) => {
      eprintln!("{}: {}", style_error("Error Detected"), err);
      exit(1);
    }
  });
}
