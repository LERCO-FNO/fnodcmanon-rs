use clap::Parser;
use csv::Writer;
use std::collections::HashSet;
use std::path::PathBuf;

use simple_logger::SimpleLogger;

mod anonymize;
mod error;
mod tag_dump;
mod utils;

use anonymize::{AnonymizationProfiles, DicomAnonymizer, PseudonameMethod};
use utils::{pseudoname_file_exists, validate_uid};

use crate::utils::read_pseudonames_files;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to directory with DICOM files
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Path to output directory
    #[arg(short, long, default_value = "./output")]
    output_dir: PathBuf,

    /// Anonymization prefix to set before pseudoname
    #[arg(short, long)]
    prefix: Option<String>,

    /// Pseudonames as random 10-character alphanumeric string (default)
    #[arg(long, conflicts_with_all = ["integer_count", "from_file"])]
    random_string: bool,

    /// Pseudonames as incrementing integers from starting VALUE, ex. --integer-count 5 -> <prefix>_5, <prefix>_6, ...
    #[arg(long, value_name = "VALUE", conflicts_with = "from_file", default_missing_value = "1", num_args = 0..=1)]
    integer_count: Option<u16>,

    /// Pseudonames from .txt file with optional prefixes
    #[arg(long, conflicts_with = "integer_count", value_parser = pseudoname_file_exists)]
    from_file: Option<PathBuf>,

    /// Anonymization profiles to apply
    #[arg(long, value_name = "PROFILE")]
    profile: Vec<AnonymizationProfiles>,

    /// Root UID to use for generating new UID values; must contain period separated digits
    #[arg(long, value_name = "ROOT", default_value = "2.25", value_parser = validate_uid)]
    uid_root: Option<String>,

    #[arg(long)]
    dump_tags: bool,

    /// Print at DEBUG logging level
    #[arg(long)]
    debug: bool,
}

fn resolve_method(args: &Args) -> Result<PseudonameMethod, std::io::Error> {
    if let Some(path) = args.from_file.clone() {
        let method = PseudonameMethod::FromMap {
            map: read_pseudonames_files(path)?,
        };
        return Ok(method);
    }

    if let Some(start) = args.integer_count {
        return Ok(PseudonameMethod::IntegerCount { current: start });
    }

    if !args.random_string {
        log::warn!("no anonymization method specified, using RandomString");
    }

    Ok(PseudonameMethod::RandomString)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    dbg!(&args);

    SimpleLogger::new()
        .with_level(if args.debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init()?;

    let method = resolve_method(&args)?;

    let prefix = args.prefix.unwrap_or(String::new());
    let profiles: HashSet<AnonymizationProfiles> = HashSet::from_iter(args.profile);
    let uid_root = args.uid_root.unwrap();

    let mut anonymizer = DicomAnonymizer::new(prefix, method, profiles, uid_root);

    let study_tags = anonymizer.run_anonymization(args.input_dir, &args.output_dir)?;

    if args.dump_tags {
        let mut writer = Writer::from_path(args.output_dir.join("anonymized.csv"))?;
        for study in study_tags {
            writer.serialize(study)?;
        }
        writer.flush()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let args_input: Vec<&str> = vec![
            "--",
            "--input-dir",
            "./input",
            "-p",
            "TEST",
            "--from-file",
            "./test-data/",
        ];

        let args_parse = match Args::try_parse_from(args_input.iter()) {
            Ok(res) => res,
            Err(err) => {
                println!("{err}");
                panic!("error parsing CLI arguments");
            }
        };

        let method = resolve_method(&args_parse);

        println!("{method:#?}");
        Ok(())
    }
}
