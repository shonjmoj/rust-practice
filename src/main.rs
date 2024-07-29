use std::{
    env::args,
    fs::File,
    io::{Read, Write},
};

fn main() {
    let args: Vec<String> = args().collect();
    let mut file_name = args[1].clone();
    let string = &args[2];
    let new_string = &args[3];

    match File::open(file_name.clone()) {
        Ok(mut file) => {
            file_name.push_str("_replace");
            let _ = match File::create(file_name) {
                Ok(mut new_file) => {
                    let mut buff = String::new();
                    let _ = file.read_to_string(&mut buff);
                    let new_buff = buff.replace(string, new_string);
                    let _ = new_file.write(new_buff.as_bytes());
                }
                Err(_) => panic!("Error creating a new file"),
            };
        }
        Err(_) => panic!("Error openening {} file", file_name),
    };
}
