/*
 *  _             _                   _   _
 * | | __ _ _ __ | |_ ___  _ __   ___| |_| |
 * | |/ _` | '_ \| __/ _ \| '_ \ / __| __| |
 * | | (_| | |_) | || (_) | |_) | (__| |_| |
 * |_|\__,_| .__/ \__\___/| .__/ \___|\__|_|
 *         |_|            |_|
 *
 * control your laptop's hidden features with ease
 *
 * File:       laptopctl
 * Maintainer: gkeep <gkeep77@protonmail.com>
 * License:    GNU General Public License v3.0
 * Repository: https://github.com/gkeep/laptopctl
*/

extern crate clap;

use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::{Arg, App};

static TURBO_LOCATION: &'static str = "/sys/devices/system/cpu/intel_pstate/no_turbo";
static CONSERVATION_LOCATION: &'static str = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";

fn main()
{
    let matches = App::new("laptopctl")
        .version("0.2.0")
        .author("gkeep")
        .arg(Arg::with_name("no_turbo")
            .short("t")
            .long("no_turbo")
            .takes_value(true)
            .help("Control CPU's turbo boost"))
        .arg(Arg::with_name("conservation_mode")
            .short("c")
            .long("conservation_mode")
            .takes_value(true)
            .help("Control battery's conservation mode"))
        .get_matches();

    if matches.is_present("no_turbo")
    {
        let argument_value = matches.value_of("no_turbo");
        let feature = "No turbo boost mode";

        if argument_value == Some("status")
        {
            get_status(TURBO_LOCATION, feature);
        }
        else if argument_value == Some("enable")
        {
            println!("{} enabled.", feature);
            change_value(argument_value, TURBO_LOCATION);
        }
        else if argument_value == Some("disable")
        {
            println!("{} disabled.", feature);
            change_value(argument_value, TURBO_LOCATION);
        }
        else
        {
            println!("Unknown value. Known values:\nenable disable status");
        }
    }
    else if matches.is_present("conservation_mode")
    {
        let argument_value = matches.value_of("conservation_mode");
        let feature = "Conservation mode";

        if argument_value == Some("status")
        {
            get_status(CONSERVATION_LOCATION, feature);
        }
        else if argument_value == Some("enable")
        {
            println!("{} enabled.", feature);
            change_value(argument_value, CONSERVATION_LOCATION);
        }
        else if argument_value == Some("disable")
        {
            println!("{} disabled.", feature);
            change_value(argument_value, CONSERVATION_LOCATION);
        }
        else
        {
            println!("Unknown value. Known values:\nenable disable status");
        }
    }
}

fn change_value(value: Option<&str>, location: &str)
/* Changes value depending on first argument
 * second argument is locaiton of the file that OS speaks to */
{
    match value
    {
        Some("enable") =>
        {
            let new_value = "1";
            fs::write(location, new_value).expect("Unable to write to file.");
        },
        Some("disable") =>
        {
            let new_value = "0";
            fs::write(location, new_value).expect("Unable to write to file.");
        },
        _ => println!("ERROR: Unknown action. Valid actions: enable; disable"),
    }
}

fn get_status(location: &str, feature: &str)
/* Get current value of the file,
 * i.e. whether or not a feature is active */
{
    let file = File::open(location).unwrap();
    let buffer = BufReader::new(file);

    for line in buffer.lines()
    {
        if line.unwrap() == 1.to_string()
        {
            println!("{} is enabled", feature);
        }
        else
        {
            println!("{} is disabled", feature);
        }
    }

}
