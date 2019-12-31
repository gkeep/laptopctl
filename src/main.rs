extern crate clap;

use std::fs;
use clap::{Arg, App};

fn main()
{
    let matches = App::new("laptopctl")
        .version("0.1.0")
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

    // TODO: Optimize code
    if matches.is_present("no_turbo")
    {
        let argument_value = matches.value_of("no_turbo");
        change_value(argument_value, "/sys/devices/system/cpu/intel_pstate/no_turbo");

        if argument_value == Some("enable")
        {
            println!("Turbo boost disabled!");
        }
        else if argument_value == Some("disable")
        {
            println!("Turbo boost enabled!");
        }
    }
    else if matches.is_present("conservation_mode")
    {
        let argument_value = matches.value_of("conservation_mode");
        change_value(argument_value, "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode");

        if argument_value == Some("enable")
        {
            println!("Battery conservation mode enabled!");
        }
        else if argument_value == Some("disable")
        {
            println!("Battery conservation mode disabled!");
        }
    }
}

/* Changes value depending on first argument
 * second argument is locaiton of the file that OS speaks to */
fn change_value(value: Option<&str>, location: &str)
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
        _ => println!("ERROR: Unknown value"),
    }
}
