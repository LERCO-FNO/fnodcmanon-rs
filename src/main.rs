use clap::{Parser, ValueEnum};
use std::collections::HashSet;
use std::path::PathBuf;

use simple_logger::SimpleLogger;

use fnodcmanon::anonymize::PseudonameMethod;
use fnodcmanon::anonymize::{self, AnonymizationProfiles};
use fnodcmanon::utils;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CmdArgs {
    #[arg(short, long)]
    input_dir: PathBuf,

    #[arg(short, long, default_value = "./output")]
    output_dir: PathBuf,

    #[arg(short, long)]
    prefix: Option<String>,

    #[arg(short, long, default_value = "random-string")]
    method: ArgPseudonameMethod,

    #[arg(long, value_name = "INTEGER_START", default_value = "1")]
    integer_start: u16,

    #[arg(long, value_name = "FILEPATH", required_if_eq("method", "from-file"))]
    pseudonames_file: Option<PathBuf>,

    #[arg(long, value_name = "PROFILE")]
    profile: Vec<AnonymizationProfiles>,
}

#[derive(Debug, Clone, ValueEnum)]
enum ArgPseudonameMethod {
    RandomString,
    IntegerCount,
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
        ArgPseudonameMethod::FromFile => {
            let path = utils::pseudoname_file_exists(cmdargs.pseudonames_file.unwrap())?;
            PseudonameMethod::FromMap {
                map: utils::read_pseudonames_files(&path)?,
            }
        }
    };

    let prefix = match cmdargs.prefix {
        Some(p) => p,
        None => String::new(),
    };

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

fn generate_uid(root: &str) -> String {
    // [TODO]: improve root - date, time, device number, etc.
    let uuid = dicom_gen_uid::uuid::Uuid::new_v4().to_u128_le();

    let mut root = if root.ends_with("..") {
        format!("{root}{uuid}")
    } else {
        format!("{root}.{uuid}")
    };

    if root.len() > 64 {
        root.truncate(64);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_gen_uid;
    use std::collections::HashSet;

    #[test]
    fn test_dicom_uid() {
        let root = "1.2650.34.5.6.55555.444545";

        let uid = generate_uid(root);
        let len = uid.chars().count();
        println!("{uid} - {len}");
        assert_ne!(len, 65);
    }

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
            ArgPseudonameMethod::FromFile => {
                let path = utils::pseudoname_file_exists(cmdargs.pseudonames_file.unwrap())?;
                PseudonameMethod::FromMap {
                    map: utils::read_pseudonames_files(&path)?,
                }
            }
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
