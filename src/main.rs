//use std::borrow::Borrow;
//use std::fmt::{Debug, Display};
use std::io;


//use serde::Deserializer;
use sysinfo::*;
//use crossterm::*;
use prettytable::*;
mod utils;

mod  yaml ;
mod config;
mod color;
//use crossterm::*;
//use color::*;
//use yaml::*;
//use std::io::stdout;
//use std::io::Write;
//use std::path::Component;
//use chrono::format;
//use prettytable::format::consts;
//use serde_yaml::to_string;


fn main() {
    let mut  system = System::new_all();

    // Update all information of `System` struct.
    system.refresh_all();

    config::write_config();

    let art = config::read_config();
    // get wm
    let wm = utils::wm();
 /*   yaml::main();
    // get terminal
  //  let term = utils::terminal();
    let yaml_str = std::fs::read_to_string("art.yaml").unwrap();

    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str).unwrap();
   // let Some(art) = yaml["art"].as_sequence();
    let mut ascii = String::new();
    if let Some(arts) = yaml["art"].as_sequence() {
        for art in arts.iter().filter_map(|art| art.as_str()) {
            ascii.push_str(art);
            ascii.push_str("\n");
        }
    }*/

    // get cpu
    let cpu = utils::get_cpu(&system);
    // get shell
    let shell = utils::shell();
    // get memory
    let memory = utils::get_memory(&system);
    let kernel = utils::get_kernel(&system);
    // get uptime
    let uptime = utils::get_uptime(&system);
    // Move cursor to the starting position of the block
    let os = utils::get_os(&system);
    // get the color palette
    let color_palette = utils::get_color_palette();
    let userprompt= utils::make_userprompt(&system);
    let hostname = utils::get_hostname_pretty(&system);
    let battery = utils::get_battery(&system);





// get terminal size


    let system_info_col = [
        "\n".to_owned(),
        userprompt,
        hostname,
        uptime,
        kernel,
        os,
        wm,
        cpu,
        memory,
        shell,
        battery,
        color_palette
    ]
        .join("\n");

    let mut table = Table::new();
    table.add_row(row![art, system_info_col]);
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    table.printstd();


    // format fetch text
/*    let fetch_text = Columns::from(vec![

       format!("{}",art).split("\n").collect::<Vec<&str>>(),

        vec![
            &format!(

                "{}@{}",
                whoami::username(),
                whoami::hostname()
            ),

            //  &format!("{}",text),

            &format!("{CYAN}Os{WHITE} ~ {CYAN}{}{BLUE}", os),
            &format!("{RED}Kernel{WHITE} ~ {RED}{}{RED}", kernel),
            &format!("{RED}Uptime{WHITE} ~ {RED}{}{RED}", uptime),
            &format!("{RED}Memory{WHITE} ~ {RED}{}{RED}", memory),
            //&format!("{YELLOW}upt {WHITE} ~ {YELLOW}{uptime:?}{BLUE}"),
            &format!("{GREEN}Wm {WHITE} ~ {GREEN}{}{BLUE}",wm),

            &format!("{GREEN}Cpu{WHITE} ~ {GREEN}{cpu}{BLUE}"),
            // &format!("{MAGENTA}term{WHITE} ~ {MAGENTA}{term}{BLUE}"),
            &format!("{YELLOW_BRIGHT}Shell {WHITE} ~ {YELLOW_BRIGHT}{shell}{BLUE}"),
            &format!("{BLACK_BG}    {RED_BG}    {GREEN_BG}    {YELLOW_BG}    {BLUE_BG}    {MAGENTA_BG}    {CYAN_BG}    {WHITE_BG}    {RESET}"),
            &format!("{BLACK_BRIGHT_BG}    {RED_BRIGHT_BG}    {GREEN_BRIGHT_BG}    {YELLOW_BRIGHT_BG}    {BLUE_BRIGHT_BG}    {MAGENTA_BRIGHT_BG}    {CYAN_BRIGHT_BG}    {WHITE_BRIGHT_BG}    {RESET}"),

            //&format!("{}",text);
        ],
    ])

        .set_tabsize(40)
        .make_columns();

    println!("{fetch_text}");*/

}