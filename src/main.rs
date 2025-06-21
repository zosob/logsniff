use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser)]
#[command(name = "logsniff")]
#[command(version = " 0.1 ")]
#[command(about = "Parser and fileter logs like a rusty pro", long_about = None)]

struct Cli{
    #[arg(short, long)]
    file: String,
}

fn main() -> io::Result<()>{
    let args = Cli::parse();

    let path = Path::new(&args.file);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines(){
        let line = line_result?;
        println!("{}", line);
    }
    Ok(())
}
