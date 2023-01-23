use std::{env,fs};
use std::ffi::OsStr;
use std::path::Path;
use sysinfo::*;
use std::string::String;
use crate::color::*;
use std::process::{Command, Stdio};
use crate::config::*;
use crate::config::RamStorageMesurement::*;
//use crate::utils::PackagesType::*;

pub fn main(){
    let mut system = System::new_all();
    system.refresh_all();
    test_os(&system);
    test_battery(&system);
    test_hostname(&system)
}
pub fn test_hostname(system:&System)  {
    let hostname = system.host_name().unwrap().to_string();
    println!("Hostname: {} ",hostname)
}
fn test_os(system:&System){
    let os = system.name().unwrap();
    let os_long = system.long_os_version().unwrap();
    let os_ver = system.os_version().unwrap();
    println!("OS: {} ",os) ;
    println!("OS Long : {} ",os_long) ;
    println!("OS version : {} ",os_ver) ;
}
fn test_battery(system:&System){
    let mut batterty_percent= String::new();
    let percen_symbol = "%";
    let os_long= system.long_os_version().unwrap().to_string();
    let os_short: Vec<&str> = os_long.split_whitespace().collect();
    let os = os_short[0];

    if os ==  "Windows"  {
        let output = Command::new("WMIC")
            .arg("Path")
            .arg("Win32_Battery")
            .arg("Get")
            .arg("EstimatedChargeRemaining")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        batterty_percent = stdout.chars().take_while(|c| !c.eq(&'\n')).collect();
        batterty_percent.push_str(percen_symbol);
        //println!("{}", stdout);

    }
    else if os == "MacOs" {
        let output = Command::new("pmset")
            .arg("-g")
            .arg("batt")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        batterty_percent = stdout.chars().take_while(|c| !c.eq(&'\n')).collect();
        batterty_percent.push_str(percen_symbol);
        //println!("{}", stdout);
    }
    else if os == "Linux"  {
        let output = Command::new("cat").arg("/sys/class/power_supply/BAT0/capacity")
            // Tell the OS to record the command's output
            .stdout(Stdio::piped())
            // execute the command, wait for it to complete, then capture the output
            .output()
            // Blow up if the OS was unable to start the program
            .unwrap();
        // extract the raw bytes that we captured and interpret them as a string
        let stdout = String::from_utf8(output.stdout).unwrap();
        //adde % add the end
        batterty_percent = stdout.chars().take_while(|c| !c.eq(&'\n')).collect();
        batterty_percent.push_str(percen_symbol);
        println!("Battery : {} ",batterty_percent);
    }
    else { println!("error battery not found"); }
}