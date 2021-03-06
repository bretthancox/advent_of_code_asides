mod day_one;
//mod day_six;
mod day_six_two;
mod day_five;
mod input_file_handler;
mod intcode;
mod day_seven;
#[macro_use] extern crate text_io;

use std::{
    env,
    path::PathBuf,
    fs::File,
    io::{self, BufReader, ErrorKind::*, Read},
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "doesntmatter")]
struct Opt {
    #[structopt(short, long)]
    module: usize,

    #[structopt(short, long, parse(from_os_str))]
    day: PathBuf,
}

fn get_args() -> Opt {
    let opt = Opt::from_args();
    opt
}


pub fn file_open() -> Result<String, io::Error> {
    let day: Opt = get_args();
    let path = env::current_dir()?;
    let mut full_path = PathBuf::new();
    full_path.push(path);
    full_path.push(day.day);

    let mut ret = String::new();

    if let Ok(file) = File::open(full_path) {
        let mut buf = BufReader::new(file);
        buf.read_to_string(&mut ret)?;
        Ok(ret)
    } else {
        Err(io::Error::new(InvalidData, "File not found"))
    }
}


fn main() {
    let mod_to_run: Opt = get_args();
    match mod_to_run.module {
        1 => day_one::day_one(),
        5 => day_five::day_five(),
        6 => day_six_two::six_two(),
        7 => day_seven::day_seven(),
        _ => println!("No mod for that day"),
    }
    
    //day_six::day_six();
    //;
    //day_five::day_five();
}