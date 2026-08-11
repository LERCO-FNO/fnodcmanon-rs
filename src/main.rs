use clap::Parser;
use std::collections::HashSet;
use std::path::PathBuf;

use simple_logger::SimpleLogger;

mod anonymize;
mod error;
mod profiles;
mod tag_dump;
mod tag_rules_generated;
mod uid;
mod utils;

use anonymize::{DicomAnonymizer, PseudonameMethod};
use profiles::DeidentifyProfile;
use tag_dump::write_tags;
use utils::{pseudoname_file_exists, read_pseudonames_files, validate_uid};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to directory with DICOM files.
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Path to output directory.
    #[arg(short, long, default_value = "./output")]
    output_dir: PathBuf,

    /// Set deidentification prefix before pseudoname; Default is empty string eg. no prefix.
    #[arg(short, long, default_value = "")]
    prefix: String,

    /// Pseudonames as random 10-character alphanumeric string (default).
    #[arg(long, conflicts_with_all = ["integer_count", "from_file"])]
    random_string: bool,

    /// Pseudonames as incrementing integers from starting VALUE, ex. --integer-count 5 -> <prefix>_5, <prefix>_6, ...
    #[arg(long, value_name = "VALUE", conflicts_with = "from_file", default_missing_value = "1", num_args = 0..=1)]
    integer_count: Option<u16>,

    /// Pseudonames from .txt file with optional prefixes.
    #[arg(long, conflicts_with = "integer_count", value_parser = pseudoname_file_exists)]
    from_file: Option<PathBuf>,

    /// Enable optional deidentification profiles. Basic Application Confidentidentiality Profile always enabled by default.
    /// See generate-tag-rules/dicom/deidentify_rules.csv table deidentification actions of tags per profile.
    #[arg(
        short = 'd',
        long = "deidentify-profile",
        value_name = "PROFILE",
        verbatim_doc_comment
    )]
    profile: Vec<DeidentifyProfile>,

    /// Root UID to use for generating new UID values; must contain period separated digits.
    #[arg(short = 'u', long, value_name = "ROOT", default_value = "2.25", value_parser = validate_uid)]
    uid_root: String,

    /// Write deidentified tag values to .csv file.
    #[arg(short = 'w', long)]
    write_tags: bool,

    /// Print at DEBUG logging level.
    #[arg(long)]
    debug: bool,
}

//TODO: add removing tags matching 'Unknown Tag' in name

fn initialize_profiles(optional_profiles: Vec<DeidentifyProfile>) -> HashSet<DeidentifyProfile> {
    optional_profiles
        .into_iter()
        .chain([DeidentifyProfile::BasicConfidentiality])
        .collect()
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
        log::warn!("No anonymization method specified, using RandomString");
    }

    Ok(PseudonameMethod::RandomString)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    SimpleLogger::new()
        .with_level(if args.debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init()?;

    let method = resolve_method(&args)?;
    let profiles: HashSet<DeidentifyProfile> = initialize_profiles(args.profile);

    let study_tags = DicomAnonymizer::new(args.prefix, method, profiles, args.uid_root)
        .run_anonymization(args.input_dir, &args.output_dir)?;

    if args.write_tags {
        write_tags(args.output_dir.join("deidentified.csv"), study_tags)?;
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
            "-a",
            "patient-characteristics",
        ];

        let args_parse =
            Args::try_parse_from(args_input.iter()).expect("error parsing CLI arguments");

        let active_profiles = initialize_profiles(args_parse.profile);

        for profile in [
            DeidentifyProfile::BasicConfidentiality,
            DeidentifyProfile::RetainPatientCharacteristics,
        ] {
            assert!(active_profiles.contains(&profile));
        }

        Ok(())
    }
}
