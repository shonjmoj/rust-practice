use crate::ProgramError;

use std::{
    fs::File,
    io::{Read, Write},
};

pub fn search_and_replace(
    file_name: &mut String,
    s1: &String,
    s2: &String,
) -> Result<(), ProgramError> {
    match File::open(file_name.clone()) {
        Ok(mut file) => {
            file_name.push_str("_replace");
            let _ = match File::create(file_name) {
                Ok(mut new_file) => {
                    let mut buff = String::new();
                    let _ = file.read_to_string(&mut buff);
                    let new_buff = buff.replace(s1, s2);
                    let _ = new_file.write(new_buff.as_bytes());
                }
                Err(_) => return Err(ProgramError::new("Error creating a new file")),
            };
        }
        Err(_) => {
            return Err(ProgramError::new(format!(
                "Error openening {} file",
                file_name
            )))
        }
    };

    Ok(())
}
