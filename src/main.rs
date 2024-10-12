use std::error::Error;

mod core;

fn main() -> Result<(), Box<dyn Error>> {
    core::app::main()
}
