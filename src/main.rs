mod error;
mod search_and_replace;
pub use self::error::ProgramError;
use crate::search_and_replace::search_and_replace;

use std::env::args;

fn main() -> Result<(), ProgramError> {
    let args: Vec<String> = args().collect();

    if args.len() < 4 {
        return Err(ProgramError::new("Invalid number of arguments!"));
    }

    let mut file_name = args[1].clone();
    let s1 = &args[2];
    let s2 = &args[3];

    let _ = search_and_replace(&mut file_name, s1, s2);

    Ok(())
}
