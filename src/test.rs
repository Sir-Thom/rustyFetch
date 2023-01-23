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
    test_os(&system)
}
fn test_os(system:&System){
    let os = system.name().unwrap();
    println!("OS: {} ",os) ;
}
fn 