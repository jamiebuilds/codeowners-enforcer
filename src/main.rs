#[macro_use]
extern crate clap;
extern crate codeowners;
extern crate ignore;

use ansi_term::{Colour, Style};
use clap::{App, Arg};
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::env;

use std::error::Error;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug)]
struct EnforcerResult {
    config_path: PathBuf,
    unowned_files: Vec<PathBuf>,
}

fn codeowners_enforcer(
    cwd: &PathBuf,
    search_patterns: Vec<&str>,
    ignore_patterns: Vec<&str>,
) -> Result<EnforcerResult, Box<dyn Error>> {
    // Find CODEOWNERS file
    let config_path = match codeowners::locate(&cwd) {
        Some(path) => path,
        None => panic!(format!("No CODEOWNERS file found from {}", cwd.display())),
    };

    // Parse CODEOWNERS file
    let codeowners = codeowners::from_path(&config_path);

    // Merge search+ignore patterns into one set to search.
    let mut override_builder = OverrideBuilder::new(&cwd);

    for search_pattern in search_patterns {
        override_builder.add(&search_pattern)?;
    }

    for ignore_pattern in ignore_patterns {
        // Prefix ignore patterns with !
        override_builder.add(&format!("!{}", ignore_pattern))?;
    }

    let overrides = override_builder.build()?;

    // Create iterator that walks the file system using the search+ignore patterns
    let walker = WalkBuilder::new(&cwd)
        .add_custom_ignore_filename(".codeownersignore")
        .parents(false)
        .git_global(false)
        .overrides(overrides)
        .build()
        .into_iter()
        .filter_map(Result::ok);

    let mut unowned_files = vec![];

    // Iterate over files that match all patterns
    for file in walker {
        if file.file_type().unwrap().is_dir() {
            continue;
        }

        let path = file.into_path();
        let owners = codeowners.of(&path); // Find the owner of the file

        match owners {
            None => unowned_files.push(path),
            _ => {}
        }
    }

    Ok(EnforcerResult {
        config_path: config_path,
        unowned_files: unowned_files,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // Definition of CLI (https://docs.rs/clap/2.33.0/clap/)
    let matches = App::new("codeowners-enforcer")
        .version(crate_version!())
        .author("Jamie Kyle <me@thejameskyle.com>")
        .about("Enforces every file has owners")
        .setting(clap::AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("PATTERNS")
                .multiple(true)
                .help("Files to check")
                .index(1),
        )
        .arg(
            Arg::with_name("ignore")
                .short("i")
                .multiple(true)
                .value_name("PATTERN")
                .help("Ignore some file patterns"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Silence informational output"),
        )
        .get_matches();

    // C.urrent W.orking D.irectory
    let cwd = env::current_dir().unwrap();

    // Get glob patterns to search for from CLI, default "**"
    let search_patterns: Vec<&str> = match matches.values_of("PATTERNS") {
        Some(patterns) => patterns.collect(),
        None => vec![],
    };

    // Get glob patterns to ignore from the CLI
    let ignore_patterns: Vec<&str> = match matches.values_of("ignore") {
        Some(patterns) => patterns.collect(),
        None => vec![],
    };

    let is_quiet_mode = matches.is_present("quiet");

    let result = codeowners_enforcer(&cwd, search_patterns, ignore_patterns)?;

    if result.unowned_files.is_empty() {
        exit(0)
    }

    if !is_quiet_mode {
        eprintln!(
            "{} Found files without CODEOWNERS!",
            Style::new().bold().fg(Colour::Red).paint("Oops!")
        );
        eprintln!("");
    }

    for path in result.unowned_files {
        println!(
            "{}",
            Style::new()
                .fg(Colour::Red)
                .paint(path.strip_prefix(&cwd)?.display().to_string())
        );
    }

    if !is_quiet_mode {
        eprintln!("");
        eprintln!(
            "{} Please delete these files, move them, or add owners to them in {}",
            Style::new().bold().fg(Colour::Cyan).paint("Fix:"),
            Style::new()
                .underline()
                .paint(result.config_path.display().to_string())
        );
    }

    exit(1);
}
