use std::fs::{read_dir, ReadDir};
use std::path::PathBuf;


pub fn get_files(curr_dir: &PathBuf) -> Vec<(String, bool)> {
    let mut holder: Vec<(String,bool)> = Vec::new();
    let dir: ReadDir = match read_dir(curr_dir) {
        Ok(f) => f,
        Err(e) => panic!("{}", "Error Retrieving Files"),
    };
    
    holder.push(("..".to_string(), true));

    for file in dir {
        let file = file.unwrap();
        let name = file.file_name().to_str().unwrap().to_string();
        let is_dir = file.file_type().unwrap().is_dir();

      
        holder.push((name.clone(), is_dir));
    }
    holder
}

