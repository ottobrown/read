use std::env::args;
use std::io::Result;

mod read;
mod config;

fn main() -> Result<()> {
    let mut cf = config::Config {
        ..Default::default()
    };

    for i in 0..args().len() - 1 {
        parse_arg(&mut cf, args().nth(i).unwrap());
    }

    //Last index is the path to the file to be read
    cf.path = concat_string(args().collect(), args().len()-1 );
    return read::read(cf)
}



fn parse_arg(cf: &mut config::Config, arg: String) {
    // if read_style has not already been set
    if cf.read_style == config::ReadStyle::Default {
        cf.read_style = config::ReadStyle::from_string(&arg)
    }
    if &arg == "numbered_lines" || &arg == "nl" {
        cf.numbered_lines = true
    }
}

fn concat_string(vec: Vec<String>, start_index: usize) -> String {
    let mut s = String::new();

    for i in start_index..vec.len() {
        s += &vec[i]
    }

    return s
}