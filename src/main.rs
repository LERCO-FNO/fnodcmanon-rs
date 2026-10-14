use clap::{Args as ClapArgs, Parser};
use snafu::ResultExt;
use std::collections::HashSet;
use std::path::PathBuf;

use tracing::error;
use tracing_subscriber::fmt::time::ChronoLocal;

mod anonymize;
mod error;
mod profiles;
mod tag_dump;
mod tag_rules_generated;
mod uid;
mod utils;

use anonymize::{DicomAnonymizer, PseudonameMethod};
use error::FromFileSnafu;
use profiles::DeidentifyProfile;
use tag_dump::write_tags_to_file;
use uid::validate_uid;
use utils::{pseudoname_file_exists, read_pseudonames_files};

use crate::error::{Error, WriteCsvDeidentifySnafu};

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

    /// Set deidentification method
    #[command(flatten)]
    pseudoname_method: PseudonameMethodArg,

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
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, ClapArgs)]
#[group(multiple = false)]
pub struct PseudonameMethodArg {
    /// Pseudonames as random 10-character alphanumeric string (default).
    #[arg(short, long, help_heading = "Pseudoname method")]
    random_string: bool,

    /// Pseudonames as incrementing unsigned integers from starting <VALUE>, ex. --integer-count 5 -> <prefix>_5, <prefix>_6, ...
    #[arg(
        short = 'c',
        long,
        value_name = "VALUE",
        default_missing_value = "1",
        num_args = 0..=1,
        help_heading = "Pseudoname method"
    )]
    integer_count: Option<u16>,

    /// Pseudonames from .txt file with optional prefixes.
    #[arg(short, long, value_parser = pseudoname_file_exists, help_heading = "Pseudoname method")]
    from_file: Option<PathBuf>,
}

impl PseudonameMethodArg {
    pub fn resolve(self) -> Result<PseudonameMethod, Error> {
        match (self.integer_count, self.from_file) {
            (Some(start), None) => Ok(PseudonameMethod::IntegerCount { current: start }),
            (None, Some(file)) => Ok(PseudonameMethod::FromMap {
                map: read_pseudonames_files(&file).context(FromFileSnafu { file })?,
            }),
            (None, None) => Ok(PseudonameMethod::RandomString),
            (Some(_), Some(_)) => {
                unreachable!("PseudonameMethodArg group enforces mutual exclusivity")
            }
        }
    }
}

//TODO: add removing tags matching 'Unknown Tag' in name

fn initialize_profiles(optional_profiles: Vec<DeidentifyProfile>) -> HashSet<DeidentifyProfile> {
    optional_profiles
        .into_iter()
        .chain([DeidentifyProfile::BasicConfidentiality])
        .collect()
}

fn main() -> Result<(), Error> {
    let Args {
        input_dir,
        output_dir,
        prefix,
        pseudoname_method,
        profile,
        uid_root,
        write_tags,
        verbose,
    } = Args::parse();

    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
            .with_max_level(if verbose {
                tracing::Level::DEBUG
            } else {
                tracing::Level::INFO
            })
            .finish(),
    )
    .unwrap_or_else(|e| error!("{}", snafu::Report::from_error(e)));

    let method = pseudoname_method.resolve()?;
    let profiles: HashSet<DeidentifyProfile> = initialize_profiles(profile);

    let study_tags = DicomAnonymizer::new(prefix, method, profiles, uid_root)
        .run_anonymization(input_dir, &output_dir)?;

    if write_tags {
        write_tags_to_file(output_dir.join("deidentified.csv"), study_tags)
            .context(WriteCsvDeidentifySnafu)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args() {
        let args_input: Vec<&str> = vec![
            "--",
            "--input-dir",
            "./input",
            "-p",
            "TEST",
            "-d",
            "retain-patient-characteristics",
            "--random-string",
        ];

        let args = Args::try_parse_from(args_input).expect("Error parsing CLI arguments");
        let active_profiles = initialize_profiles(args.profile);

        for p in [
            DeidentifyProfile::BasicConfidentiality,
            DeidentifyProfile::RetainPatientCharacteristics,
        ] {
            assert!(
                active_profiles.contains(&p),
                "Active profile set should contain {p:?}"
            );
        }

        use std::str::FromStr;
        assert_eq!(
            args.output_dir,
            PathBuf::from_str("./output").expect("Error parsing true output_dir into PathBuf")
        );
    }
}
