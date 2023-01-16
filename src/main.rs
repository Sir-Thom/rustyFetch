use sysinfo::*;
use prettytable::*;
mod utils;
mod  yaml ;
mod config;
mod color;



fn main() {
    let mut system = System::new_all();
    // Update all information of `System` struct.
    system.refresh_all();

    config::write_config();
    //get the ascii art form the config file
    let art = config::read_config();
    // get wm
    let wm = utils::wm();
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

    let userprompt = utils::make_userprompt(&system);

    let hostname = utils::get_hostname_pretty(&system);

    let battery = utils::get_battery(&system);

    //prepare all fetch item to print Note:order matter here
    let system_info_col = [
        "\n".to_owned(),
        userprompt,
        os,
        hostname,
        uptime,
        kernel,
        wm,
        cpu,
        memory,
        shell,
        battery,
        color_palette
    ]
        .join("\n");

    //add all info to a table and sow the table we made
    let mut table = Table::new();
    table.add_row(row![art, system_info_col]);
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    table.printstd();
}