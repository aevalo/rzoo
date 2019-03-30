#[macro_use]
extern crate structopt;

mod librzoo;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rzoo", about = "An example of StructOpt usage.")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// Set speed
    #[structopt(short = "s", long = "speed", default_value = "42")]
    speed: f64,
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
  println!("Using librzoo v{:?}", librzoo::rzooVersion());
  let opt = Opt::from_args();
  println!("{:?}", opt);
}

// externing crate for test-only use
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
