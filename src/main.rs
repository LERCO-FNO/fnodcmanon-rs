use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use simple_logger::SimpleLogger;

use fnodcmanon::anonymize;
use fnodcmanon::anonymize::PseudonameMethod;
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

    #[arg(value_enum, short, long, default_value = "random-string")]
    method: ArgPseudonameMethod,

    #[arg(long, value_name = "INTEGER_START", default_value = "1")]
    integer_start: u16,

    #[arg(
        long,
        value_name = "PSEUDONAMES_FILE",
        required_if_eq("method", "from-file")
    )]
    pseudonames_file: Option<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ArgPseudonameMethod {
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
            start: cmdargs.integer_start,
        },
        ArgPseudonameMethod::FromFile => {
            let filepath = cmdargs
                .pseudonames_file
                .ok_or("missing path to pseudoname file (--pseudonames-file)")?;

            PseudonameMethod::FromFile {
                path: utils::pseudoname_file_exists(filepath)?,
            }
        }
    };

    let prefix = match cmdargs.prefix {
        Some(p) => p,
        None => String::new(),
    };

    anonymize::run_anonymization(cmdargs.input_dir, cmdargs.output_dir, method, prefix)?;

    Ok(())
}
