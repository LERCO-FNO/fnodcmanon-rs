use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

use rand::Rng;
use rand::distr::Alphanumeric;
use tracing::{error, info, warn};

use crate::error::{Error, ScanDirectorySnafu};

pub fn find_dicom_dirs(input_path: &Path) -> Result<Vec<PathBuf>, Error> {
    if !input_path.exists() {
        return ScanDirectorySnafu {
            message: format!("Input path does not exist at `{}`", input_path.display()),
        }
        .fail();
    }

    if !input_path.is_dir() {
        return ScanDirectorySnafu {
            message: format!("Input path is not directory `{}`", input_path.display()),
        }
        .fail();
    }

    let dirs = get_dirs(input_path).map_err(|e| {
        ScanDirectorySnafu {
            message: format!(
                "Failed to scan `{}` for DICOM directories: {e}",
                input_path.display(),
            ),
        }
        .build()
    })?;

    if dirs.is_empty() {
        return ScanDirectorySnafu {
            message: format!("No dicom directories at `{}`", input_path.display()),
        }
        .fail();
    }

    info!(
        "Found {} directories with DICOM files in `{}`",
        dirs.len(),
        input_path.display()
    );
    Ok(dirs)

    // {
    //     Ok(dirs) => {
    //         if dirs.is_empty() {
    //             ScanDirectorySnafu {
    //                 message: format!("No dicom directories at `{}`", input_path.display()),
    //             }
    //             .build();
    //         }
    //
    //         info!(
    //             "Found {0} directories with dicom files in {1}",
    //             dirs.len(),
    //             input_path.display()
    //         );
    //         Ok(dirs)
    //     }
    //     Err(e) => Err(ScanDirectorySnafu {
    //         message: format!("{e}"),
    //     }
    //     .build()),
    // }
}

fn get_dirs(input_path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut dicom_dirs: Vec<PathBuf> = Vec::new();
    let mut has_direct_file = false;

    for entry in fs::read_dir(input_path)? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!(
                    "Skipping unreadable entry in `{}`: {e}",
                    input_path.display()
                );
                continue;
            }
        };

        let Ok(ft) = entry.file_type() else { continue };

        if ft.is_symlink() {
            continue;
        } else if ft.is_file() {
            has_direct_file = true;
        } else {
            dicom_dirs.extend(get_dirs(&entry.path())?);
        }
    }

    if has_direct_file {
        dicom_dirs.push(input_path.to_path_buf());
    }

    Ok(dicom_dirs)
}

pub fn pseudoname_file_exists(path: &str) -> Result<PathBuf, io::Error> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("pseudonames file path not found `{0}`", path.display()),
        ));
    }

    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::IsADirectory,
            format!("pseudonames file path is a directory `{0}`", path.display()),
        ));
    }

    Ok(path)
}

pub fn read_pseudonames_files(path: &Path) -> Result<HashMap<String, String>, io::Error> {
    let content = std::fs::read_to_string(path)?;

    let mut pseudonames_map: HashMap<String, String> = HashMap::new();

    for line in content.lines() {
        if line.contains("PatientID") {
            continue;
        }
        let mut parts = line
            .trim()
            .splitn(2, ',')
            .map(|s| s.trim().trim_matches(','));

        if let (Some(id), Some(pseudoname)) = (parts.next(), parts.next()) {
            pseudonames_map.insert(id.to_owned(), pseudoname.to_owned());
        }
    }

    Ok(pseudonames_map)
}

pub fn get_dicom_files(dir: &Path) -> Option<Vec<PathBuf>> {
    let entries: fs::ReadDir = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return None;
        }
    };
    let files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|x| x.path())
        .filter(|x| {
            x.is_file()
//                && x.extension().is_none()
                && x.file_name().and_then(|x| x.to_str()) != Some("DICOMDIR")
        })
        .collect();

    if files.is_empty() {
        return None;
    }
    Some(files)
}

pub fn create_study_dir(output_parent_path: &Path, study_uid: &str) -> Result<PathBuf, io::Error> {
    let study_dir = output_parent_path.join(PathBuf::from(study_uid));

    if study_dir.exists() {
        warn!("directory `{}` exists, overwriting", study_dir.display());
        return Ok(study_dir);
    }

    fs::create_dir_all(&study_dir)?;

    info!("created directory `{}`", study_dir.display());

    Ok(study_dir)
}

pub fn generate_random_string() -> String {
    let mut rng = rand::rng();
    (0..10).map(|_| rng.sample(Alphanumeric) as char).collect()
}
