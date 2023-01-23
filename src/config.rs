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
use crate::ascii::*;
use crate::utils::verify_os;
//use crate::config::RamStorageMesurement::*;

use std::fs;
//use std::borrow::Cow;
#[derive(Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
 pub enum RamStorageMesurement {
    Mb,
    Mib,
    Gb,
    Gib,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {

    battery:bool,
    ram_data_type: RamStorageMesurement,
    ascii: String,
}


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
    let os_name = system.name().unwrap();
    return  os_name
 //   println!("{}",os_name.unwrap());
}


pub fn load_conf() -> Result<Config, confy::ConfyError> {

    let cfg: Config = confy::load("RustyFetch", "config")?;
    Ok(cfg)
}

/*pub fn save(){

    let config = Config {
        battery:true,
        ram_data_type: Gib,
        ascii: r#"
{c2}   ///
{c2}   -::-`:     {c3}.--/:-
{c2}   `-:::::..::{c3}+///+//++:+
{c2}      +-:::::-:{c3}/++++++++++//:--
{c2}       -:::::--{c3}/++++++++++++++//:
{c2}       .-:::::::{c3}//++++++++++++++//+:
{c2}       .---::::::{c3}//////++++++++++++/+
{c3}  ///:/++++++++++++++////////+++++++//::.
{c3} +/++++++++++++++++++++++++++///////++//--
{c3} ://///////////////////////////++////////:.
{c3}     /..-://////////////////++/////////////:.
{c14}     `:+{c6}oooooooooooo::+oooooooo:-{c3}/++++++:o//
{c14}    `-+{c6}oooooooooooo+  /oooooooo` -ooooos{c14}+-
{c14}    -+++{c6}ooooooooooo+  /oooooooo` -ooooooso{c14}.
{c14}    -+++{c6}ooooooooooo+--+oooooooo--/ooooooso{c14}.
{c14}  -:++++{c6}oooooooosysoooooooooooooosyyoooooo{c14}/.
{c14}  `-o+++{c6}ooooooooossooooooooooooooossoooooo{c14}+-
{c14}  `-o+++{c6}oooooooooooooooooooooooooooooooooo{c14}+-
{c14}  `-o+++{c6}oooooooooooooooooooooooooooooooooo{c14}+-
{c14}  .:+++++++{c6}ooooooooooooooooooooooooooooo{c14}++/.
{c14}    .://+++++++++++++++{c6}oo+oooooooooo{c14}+++/::.`
          {c14}///////////////////////////:
        "#.to_string(),
    };

    match confy::store("RustyFetch", "config", &config) {
        Ok(_) => println!(""),
        Err(error) => println!("error: {:?}", error),
    }
}*/
pub fn read_ascii() -> Result<String, confy::ConfyError> {
    let cfg: Config = confy::load("RustyFetch", "config")?;
    Ok(cfg.ascii)
}
pub fn read_ram() -> RamStorageMesurement {
    let config = load_conf().unwrap();
    let ram_data_type = config.ram_data_type;
    //println!("Ram data type: {:?}", ram_data_type);
    ram_data_type
}

pub fn check_battery() ->bool{
    let config = load_conf().unwrap();
    let battery  = config.battery;
    //println!("Ram data type: {:?}", ram_data_type);
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
                ascii: r#"
{c2}   ///
{c2}   -::-`:     {c3}.--/:-
{c2}   `-:::::..::{c3}+///+//++:+
{c2}      +-:::::-:{c3}/++++++++++//:--
{c2}       -:::::--{c3}/++++++++++++++//:
{c2}       .-:::::::{c3}//++++++++++++++//+:
{c2}       .---::::::{c3}//////++++++++++++/+
{c3}  ///:/++++++++++++++////////+++++++//::.
{c3} +/++++++++++++++++++++++++++///////++//--
{c3} ://///////////////////////////++////////:.
{c3}     /..-://////////////////++/////////////:.
{c14}     `:+{c6}oooooooooooo::+oooooooo:-{c3}/++++++:o//
{c14}    `-+{c6}oooooooooooo+  /oooooooo` -ooooos{c14}+-
{c14}    -+++{c6}ooooooooooo+  /oooooooo` -ooooooso{c14}.
{c14}    -+++{c6}ooooooooooo+--+oooooooo--/ooooooso{c14}.
{c14}  -:++++{c6}oooooooosysoooooooooooooosyyoooooo{c14}/.
{c14}  `-o+++{c6}ooooooooossooooooooooooooossoooooo{c14}+-
{c14}  `-o+++{c6}oooooooooooooooooooooooooooooooooo{c14}+-
{c14}  `-o+++{c6}oooooooooooooooooooooooooooooooooo{c14}+-
{c14}  .:+++++++{c6}ooooooooooooooooooooooooooooo{c14}++/.
{c14}    .://+++++++++++++++{c6}oo+oooooooooo{c14}+++/::.`
          {c14}///////////////////////////:
        "#.to_string(),
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

/*
pub fn write_config_fetch_item() {
    let file = OpenOptions::new().create(true).write(true).open("config.conf");
    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening/creating file: {}", e);
            return;
        }
    };
    if file.metadata().unwrap().len() == 0 {
        let section_header = "[Fetch Items]\n";
        match file.write_all(section_header.as_bytes()) {
            Ok(_) => (),
            Err(e) => println!("Error writing to file: {}", e),
        }
        //let infos = r#""#;
    }
}*/
//pub fn
//pub fn
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

/*pub fn write_config() {
    // Open the file for writing and create it if it doesn't exist
    let file = OpenOptions::new().create(true).write(true).open("config.conf");

    // Check if the file was successfully opened
    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening/creating file: {}", e);
            return;
        }
    };

    // Write the ASCII art to the file if it is empty
    if file.metadata().unwrap().len() == 0 {
        let section_header = "[Custom]\n";
        match file.write_all(section_header.as_bytes()) {
            Ok(_) =>  (),
            Err(e) => println!("Error writing to file: {}", e),
        }
        let ascii = r#"
{c2}   ///
{c2}   -::-`:     {c3}.--/:-
{c2}   `-:::::..::{c3}+///+//++:+
{c2}      +-:::::-:{c3}/++++++++++//:--
{c2}       -:::::--{c3}/++++++++++++++//:
{c2}       .-:::::::{c3}//++++++++++++++//+:
{c2}       .---::::::{c3}//////++++++++++++/+
{c3}  ///:/++++++++++++++////////+++++++//::.
{c3} +/++++++++++++++++++++++++++///////++//--
{c3} ://///////////////////////////++////////:.
{c3}     /..-://////////////////++/////////////:.
{c14}     `:+{c6}oooooooooooo::+oooooooo:-{c3}/++++++:o//
{c14}    `-+{c6}oooooooooooo+  /oooooooo` -ooooos{c14}+-
{c14}    -+++{c6}ooooooooooo+  /oooooooo` -ooooooso{c14}.
{c14}    -+++{c6}ooooooooooo+--+oooooooo--/ooooooso{c14}.
{c14}  -:++++{c6}oooooooosysoooooooooooooosyyoooooo{c14}/.
{c14}  `-o+++{c6}ooooooooossooooooooooooooossoooooo{c14}+-
{c14}  `-o+++{c6}oooooooooooooooooooooooooooooooooo{c14}+-
{c14}  `-o+++{c6}oooooooooooooooooooooooooooooooooo{c14}+-
{c14}  .:+++++++{c6}ooooooooooooooooooooooooooooo{c14}++/.
{c14}    .://+++++++++++++++{c6}oo+oooooooooo{c14}+++/::.`
          {c14}///////////////////////////:
        "#;
        match file.write_all(ascii.as_bytes()) {
            Ok(_) =>  return,
            Err(e) => println!("Error writing to file: {}", e),
        }
    }


    file.flush().expect("Error flushing the file");
    file.sync_all().expect("Error syncing the file");
}

pub fn read_config() -> String{
    let mut file = OpenOptions::new().read(true).open("config.conf").expect("Error opening file for reading");

    file.flush().expect("Error flushing the file");
    file.sync_all().expect("Error syncing the file");
    file.seek(std::io::SeekFrom::Start(0)).expect("Error reading from file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading from file");

    let section_header = "[Custom]\n";
    let start_index = contents.find(section_header).expect("ASCII ART SECTION not found");
    let ascii = contents[start_index + section_header.len()..].to_string();

    let re = Regex::new(r"\{([1-9a-zA-Z_]+)\}").unwrap();
    let ascii = re.replace_all(&ascii, |caps: &regex::Captures| {
        let color = caps[1].to_string();
        match color.as_str() {
            "c1" => BLACK,
            "c2" => RED,
            "c3" => GREEN,
            "c4" => YELLOW,
            "c5" => BLUE,
            "c6" => MAGENTA,
            "c7" => CYAN,
            "c8" => WHITE,
            "c9" => BLACK_BRIGHT,
            "c10" => RED_BRIGHT,
            "c11" => GREEN_BRIGHT,
            "c12" => YELLOW_BRIGHT,
            "c13" => BLUE_BRIGHT,
            "c14" => MAGENTA_BRIGHT,
            "c15" => CYAN_BRIGHT,
            "c16" => WHITE_BRIGHT,
            "reset" => RESET,
            _ => ""
        }

    });
    return  (ascii).to_string()
}*/

impl Default for Config {
    fn default() -> Self {
        Config {
            battery:true,
            ram_data_type: RamStorageMesurement::Mib,
            ascii: "".to_string(),
        }
    }
}

