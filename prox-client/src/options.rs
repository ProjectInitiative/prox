extern crate clap;
use clap::{Arg, App};

pub fn get_matches() -> clap::ArgMatches<'static>
{
    return App::new("Prox")
                    .version("0.0.1")
                    .author("Kyle Petryszak <projectinitiativedev@gmail.com>")
                    .about("Control a Proxmox instance")
                    // .arg(Arg::with_name("config")
                    //     .short("c")
                    //     .long("config")
                    //     .value_name("FILE")
                    //     .help("Sets a custom config file")
                    //     .takes_value(true))
                    // .arg(Arg::with_name("INPUT")
                    //     .help("Sets the input file to use")
                    //     .required(true)
                    //     .index(1))
                    .arg(Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Sets the level of verbosity"))
                    .arg(Arg::with_name("deletecache")
                        .short("x")
                        .long("deletecache")
                        .help("Deletes the configuration file"))
                    .get_matches();
}