use clap::{Parser, ValueEnum};
use std::collections::HashSet;
use std::path::PathBuf;

use simple_logger::SimpleLogger;

mod anonymize;
mod utils;

use anonymize::{AnonymizationProfiles, PseudonameMethod};

use crate::utils::pseudoname_file_exists;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CmdArgs {
    /// Path to directory with DICOM files
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Path to output directory
    #[arg(short, long, default_value = "./output")]
    output_dir: PathBuf,

    /// Anonymization prefix to set before pseudoname
    #[arg(short, long)]
    prefix: Option<String>,

    /// Pseudoname generation method
    #[arg(short, long, default_value = "random-string")]
    method: ArgPseudonameMethod,

    /// Initial integer counter value, requires --method integer-count
    #[arg(long, value_name = "INTEGER_START", default_value = "1")]
    integer_start: u16,

    /// Path to .txt file containing pseudonames with optional prefixes; requires --method from-file
    #[arg(long, value_name = "FILEPATH", required_if_eq("method", "from-file"), value_parser = pseudoname_file_exists)]
    pseudonames_file: Option<PathBuf>,

    /// Anonymization profile to apply
    #[arg(long, value_name = "PROFILE")]
    profile: Vec<AnonymizationProfiles>,

    /// Root UID to use for generating new UID values; must contain period separated digits
    #[arg(long, value_name = "ROOT", default_value = "2.25", value_parser = validate_uid)]
    uid_root: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum ArgPseudonameMethod {
    /// Generate ten-character alphanumeric string
    RandomString,
    /// Increment <integer_start> from initial value, ex. <prefix>_1, <prefix>_2, ...
    IntegerCount,
    /// Use pseudonames from a .txt file
    FromFile,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmdargs = CmdArgs::parse();

    dbg!(&cmdargs);

    SimpleLogger::new().init()?;

    let method = match cmdargs.method {
        ArgPseudonameMethod::RandomString => PseudonameMethod::RandomString,
        ArgPseudonameMethod::IntegerCount => PseudonameMethod::IntegerCount {
            current: cmdargs.integer_start,
        },
        ArgPseudonameMethod::FromFile => PseudonameMethod::FromMap {
            map: utils::read_pseudonames_files(cmdargs.pseudonames_file.unwrap())?,
        },
    };

    let prefix = cmdargs.prefix.unwrap_or(String::new());

    let profiles = HashSet::from_iter(cmdargs.profile);
    anonymize::run_anonymization(
        cmdargs.input_dir,
        cmdargs.output_dir,
        method,
        prefix,
        profiles,
    )?;

    Ok(())
}

fn validate_uid(uid: &str) -> Result<String, String> {
    if uid.chars().any(|c| !c.is_ascii_digit() && c != '.') {
        return Err(format!(
            "'{}' invalid character in UID root, only period separated digits allowed",
            uid
        ));
    }

    if uid.ends_with("..") {
        return Err(format!("'{}' UID cannot end with '..'", uid));
    }

    Ok(uid.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn arg_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let cli_args: Vec<&str> = vec![
            "--",
            "--input-dir",
            "./input",
            "-p",
            "TEST",
            "-m",
            "from-file",
            "--pseudonames-file",
            "./test-data/pseudonames.txt",
        ];

        let cmdargs = match CmdArgs::try_parse_from(cli_args.iter()) {
            Ok(res) => res,
            Err(err) => {
                println!("{err}");
                panic!("error parsing CLI arguments");
            }
        };

        dbg!(&cmdargs);

        let method = match cmdargs.method {
            ArgPseudonameMethod::RandomString => PseudonameMethod::RandomString,
            ArgPseudonameMethod::IntegerCount => PseudonameMethod::IntegerCount {
                current: cmdargs.integer_start,
            },
            ArgPseudonameMethod::FromFile => PseudonameMethod::FromMap {
                map: utils::read_pseudonames_files(cmdargs.pseudonames_file.unwrap())?,
            },
        };

        println!("{method:#?}");

        Ok(())
    }

    #[test]
    fn set_profiles() {
        let cli_args: Vec<&str> = vec![
            "--",
            "--input-dir",
            "./input",
            "-p",
            "TEST",
            "--profile",
            "device",
            "--profile",
            "device",
            "--profile",
            "patient",
            "--profile",
            "institution",
            "--profile",
            "institution",
            "--profile",
            "patient",
        ];

        let cmdargs = match CmdArgs::try_parse_from(cli_args.iter()) {
            Ok(res) => res,
            Err(err) => panic!("error parsing CLI arguments: {err}"),
        };

        let true_set: HashSet<anonymize::AnonymizationProfiles> = HashSet::from_iter([
            anonymize::AnonymizationProfiles::Institution,
            anonymize::AnonymizationProfiles::Device,
            anonymize::AnonymizationProfiles::Patient,
        ]);

        let set = HashSet::from_iter(cmdargs.profile);
        dbg!(&set);
        assert_eq!(set, true_set);
    }
}
