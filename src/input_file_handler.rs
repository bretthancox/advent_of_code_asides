use std::{
    env,
    path::PathBuf,
    fs::File,
    io::{self, BufReader, ErrorKind::*, Read},
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "advent")]
struct Opt {
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