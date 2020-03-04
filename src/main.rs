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

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io::prelude::Read;

static TURBO_LOCATION: &'static str = "/sys/devices/system/cpu/intel_pstate/no_turbo";
static CONSERVATION_LOCATION: &'static str =
    "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";

fn main() {
    let matches = App::new("laptopctl")
        .version("0.3.1")
        .author("gkeep")
        .arg(
            Arg::with_name("no_turbo")
                .short("t")
                .long("no_turbo")
                .takes_value(true)
                .help("Control CPU's turbo boost"),
        )
        .arg(
            Arg::with_name("conservation_mode")
                .short("c")
                .long("conservation_mode")
                .takes_value(true)
                .help("Control battery's conservation mode"),
        )
        .get_matches();

    if matches.is_present("no_turbo") {
        let argument_value = matches.value_of("no_turbo");
        let feature = "No turbo boost mode";

        if argument_value == Some("status") {
            println!("{} is {}.", feature, get_status(TURBO_LOCATION));
        } else if argument_value == Some("enable") {
            println!("{} on.", feature);
            change_value(argument_value, TURBO_LOCATION);
        } else if argument_value == Some("disable") {
            println!("{} off.", feature);
            change_value(argument_value, TURBO_LOCATION);
        } else {
            println!("Unknown value. Valid actions: enable disable status");
        }
    } else if matches.is_present("conservation_mode") {
        let argument_value = matches.value_of("conservation_mode");
        let feature = "Conservation mode";

        if argument_value == Some("status") {
            println!("{} is {}.", feature, get_status(CONSERVATION_LOCATION));
        } else if argument_value == Some("enable") {
            println!("{} on.", feature);
            change_value(argument_value, CONSERVATION_LOCATION);
        } else if argument_value == Some("disable") {
            println!("{} off.", feature);
            change_value(argument_value, CONSERVATION_LOCATION);
        } else {
            println!("Unknown value. Valid actions: enable disable status");
        }
    }
}

/* Changes value depending on first argument
 * second argument is locaiton of the file that OS speaks to */
fn change_value(value: Option<&str>, location: &str) {
    if value == Some("enable") {
        fs::write(location, "1").expect("Unable to write to file.");
    } else if value == Some("disable") {
        fs::write(location, "0").expect("Unable to write to file.");
    } else {
        println!("ERROR: Unknown action. Valid actions: enable disable")
    }
}

/* Get current value of the file,
 * i.e. whether or not a feature is active */
fn get_status(location: &str) -> &str {
    let mut file = File::open(location).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Couldn't read status.");
    // Remove \n from the input file
    content.truncate(content.len() - 1);

    if content == "1" {
        return "on";
    } else if content == "0" {
        return "off";
    } else {
        return "unknown";
    }
}
