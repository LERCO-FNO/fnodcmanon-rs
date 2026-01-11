use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn find_dicom_dirs(input_path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    if !input_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Input path does not exist {0:?}", input_path.display()),
        ));
    }

    if !input_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            format!(
                "Error: Input path is not directory {0:?}",
                input_path.display()
            ),
        ));
    }

    match get_dirs(input_path) {
        Ok(dirs) => {
            if dirs.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("No dicom directories found at {:?}", input_path),
                ));
            }

            log::info!(
                "Found {0} directories with dicom files in {1}",
                dirs.len(),
                input_path.display()
            );
            Ok(dirs)
        }
        Err(e) => Err(e),
    }
}

fn get_dirs(input_path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut dicom_dirs: Vec<PathBuf> = Vec::new();
    let mut has_direct_file = false;

    for entry in fs::read_dir(input_path)? {
        let entry = match entry {
            Ok(e) => e.path(),
            Err(_) => continue,
        };

        if entry.is_symlink() {
            continue;
        }

        if entry.is_file() {
            has_direct_file = true;
        } else if entry.is_dir() {
            dicom_dirs.extend(get_dirs(&entry)?);
        }
    }

    if has_direct_file {
        dicom_dirs.push(input_path.to_path_buf());
    }

    Ok(dicom_dirs)
}

pub fn pseudoname_file_exists(path: PathBuf) -> Result<PathBuf, io::Error> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "pseudonames file not found at given path `{0}`",
                path.display()
            ),
        ));
    }

    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::IsADirectory,
            format!(
                "given pseudonames file path is a directory `{0}`",
                path.display()
            ),
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
    let entries: fs::ReadDir = match fs::read_dir(&dir) {
        Ok(r) => r,
        Err(e) => {
            log::error!("{e}");
            return None;
        }
    };
    let files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|x| x.path())
        .filter(|x| x.is_file() && x.file_name() != Some("DICOMDIR".as_ref()))
        .collect();

    if files.is_empty() {
        return None;
    }
    Some(files)
}

pub fn create_study_dir(output_parent_path: &Path, study_uid: &str) -> Result<PathBuf, io::Error> {
    let study_dir = output_parent_path.join(PathBuf::from(study_uid));

    if study_dir.exists() {
        log::warn!("directory `{}` exists, overwriting", study_dir.display());
        return Ok(study_dir);
    }

    fs::create_dir_all(&study_dir)?;

    log::info!("created directory `{}`", study_dir.display());

    Ok(study_dir)
}
