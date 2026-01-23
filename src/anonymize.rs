// use std::error::Error;
use clap::ValueEnum;
use dicom_core::{DataElement, VR};
use dicom_dictionary_std::tags;
use dicom_object::{InMemDicomObject, open_file};
use rand::{Rng, distr::Alphanumeric};
use std::{
    collections::{HashMap, HashSet},
    io,
    path::{Path, PathBuf},
};

use crate::utils;

#[derive(Debug, Default)]
pub enum PseudonameMethod {
    #[default]
    RandomString,
    IntegerCount {
        current: u16,
    },
    FromMap {
        map: HashMap<String, String>,
    },
}

#[derive(Debug, Default)]
pub struct DicomAnonymizer {
    prefix: String,
    pseudoname_method: PseudonameMethod,
    old_name: String,
    old_id: String,
    pseudoname: String, // applied to PatientName, PatientID
    study_uid: String,
    additional_profiles: HashSet<AnonymizationProfiles>,
}

impl DicomAnonymizer {
    fn new(prefix: String, method: PseudonameMethod) -> Self {
        Self {
            prefix,
            pseudoname_method: method,
            ..Default::default()
        }
    }

    fn set_pseudoname(&mut self) {
        self.pseudoname = match &mut self.pseudoname_method {
            PseudonameMethod::RandomString => {
                format!("{0}{1}", self.prefix, generate_random_string())
            }
            PseudonameMethod::IntegerCount { current } => {
                let pseudoname = format!("{0}{1}", self.prefix, *current);
                *current += 1;
                pseudoname
            }
            PseudonameMethod::FromMap { map } => match map.get(&self.old_id) {
                Some(v) => v.to_owned(),
                None => format!("{0}{1}", self.prefix, generate_random_string()),
            },
        };
    }

    fn get_basic_tags(&mut self, filepath: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let dicom_obj = open_file(filepath)?;

        self.old_id = dicom_obj.element(tags::PATIENT_ID)?.to_str()?.to_string();
        self.old_name = dicom_obj.element(tags::PATIENT_NAME)?.to_str()?.to_string();
        self.study_uid = dicom_obj
            .element(tags::STUDY_INSTANCE_UID)?
            .to_str()?
            .to_string();

        Ok(())
    }

    fn anonymize_study(
        &mut self,
        dicom_files: Vec<PathBuf>,
        output_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let study_dir = utils::create_study_dir(output_dir, &self.study_uid)?;

        for file in dicom_files {
            let mut dataset = open_file(&file)?;

            anonymize_basic_profile(&self.pseudoname, &mut dataset);

            for profile in &self.additional_profiles {
                profile.apply(&mut dataset);
            }

            let filepath = study_dir.join(file.file_name().unwrap());

            if filepath.exists() {
                log::warn!("file {} exists, overwriting", filepath.display());
            }

            dataset.write_to_file(filepath)?;
        }

        println!(
            "old id {0}, old name {1}, new id/name {2}",
            self.old_id, self.old_name, self.pseudoname
        );

        Ok(())
    }
    //     todo!()
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum AnonymizationProfiles {
    Patient,
    Institution,
    Device,
}

impl AnonymizationProfiles {
    fn apply(&self, dataset: &mut InMemDicomObject) {
        match self {
            Self::Patient => anonymize_patient_characteristic_profile(dataset),
            Self::Institution => anonymize_institution_profile(dataset),
            Self::Device => anonymize_device_profile(dataset),
        }
    }
}

pub fn run_anonymization(
    input_dir: PathBuf,
    output_dir: PathBuf,
    method: PseudonameMethod,
    prefix: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let dicom_dirs = match utils::find_dicom_dirs(&input_dir) {
        Ok(dicom_dirs) => dicom_dirs,
        Err(e) => {
            log::error!("{e}");
            return Err(Box::new(e));
        }
    };

    // TODO: finish this
    let mut dicom_anonymizer = DicomAnonymizer::new(prefix, method);

    for dir in dicom_dirs {
        let dicom_files = match utils::get_dicom_files(&dir) {
            Some(files) => files,
            None => continue,
        };

        dicom_anonymizer.get_basic_tags(dicom_files.first().unwrap())?;
        dicom_anonymizer.set_pseudoname();
        dicom_anonymizer.anonymize_study(dicom_files, &output_dir)?;
    }

    Ok(())
}

fn anonymize_basic_profile(pseudoname: &str, dataset: &mut InMemDicomObject) {
    _ = dataset.put_element(DataElement::new(tags::PATIENT_ID, VR::LO, pseudoname));

    _ = dataset.put_element(DataElement::new(tags::PATIENT_NAME, VR::PN, pseudoname));

    _ = dataset.put_element(DataElement::new(
        tags::PATIENT_SEX,
        VR::CS,
        String::from("O"),
    ));
}

fn anonymize_patient_characteristic_profile(dataset: &mut InMemDicomObject) {
    todo!()
}

fn anonymize_institution_profile(dataset: &mut InMemDicomObject) {
    todo!()
}

fn anonymize_device_profile(dataset: &mut InMemDicomObject) {
    todo!()
}

fn generate_random_string() -> String {
    let mut rng = rand::rng();
    (0..10).map(|_| rng.sample(Alphanumeric) as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod profiles {
        use super::*;

        #[test]
        fn basic() {
            let mut dataset = dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::PATIENT_ID, VR::LO, "012345"),
                DataElement::new(tags::PATIENT_NAME, VR::PN, "Some^Name"),
                DataElement::new(tags::PATIENT_SEX, VR::CS, "M"),
            ]);

            let true_dataset = dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::PATIENT_ID, VR::LO, "TST0"),
                DataElement::new(tags::PATIENT_NAME, VR::PN, "TST0"),
                DataElement::new(tags::PATIENT_SEX, VR::CS, "O"),
            ]);

            anonymize_basic_profile("TST0", &mut dataset);

            assert_eq!(dataset, true_dataset);
        }

        #[test]
        fn patient() {
            todo!()
        }

        #[test]
        fn institution() {
            todo!()
        }

        #[test]
        fn device() {
            todo!()
        }
    }
}
