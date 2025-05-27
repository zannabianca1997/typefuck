mod display;
mod io;
mod memory;
mod numbers;
mod program;
mod state;

use display::Display as _;
use program::Run;

type Program = program!( , > , [ - < + > ] < . );
type Input = input!( 13 4 );

type Output = Run<Program, Input>;

fn main() {
    println!("Result: {}", Output::display());
}
