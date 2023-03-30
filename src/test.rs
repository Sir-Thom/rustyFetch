use std::{env,fs};
use std::ffi::OsStr;
use std::path::Path;
use sysinfo::*;
use std::string::String;
use crate::color::*;
use std::process::{Command, Stdio};
use crate::config::*;
use crate::config::RamStorageMesurement::*;
use crate::utils::*;
use crate::utils::PackagesType::*;

#[test]
pub fn main(){
    let mut system = System::new_all();
    system.refresh_all();
    test_os(&system);
    test_battery(&system);
    test_hostname(&system);
    test_packages(&system);
}
#[warn(dead_code)]
 fn test_hostname(system:&System)  {
    let hostname = system.host_name().unwrap().to_string();
    println!("Hostname: {} ",hostname)
}

pub enum PackagesType{
    Rpm,
    Apt,
    Pacman,
    Dpkg,
    Flatpak,
    None,
}

fn test_os(system:&System){
    let os = system.name().unwrap();
    let os_long = system.long_os_version().unwrap();
    let os_ver = system.os_version().unwrap();
    println!("OS: {} ",os) ;
    println!("OS Long : {} ",os_long) ;
    println!("OS version : {} ",os_ver) ;
}
 fn test_packages(system:&System) {
    let mut packages_base =String::new();
    if verify_os(&system) == "Windows" {
        let output = Command::new("winget")
            .arg("list")
            // .arg("-a")
            .output().ok().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout).lines().count();

        packages_base.push_str(stdout.to_string().as_str());
        println!(" packages : {}",packages_base)
    }
    else {
        let mut install_packages_managers = vec![];
        let package_managers = vec![
            (Rpm, "/usr/bin/rpm"),
            (Apt, "/usr/bin/apt"),
            (Pacman, "/usr/bin/pacman"),
            (Flatpak, "/usr/bin/flatpak"),
            (Dpkg, "/usr/bin/dpkg"),
        ];

        for (pkg_type, path) in package_managers {
            if fs::metadata(path).is_ok() {
                install_packages_managers.push(pkg_type);
            }
        }


        for package in install_packages_managers {
            match package {
                Rpm => {
                    let str_pkg_type = " (Rpm) ";
                    let output = Command::new("rpm")
                        .arg("-q")
                        .arg("-a")
                        .output().ok().unwrap();
                    let stdout = String::from_utf8_lossy(&output.stdout).lines().count();

                    packages_base.push_str(stdout.to_string().as_str());
                    packages_base.push_str(str_pkg_type);
                },
                Apt => {
                    let str_pkg_type = " (Apt) ";
                    let output = Command::new("apt")
                        .arg("-l")
                        .output().ok().unwrap();
                    let stdout = String::from_utf8_lossy(&output.stdout).lines().count();
                    packages_base.push_str(stdout.to_string().as_str());
                    packages_base.push_str(str_pkg_type);
                },
                Pacman => {
                    let str_pkg_type = " (Pacman) ";
                    let output = Command::new("pacman")
                        .arg("-Q")
                        .output().ok().unwrap();
                    let stdout = String::from_utf8_lossy(&output.stdout).lines().count();
                    packages_base.push_str(stdout.to_string().as_str());
                    packages_base.push_str(str_pkg_type);
                },
                Dpkg => {
                    let str_pkg_type = " (Dpkg) ";
                    let output = Command::new("dpkg-query")
                        .arg("-f")
                        .arg(".\n")
                        .arg("-W")
                        .output().ok().unwrap();
                    let stdout = String::from_utf8_lossy(&output.stdout).lines().count();
                    packages_base.push_str(stdout.to_string().as_str());
                    packages_base.push_str(str_pkg_type);
                },
                Flatpak => {
                    let type_pkg = " (flatpak) ";
                    let output = Command::new("flatpak")
                        .arg("list")
                        .output().ok().unwrap();
                    let stdout = String::from_utf8_lossy(&output.stdout).lines().count();
                    packages_base.push_str(stdout.to_string().as_str());
                    packages_base.push_str(type_pkg);
                },
                None => println!("None of those packages managers are present"),
            }
        }
        println!(" packages : {}",packages_base)
        
    }
}
fn test_battery(system:&System){
    let mut batterty_percent= String::new();
    let percen_symbol = "%";
    let os_long= system.long_os_version().unwrap().to_string();
    let os_short: Vec<&str> = os_long.split_whitespace().collect();
    let os = os_short[0];

    if verify_os(&system) ==  "Windows"  {
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
       

    }
    else if verify_os(&system) == "MacOs" {
        let output = Command::new("pmset")
            .arg("-g")
            .arg("batt")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        batterty_percent = stdout.chars().take_while(|c| !c.eq(&'\n')).collect();
        batterty_percent.push_str(percen_symbol);
        
    }
  
    else   {
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

}