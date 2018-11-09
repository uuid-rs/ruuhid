#![feature (uniform_paths)]

use clap as cmd;
use uuid::{
    Uuid,
};

fn print_uuid (v: &str) {
    match v {
        // "1" => {
        //     println! ("{}", Uuid::new_v1 () .to_hyphenated ())
        // }
        // "3" => {
        //     println! ("{}", Uuid::new_v3 () .to_hyphenated ())
        // }
        "4" => {
            println! ("{}", Uuid::new_v4 () .to_hyphenated ())
        }
        // "5" => {
        //     println! ("{}", Uuid::new_v5 () .to_hyphenated ())
        // }
        _ => {
            eprintln! ("- fail to parse -v : {}", v);
            eprintln! ("  expecting an uuid version");
            std::process::exit (1);
        }
    }
}

fn print_many_uuids (n: &str, v: &str) {
    if let Ok (n) = n.parse::<usize> () {
        for _ in 0 .. n {
            print_uuid (v);
        }
    } else {
        eprintln! ("- fail to parse -n : {}", n);
        eprintln! ("  expecting an usize number");
        std::process::exit (1);
    }
}

fn main () {
    let matches = cmd::App::new ("uuid")
        .version (cmd::crate_version! ())
        .arg (cmd::Arg::with_name ("-v")
              .short ("-v")
              .takes_value (true)
              .help ("version of uuid format, default to 4"))
        .arg (cmd::Arg::with_name ("-n")
              .short ("-n")
              .takes_value (true)
              .help ("number of uuids to generate"))
        .get_matches ();
    let mut n = "1";
    let mut v = "4";
    if let Some (n_arg) = matches.value_of ("-n") {
        n = n_arg;
    }
    if let Some (v_arg) = matches.value_of ("-v") {
        v = v_arg;
    }
    print_many_uuids (n, v);
}
