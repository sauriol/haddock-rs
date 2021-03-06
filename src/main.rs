extern crate clap;
mod haddock;

use clap::{Arg, App};
use haddock::Haddock;

fn main() {
    let matches = App::new("haddock-rs")
        .version("0.9.1")
        .author("Jack Sauriol <sauriol@csh.rit.edu>")
        .about("Generates easy to remember passwords")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("file")
            .help("Set a custom wordlist. Default: \"2of12.txt\"")
            .takes_value(true))
        .arg(Arg::with_name("length")
             .short("l")
             .long("length")
             .value_name("length")
             .help("Set a password length. Default: 20")
             .takes_value(true))
        .arg(Arg::with_name("count")
             .short("n")
             .long("number")
             .value_name("count")
             .help("The number of passwords to print. Default: 1")
             .takes_value(true))
        .get_matches();


    let file = matches.value_of("file")
        .unwrap_or("2of12.txt");
    let length = matches.value_of("length")
        .unwrap_or("20")
        .parse::<usize>()
        .unwrap();
    let count = matches.value_of("count")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap();

    let fish = Haddock::new(length, file);

    for _ in 0..count {
        match fish.generate() {
            Some(pass)  => println!("{}", pass),
            None        => panic!("Could not generate password"),
        };
    }
}
