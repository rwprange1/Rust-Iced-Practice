use std::fmt::format;
use std::fs::{read_dir, ReadDir, OpenOptions};
use std::path::PathBuf;
use std::process::{Command};


pub fn get_files(curr_dir: &PathBuf) -> Vec<(String, bool)> {
    let mut holder: Vec<(String,bool)> = Vec::new();
    let dir: ReadDir = match read_dir(curr_dir) {
        Ok(f) => f,
        Err(_) => panic!("{}", "Error Retrieving Files"),
    };
    
    holder.push(("..".to_string(), true));

    for file in dir {
        let file = file.unwrap();
        let name = file.file_name().to_str().unwrap().to_string();
        let is_dir = file.file_type().unwrap().is_dir();

        if is_dir || name.ends_with("mkv"){
            holder.push((name.clone(), is_dir));
        }
      

    }
    holder
}


pub fn rip_it(file: &String, parent: &PathBuf) -> bool {
    if let Ok(_) = Command::new("ffmpeg")
        .args(
            ["-i",
                format!("{}\\{}",parent.to_str().unwrap_or("temp"), file ).as_str(),
                "-y",
                format!("{}.mp3", file.replace(".mkv", "")).as_str(),
                "-loglevel",
                "-8"
            ])
        .status(){
        return true;
    };
    false
}
