
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{Read, Write,Seek};
use regex::Regex;
use crate::color::*;
use serde_derive::{Serialize, Deserialize};
use confy::{load, ConfyError};
use toml::{Value};
use sysinfo::*;
use std::collections::HashMap;
use crate::ascii::{ascii_storage,find_art};
use crate::utils::verify_os;


use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
 pub enum RamStorageMesurement {
    Mb,
    Mib,
    Gb,
    Gib,
}
#[derive(Debug)]
pub enum ReadAsciiError {
    KeyNotFound(String),
    ConfyError(ConfyError),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    battery:bool,
    ram_data_type: RamStorageMesurement,
   ascii_mode:String,
   ascii:HashMap<String,String>,

}

#[warn(dead_code)]
fn add_comment_to_toml(file_path: &str, comment: &str, line: usize) {
    let mut file = match OpenOptions::new().read(true).write(true).open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return;
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to read file: {}", e);
            return;
        }
    }

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut lines = lines.into_iter();
    let mut new_contents = String::new();

    for ( mut i, line) in lines.enumerate() {
        if  i == line.lines().count() {

            new_contents.push_str(&format!("# {}\n", comment));

        }
        new_contents.push_str(&format!("{}\n", line));
        println!("new contents : {}",new_contents);
        println!("line : {}",line);
        println!("i : {}",i);
        i+=1;
    }

    match file.seek(std::io::SeekFrom::Start(0)) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to seek in file: {}", e);
            return;
        }
    }
    match file.set_len(0) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to truncate file: {}", e);
            return;
        }
    }
    match file.write_all(new_contents.as_bytes()) {

        Ok(_) => println!("Comment added to {} at line {}", file_path, line+1),
        Err(e) => println!("Failed to write to file: {}", e),
    }
}

pub fn find_os(system:&System) -> String {
    let os_name = system .name().unwrap();
    return  os_name
}


pub fn load_conf() -> Result<Config, confy::ConfyError> {

    let cfg: Config = confy::load("RustyFetch", "config")?;
    Ok(cfg)
}

pub fn read_ascii() -> Result<String, ReadAsciiError> {
let cfg: Config = match confy::load("RustyFetch", "config") {
Ok(cfg) => cfg,
Err(e) => return Err(ReadAsciiError::ConfyError(e))
};
let value = cfg.ascii.get(&*cfg.ascii_mode.to_string());
if value.unwrap() == "auto"{
    print!("TODO");
}
match value {
Some(v) => Ok(v.to_string()),
None => Err(ReadAsciiError::KeyNotFound(("not working".to_string()).to_string())),
}
}


pub fn read_ram() -> RamStorageMesurement {
    let config = load_conf().unwrap();
    let ram_data_type = config.ram_data_type;
    ram_data_type
}

pub fn check_battery() ->bool{
    let config = load_conf().unwrap();
    let battery  = config.battery;
    return battery
}
pub fn check_conf_file(system:&System) {

    if find_os(&system) == "Windows"  {

    }
    else {
        let user = std::env::var("USER").expect("Failed to get current user");
        if !fs::metadata(format!("/home/{}/.config/rustyfetch/", user)).is_ok() {
            let config = Config {
                battery: false,
                ram_data_type: RamStorageMesurement::Gib,
                ascii_mode: "".to_string(),
                ascii:ascii_storage()

            };
            match confy::store("RustyFetch", "config", &config) {
                Ok(_) => println!(""),
                Err(error) => println!("error: {:?}", error),
            }
            add_comment_to_toml("/home/thomas/.config/rustyfetch/config.toml", "show battery %", 0);
            add_comment_to_toml("/home/thomas/.config/rustyfetch/config.toml", "this allow to choose between multiple data units the option are Mb,Mib,Gb,Gbi ", 6)
        }
    }

}

pub fn translate_ascii_colors(ascii: &str) -> String {
    let ascii = ascii
        .replace("{c1}", BLACK)
        .replace("{c2}", RED)
        .replace("{c3}", GREEN)
        .replace("{c4}", YELLOW)
        .replace("{c5}", BLUE)
        .replace("{c6}", MAGENTA)
        .replace("{c7}", CYAN)
        .replace("{c8}", WHITE)
        .replace("{c9}", BLACK_BRIGHT)
        .replace("{c10}", RED_BRIGHT)
        .replace("{c11}", GREEN_BRIGHT)
        .replace("{c12}", YELLOW_BRIGHT)
        .replace("{c13}", BLUE_BRIGHT)
        .replace("{c14}", MAGENTA_BRIGHT)
        .replace("{c15}", CYAN_BRIGHT)
        .replace("{c16}", WHITE_BRIGHT)
        .replace("{reset}", RESET);

    ascii
}


impl Default for Config {
    fn default() -> Self {
        Config {
            battery:false,
            ram_data_type: RamStorageMesurement::Mib,
            ascii_mode: find_art(),
            ascii:ascii_storage(),


        }
    }
}

