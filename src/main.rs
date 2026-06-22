use clap::Parser;
use log::{info, warn};
use serde::Deserialize;
use simplelog::{
    ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::collections::HashSet;
use std::fs;
use std::fs::File;

#[derive(Parser)]
#[command(name = "email-cleaner-cli")]
#[command(about = "Professional email cleaning and validation CLI tool")]
struct Args {
	#[arg(long, help = "Path to input file(s)", num_args = 1..)]
	input: Vec<String>,

	#[arg(long, help = "Path to output file")]
	output: String,

	#[arg(long, default_value_t = false, help = "Simulate without saving results")]
	dry_run: bool,
	
	#[arg(long, default_value_t = false, help = "Print statistics only, do not write output")]
	stats_only: bool,
	
	#[arg(long, default_value = "email", help = "Validation mode: email, phone, url")]
	mode: String,
}

#[derive(Deserialize)]
struct Config {
		email_pattern: String,
		log_file: String,
}

fn load_config(path: &str) -> Result<Config, String> {
    let content = fs::read_to_string(path)
        .map_err(|_| format!("'{}' not found.", path))?;

    let config: Config = serde_json::from_str(&content)
        .map_err(|_| "config.json is not valid JSON.".to_string())?;

    if config.email_pattern.trim().is_empty() {
        return Err("Invalid 'email_pattern' in config.json.".to_string());
    }
    if config.log_file.trim().is_empty() {
        return Err("Invalid 'log_file' in config.json.".to_string());
    }

    Ok(config)
}

fn get_pattern(mode: &str, config_pattern: &str) -> Result<String, String> {
    match mode {
        "email" => Ok(config_pattern.to_string()),
        "phone" => Ok(r"^\+?[\d\s\-\(\)]{7,15}$".to_string()),
        "url" => Ok(r"^https?://[\w\-]+(\.[\w\-]+)+(/[\w\-._~:/?#\[\]@!$&'()*+,;=]*)?$".to_string()),
        other => Err(format!("Unknown mode '{}'. Use: email, phone, url", other)),
    }
}

struct Stats {
    total_lines: usize,
    empty_lines: usize,
    valid_matches: usize,
    invalid_count: usize,
    duplicates_removed: usize,
}

fn process_emails(
    input_paths: &[String],
    pattern: &str,
) -> Result<(Vec<String>, Stats), String> {
    let regex = regex::Regex::new(pattern)
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;

    let mut unique_emails: HashSet<String> = HashSet::new();
    let mut total_lines = 0;
    let mut empty_lines = 0;
    let mut valid_matches: usize = 0;
    let mut invalid_count = 0;

    for input_path in input_paths {
        info!("Processing file: {}", input_path);

        let content = fs::read_to_string(input_path)
            .map_err(|_| format!("Input file not found: {}", input_path))?;

        for (line_num, line) in content.lines().enumerate() {
            total_lines += 1;
            let email = line.trim().to_lowercase();

            if email.is_empty() {
                empty_lines += 1;
                continue;
            }

            if regex.is_match(&email) {
                valid_matches += 1;
                unique_emails.insert(email);
            } else {
                invalid_count += 1;
                warn!("Invalid entry at line {} in {}: {}", line_num + 1, input_path, email);
            }
        }
    }

    let duplicates_removed = valid_matches.saturating_sub(unique_emails.len());
    let mut sorted_emails: Vec<String> = unique_emails.into_iter().collect();
    sorted_emails.sort();

    let stats = Stats {
        total_lines,
        empty_lines,
        valid_matches,
        invalid_count,
        duplicates_removed,
    };

    Ok((sorted_emails, stats))
}

fn main() {
    let args = Args::parse();
	
			let log_file = File::create("app.log").expect("Could not create log file");

		CombinedLogger::init(vec![
    TermLogger::new(LevelFilter::Info, simplelog::Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
    WriteLogger::new(LevelFilter::Info, simplelog::Config::default(), log_file),
])
.expect("Could not initialize logger");

    let config = match load_config("config.json") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Critical Error: {}", e);
            std::process::exit(1);
        }
    };
	
		let pattern = match get_pattern(&args.mode, &config.email_pattern) {
			Ok(p) => p,
			Err(e) => {
				eprintln!("Critical Error: {}", e);
				std::process::exit(1);
			}
	};

info!("Mode: {}", args.mode);

    let (emails, stats) = match process_emails(&args.input, &pattern) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Critical Error: {}", e);
            std::process::exit(1);
        }
    };

		info!("----------------------------------------");
		info!("SUMMARY");
		info!("----------------------------------------");
		info!("Total lines processed : {}", stats.total_lines);
		info!("Empty lines           : {}", stats.empty_lines);
		info!("Valid entries (matches): {}", stats.valid_matches);
		info!("Valid unique entries   : {}", emails.len());
		info!("Invalid entries        : {}", stats.invalid_count);
		info!("Duplicates removed     : {}", stats.duplicates_removed);
		info!("----------------------------------------");

    if args.stats_only {
    info!("Stats-only mode: no output file written.");
} else if args.dry_run {
    info!("Dry-run mode: no output file written.");
} else {
    let output_content = emails.join("\n");
    match fs::write(&args.output, output_content) {
        Ok(_) => info!("SUCCESS: Output written to '{}'", args.output),
        Err(e) => {
            eprintln!("Permission Error: Could not write to '{}': {}", args.output, e);
            std::process::exit(1);
        }
    }
}
}