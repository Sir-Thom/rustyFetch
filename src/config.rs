use std::fs::OpenOptions;
use std::io::{Read, Write,Seek};
use regex::Regex;
use crate::color::*;
//use std::borrow::Cow;

pub fn write_config() {
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
        let section_header = "[ASCII ART SECTION]\n";
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

    let section_header = "[ASCII ART SECTION]\n";
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
}



