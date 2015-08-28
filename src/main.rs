use std::fs::*;
use std::path::*;
use std::os::unix::fs::MetadataExt;
use std::io::{Result};
use std::process::*;

mod parse;
use parse::*;

fn main() {
    let recent_file = most_recent_file("/home/jonathan/.freeciv/saves/");
    let output = Command::new("bzip2")
                         .arg("-d")     // decompress
                         .arg("-c")     // to stdout
                         .arg(recent_file.unwrap())
                         .output().unwrap();
    let contents = String::from_utf8(output.stdout).unwrap();
}

fn most_recent_file<P: AsRef<Path>>(dir: P) -> Option<PathBuf> {
    fn file_create_time(file : &DirEntry) -> Option<i64> {
        file.metadata().ok().map(|metadata| metadata.ctime())
    }
    fn option_less_than(mnum1 : Option<i64>, mnum2 : Option<i64>) -> bool {
        mnum1.and_then(|num1| mnum2.map(|num2| num1 < num2)).unwrap_or(false)
    }
    read_dir(dir).ok().and_then(|dir_contents| dir_contents.fold(None, |option_recent_file, entry|
        if let Some(file) = entry.ok() {
            if let Some(recent_file) = option_recent_file {
                if option_less_than(file_create_time(&recent_file),
                                    file_create_time(&file)) {
                    Some(file)
                }
                else {
                    Some(recent_file)
                }
            }
            else {
                Some(file)
            }
        }
        else {
            option_recent_file
        }).map(|dir_entry| dir_entry.path()))
}
