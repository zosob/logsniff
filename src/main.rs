use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser)]
#[command(name = "logsniff")]
#[command(version = " 0.1 ")]
#[command(about = "Parser and fileter logs like a rusty pro", long_about = None)]

struct Cli{
    #[arg(short = 'f', long)]
    file: String,

    //Filter lines
    #[arg(short = 'k', long)]
    filter: Option<String>,

    //Count matching lines
    #[arg(long)]
    count: bool,

    //Summarize log level frequency
    #[arg(long)]
    summary: bool,
}

fn main() -> io::Result<()>{
    let args = Cli::parse();

    let path = Path::new(&args.file);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    use std::collections::HashMap;

    let mut total_matches = 0;
    let mut log_level_counts: HashMap<String, usize> = HashMap::new();
    for line_result in reader.split(b'\n'){
        let line = line_result?;
        let line_str = String::from_utf8_lossy(&line);

        let matches_filter = if let Some(ref keyword) = args.filter{
            line_str.contains(keyword)
        } else {
            true
        };
        
        if matches_filter{
            total_matches+=1;
            println!("{}", line_str);

            for word in ["INFO", "ERROR", "WARN", "Failed", "Accepted"]{
                if line_str.contains(word){
                    *log_level_counts.entry(word.to_string()).or_default() += 1;
                }
            }
        }
    }

    if args.count{
        println!("\n Total matching lines: {}", total_matches);
    }

    if args.summary {
        println!("\n Summary:");
        for (level,count) in &log_level_counts {
            println!("{:<10} : {}", level, count);
        }
    }
    Ok(())
}
