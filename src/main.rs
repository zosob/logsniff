use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use serde::Serialize;
use serde::Deserialize;
use std::process::Command;

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

    #[arg(long, value_parser=["json","csv"])]
    export: Option<String>,

    #[arg(long)]
    detect_anomalies:bool,
}

#[derive(Debug, Serialize)]
struct LogEntry{
    line: String,
    line_length: usize,
    token_count: usize,
}

#[derive(Debug, Deserialize)]
struct Rule{
    name: String,
    pattern: String,
    severity: String
}

fn main() -> io::Result<()>{
    let args = Cli::parse();
    let rule_file = std::fs::read_to_string("data/rules.json")?;
    let rules: Vec<Rule> = serde_json::from_str(&rule_file)?;
    let path = Path::new(&args.file);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    use std::collections::HashMap;

    let mut total_matches = 0;
    let mut log_level_counts: HashMap<String, usize> = HashMap::new();
    let mut export_data: Vec<LogEntry> = Vec::new();
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
            for rule in &rules {
            if line_str.contains(&rule.pattern){
                println!("[{}] Rule Matched: {} - {}", rule.severity, rule.name, line_str);
            }
        }
            export_data.push(LogEntry {
                line: line_str.to_string(),
                line_length: line_str.len(),
                token_count: line_str.split_whitespace().count(),
            });

            for word in ["INFO", "ERROR", "WARN", "Failed", "Accepted"]{
                if line_str.contains(word){
                    *log_level_counts.entry(word.to_string()).or_default() += 1;
                }
            }
        }
        
    }

    println!("\n Total matching lines: {}", total_matches);

    for (level, count) in &log_level_counts {
        println!("{:<10}: {}", level, count);
    }


    if let Some(format) = &args.export {
        let export_path = format!("output/export.{}", format);
        println!("\n Exporting to {}", export_path);

        match format.as_str(){
            "json" => {
                let json = serde_json::to_string_pretty(&export_data)?;
                std::fs::write(&export_path, json)?;
            }

            "csv" => {
                let mut wtr = csv::Writer::from_path(&export_path)?;
                for entry in &export_data {
                    wtr.serialize(entry)?;
                }
                wtr.flush()?;
            }
            _ => eprintln!("Unknown export format!"),
        }
    
    
        if args.detect_anomalies{
            println!("\nRunning ML analysis...");
            let output = Command::new("python")
                .arg("py/analyze.py")
                .arg(&export_path)
                .output()?;
            println!("Analysis complete: See `anomalies.png` and `analyzed.csv`");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
    Ok(())
}
