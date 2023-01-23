use std::{env,fs};
use std::ffi::OsStr;
use std::path::Path;
use sysinfo::*;
use std::string::String;
use crate::color::*;
use std::process::{Command, Stdio};
use crate::config::*;
use crate::config::RamStorageMesurement::*;
use crate::utils::PackagesType::*;
/*pub fn get_terminal() -> String{
    let output = Command::new("basename");
    return format!("{RED}Terminal{WHITE} ~ {RED}{}{RED}", term).to_string();
}*/
enum PackagesType{
    Rpm,
    Apt,
    Pacman,
    Dpkg,
    Flatpak,
    None,
}
pub fn get_color_palette() ->String{
    let color_palette =format!("{BLACK_BG}    {RED_BG}    {GREEN_BG}    {YELLOW_BG}    \
    {BLUE_BG}    {MAGENTA_BG}    {CYAN_BG}    {WHITE_BG}    {RESET}\n{BLACK_BRIGHT_BG}    \
    {RED_BRIGHT_BG}    {GREEN_BRIGHT_BG}    {YELLOW_BRIGHT_BG}    {BLUE_BRIGHT_BG}    \
    {MAGENTA_BRIGHT_BG}    {CYAN_BRIGHT_BG}    {WHITE_BRIGHT_BG}    {RESET}").to_string();
    return  color_palette
}

//get the get uptime
pub fn get_uptime(system:&System) -> String {

    let uptime = system.uptime();
    let time = convert_time(uptime);
    return format!("{RED}Uptime{WHITE} ~ {}{RED}", time).to_string()

}

fn convert_time(uptime: u64) -> String {
    let  day = uptime / 60 / 60 / 24;
    let hours = uptime / 60 / 60 % 24;
    let minutes = uptime / 60 % 60;
    let mut uptime_str = String::new();
    if day > 0 {
        uptime_str.push_str(&format!("{WHITE}{} day{} ", day, if day > 1 { "s" } else { "" }));
    }
    if hours > 0 {
        uptime_str.push_str(&format!("{WHITE}{} hour{} ", hours, if hours > 1 { "s" } else { "" }));
    }
    if minutes > 0 {
        uptime_str.push_str(&format!("{WHITE}{} minute{}", minutes, if minutes > 1 { "s" } else { "" }));
    }
    if uptime_str.is_empty() {
        uptime_str = "0 seconds".to_string();
    }
    uptime_str
}
/*pub fn get_gpu() -> String {
    let output = Command::new("lspci")
        .arg("-vmm")
        .arg("-nn")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let gpu_name = stdout
        .lines()
        .filter(|line| line.contains("VGA"))
        .map(|line| line.split(':').nth(3))
        .next()
        .unwrap();

    println!("GPU name: {}", gpu_name.unwrap().to_string());
    return gpu_name.to_string()

}*/
fn verify_os(system:&System) -> String{
    let os = system.name().unwrap();
    return  os;
}
pub fn make_userprompt(sys:&System) -> String{

    if verify_os(&sys) == "Windows".to_string() {
        let host = get_hostname(sys);
        let username =  std::env::var("username").unwrap();

        let total_width = host.len() + username.len() + 1;
        let linebreak = std::iter::repeat("-").take(total_width).collect::<String>();
        let prompt = format!("{WHITE}{}@{}\n{WHITE}{}",username,host,linebreak);
        return prompt ;
    }
    else {
        let host = get_hostname(sys);
        let username = std::env::var("USER").unwrap();

        let total_width = host.len() + username.len() + 1;
        let linebreak = std::iter::repeat("-").take(total_width).collect::<String>();
        let prompt = format!("{WHITE}{}@{}\n{WHITE}{}", username, host, linebreak);
        return prompt ;
    }

}

 pub fn get_kernel(system:&System) -> String{
     let kernel =  system.kernel_version().unwrap().to_string();
     return format!("{RED}Kernel{WHITE} ~ {WHITE}{}{RED}", kernel).to_string();
 }

pub fn get_battery(system:&System) -> String
{
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
    }
    else { println!("error battery not found"); }
    return format!("{GREEN}Battery{WHITE} ~ {WHITE}{}{RESET}",batterty_percent.to_string())




}
/// Get current window manager(or DE) using env vars
fn get_wm() ->Option<String> {
    //let os = system.name().unwrap();
    //if
    let mut wm = env::var("DESKTOP_SESSION")
        .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
        .or_else(|_| env::var("WINDOWMANAGER"))
        .ok()?;
    if wm.starts_with('/') {
        wm = extract_file_from_path(&wm)?;
    }

    Some(wm)
}
pub fn wm() -> String{
    let wm =get_wm().unwrap_or_else(|| String::from("unknown"));
    return format!("{GREEN}Wm {WHITE} ~ {WHITE}{}{BLUE}",wm.to_string()).to_string()

}
/// Get Current Shell using $SHELL
fn get_shell() -> Option<String> {
   env::var("SHELL").ok().and_then(extract_file_from_path)
}
pub fn shell()-> String{
    let shell = get_shell().unwrap_or_else(|| String::from("unknown"));
    return format!("{YELLOW_BRIGHT}Shell {WHITE} ~ {WHITE}{shell}{BLUE}").to_string()

}
fn bytes_to_mb(bytes: u64) -> String {
    let mb =(bytes as f64) / 1_048_576.0;
    return format!("{:.2} Mb",mb)
}
// bytes 1_048_576.0 = 1 mib
fn bytes_to_mib(bytes: u64) -> String {
    let mb =(bytes as f64) / 1_048_576.0;
    return format!("{:.2} Mib",mb)
}
fn bytes_to_gb(bytes: u64) -> String{
    let gb = (bytes as f64)/1_073_741_824.0;
    return format!("{:.2} Gb",gb)
}
fn bytes_to_gib(bytes: u64) -> String {
   let gib= (bytes as f64) / 1_073_741_824.0;
    return format!("{:.2} Gib",gib)
}
//byte 1073741824 = 1 gib
pub fn get_os(system:&System) -> String{
    let os = system.long_os_version().unwrap();
    return format!("{CYAN}OS{WHITE} ~ {WHITE}{}{BLUE}", os).to_string();


}
pub fn get_cpu(system:&System) -> String {

    let mut cpu_brand = "";
    for cpu in system.cpus() {
        cpu_brand = &cpu.brand();

    }
    return format!("{RED}Cpu{WHITE} ~ {WHITE}{}{RED}",cpu_brand);
}
// get memory
pub fn get_memory(system: &System) -> String {
    let mem_in_total;
    let mem_in_used;
    let mem_str = match read_ram() {
        Mb => {
            mem_in_total = bytes_to_mb(system.total_memory());
            mem_in_used = bytes_to_mb(system.used_memory());
            format!("{} / {} ", mem_in_used, mem_in_total)
        },
        Mib => {
            mem_in_total = bytes_to_mib(system.total_memory());
            mem_in_used = bytes_to_mib(system.used_memory());
            format!("{} / {} ", mem_in_used, mem_in_total)
        },
        Gb => {
            mem_in_total = bytes_to_gb(system.total_memory());
            mem_in_used = bytes_to_gb(system.used_memory());
            format!("{} / {} ", mem_in_used, mem_in_total)
        },
        Gib => {
            mem_in_total = bytes_to_gib(system.total_memory());
            mem_in_used = bytes_to_gib(system.used_memory());
            format!("{} / {} ", mem_in_used, mem_in_total)
        },
    };
    return format!("{RED}Memory{WHITE} ~ {WHITE}{}{RED}", mem_str).to_string();
}

/*pub fn get_username() -> String{
    let username = std::env::var("USER").unwrap();
    return username;
}*/
pub fn get_hostname(system:&System) -> String{
    let hostname= system.host_name().unwrap().to_string();
    return hostname;

}
pub fn get_hostname_pretty(system:&System) -> String{
    let hostname= system.host_name().unwrap().to_string();
    return format!("{RED}Host{WHITE} ~ {WHITE}{}{RED}", hostname).to_string();
}

pub fn get_nb_packages() -> String{
    let mut packages_base =String::new();
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
                let str_pkg_type=" (Rpm) ";
                let output = Command::new("rpm")
                    .arg("-q")
                    .arg("-a")
                    .output().ok().unwrap();
                let stdout = String::from_utf8_lossy(&output.stdout).lines().count();

                packages_base.push_str(stdout.to_string().as_str());
                packages_base.push_str(str_pkg_type);

            },
            Apt => {
                let str_pkg_type=" (Apt) ";
                let output = Command::new("apt")
                    .arg("-l")
                    .output().ok().unwrap();
                let stdout = String::from_utf8_lossy(&output.stdout).lines().count();
                packages_base.push_str(stdout.to_string().as_str());
                packages_base.push_str(str_pkg_type);
            },
            Pacman =>{
                let str_pkg_type=" (Pacman) ";
                let output = Command::new("pacman")
                    .arg("-Q")
                    .output().ok().unwrap();
                let stdout = String::from_utf8_lossy(&output.stdout).lines().count();
                packages_base.push_str(stdout.to_string().as_str());
                packages_base.push_str(str_pkg_type);
            },
            Dpkg => {
                let str_pkg_type=" (Dpkg) ";
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
                let type_pkg=" (flatpak) ";
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
    packages_base=format!("{GREEN}Packages{WHITE} ~ {WHITE}{}{RESET}",packages_base);
    return packages_base
}
/// Extract last element of path
/// Example: a/b/c -> c
fn extract_file_from_path(path: impl ToString) -> Option<String> {
    let path = path.to_string();
    let split: Vec<_> = path.split('/').collect();
    split.last().map(|p| p.to_string())
}
