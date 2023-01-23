use sysinfo::*;
use prettytable::*;
use crate::config::read_ascii;
mod utils;
mod config;
mod color;
mod ascii;
mod test;

fn initialize_config_file() {
    let mut system = System::new_all();
    system.refresh_all();
    config::check_conf_file(&system);
}

fn main() {
    initialize_config_file();
    let mut system = System::new_all();
    // Update all information of `System` struct.
    system.refresh_all();
    let i = ascii::ascii_storage();
    i;
    let o = config::find_os(&system);
    test::main();
    //get the ascii art form the config file

    //let ram_data_type = config::read_ram();
    let art = config::translate_ascii_colors(read_ascii().unwrap().as_str());
    // get wm
    let wm = utils::wm();
    //get nb of installed packages
    let packages = utils::get_nb_packages();
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
    let mut system_info_col =vec! [
        "\n".to_owned(),
        userprompt,
        os,
        hostname,
        uptime,
        packages,
        kernel,
        wm,
        cpu,
        memory,
        shell,
        color_palette,
    ];

    if config::check_battery() {
        system_info_col.insert(10,battery);
    }

    let system_info_col = system_info_col.join("\n");


    //add all info to a table and sow the table we made
    let mut table = Table::new();
    table.add_row(row![art, system_info_col]);
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    table.printstd();
}