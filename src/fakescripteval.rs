extern crate regex;

use regex::Regex;
use std::io;
use std::io::{Error, ErrorKind};

fn extract_steam_app_id(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*script_(?P<id>\d+)\.vdf").unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("id").map(|x| x.as_str()))
}

fn print_description(_app_id: String) -> io::Result<()> {
    println!("0/1: <luxtorpeda game package>");
    Ok(())
}

fn download(app_id: String) -> io::Result<()> {
    println!("download {:}", app_id);
    Ok(())
}

pub fn iscriptevaluator(args: &[&str]) -> io::Result<()> {
    match args {
        ["--get-current-step", steam_app_id] => {
            let app_id = steam_app_id.to_string();
            print_description(app_id)
        }
        [script_vdf] => {
            let steam_app_id = extract_steam_app_id(script_vdf);
            match steam_app_id {
                Some(app_id) => download(app_id.to_string()),
                None => Err(Error::new(ErrorKind::Other, "Unknown app_id")),
            }
        }
        _ => Ok(()),
    }
}
