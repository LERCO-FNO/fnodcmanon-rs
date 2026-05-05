// use std::error::Error;
use clap::ValueEnum;
use dicom_core::{DataElement, VR};
use dicom_dictionary_std::tags;
use dicom_object::{InMemDicomObject, open_file};
use rand::{Rng, distr::Alphanumeric};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};
use uuid;

use crate::utils::{create_study_dir, find_dicom_dirs, get_dicom_files};

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
    // uid_root: String,
}

impl DicomAnonymizer {
    fn new(
        prefix: String,
        method: PseudonameMethod,
        profiles: HashSet<AnonymizationProfiles>,
    ) -> Self {
        Self {
            prefix,
            pseudoname_method: method,
            additional_profiles: profiles,
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
        let study_dir = create_study_dir(output_dir, &self.study_uid)?; // utils::create_study_dir(output_dir, &self.study_uid)?;

        // todo!("create study uid");

        for file in dicom_files {
            let mut dataset = open_file(&file)?;

            anonymize_basic_profile(&self.pseudoname, &mut dataset);

            for profile in &self.additional_profiles {
                profile.apply(&mut dataset);
                update_deidentification_method_element(&mut dataset, profile);
            }

            let filepath = study_dir.join(file.file_name().unwrap());

            if filepath.exists() {
                log::warn!("file {} exists, overwriting", filepath.display());
            }

            // todo!("add study instance uid, series instance uid, sop instance uid replacement");

            dataset.write_to_file(filepath)?;
        }

        println!(
            "old id {0}, old name {1}, new id/name {2}",
            self.old_id, self.old_name, self.pseudoname
        );

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum AnonymizationProfiles {
    /// Patient related DICOM tags
    Patient,
    /// Institution related DICOM tags
    Institution,
    /// Device related DICOM tags
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

    fn profile_as_string_code(&self) -> String {
        let method = match self {
            Self::Patient => "DCM_01",
            Self::Institution => "DCM_02",
            Self::Device => "DCM_03",
        };
        method.to_string()
    }
}

pub fn run_anonymization(
    input_dir: PathBuf,
    output_dir: PathBuf,
    method: PseudonameMethod,
    prefix: String,
    profiles: HashSet<AnonymizationProfiles>,
) -> Result<(), Box<dyn std::error::Error>> {
    let dicom_dirs = find_dicom_dirs(&input_dir)?; // utils::find_dicom_dirs(&input_dir)?;

    // TODO: finish this
    let mut dicom_anonymizer = DicomAnonymizer::new(prefix, method, profiles);

    for dir in dicom_dirs {
        let dicom_files = match get_dicom_files(&dir) {
            // utils::get_dicom_files(&dir) {
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

    _ = dataset.put_element(DataElement::new(
        tags::PATIENT_IDENTITY_REMOVED,
        VR::CS,
        "YES",
    ));
}

fn anonymize_patient_characteristic_profile(dataset: &mut InMemDicomObject) {
    dataset.remove_element(tags::ALLERGIES);
    dataset.remove_element(tags::PATIENT_AGE);
    dataset.remove_element(tags::PATIENT_SEX_NEUTERED);
    dataset.remove_element(tags::PATIENT_SIZE);
    dataset.remove_element(tags::PATIENT_WEIGHT);
    dataset.remove_element(tags::PATIENT_STATE);
    dataset.remove_element(tags::PREGNANCY_STATUS);
    dataset.remove_element(tags::SMOKING_STATUS);
}

fn anonymize_institution_profile(dataset: &mut InMemDicomObject) {
    dataset.remove_element(tags::INSTITUTION_ADDRESS);
    dataset.remove_element(tags::INSTITUTIONAL_DEPARTMENT_NAME);
    dataset.remove_element(tags::INSTITUTIONAL_DEPARTMENT_TYPE_CODE_SEQUENCE);
    dataset.remove_element(tags::INSTITUTION_CODE_SEQUENCE);
    dataset.remove_element(tags::INSTITUTION_NAME);
}

fn anonymize_device_profile(dataset: &mut InMemDicomObject) {
    dataset.remove_element(tags::DEVICE_DESCRIPTION);
    dataset.remove_element(tags::DEVICE_LABEL);
    dataset.remove_element(tags::DEVICE_SERIAL_NUMBER);
    dataset.remove_element(tags::MANUFACTURER_DEVICE_IDENTIFIER);
    dataset.remove_element(tags::PERFORMED_STATION_NAME);
    dataset.remove_element(tags::PERFORMED_STATION_NAME_CODE_SEQUENCE);
    dataset.remove_element(tags::SCHEDULED_STATION_NAME);
    dataset.remove_element(tags::SCHEDULED_STATION_NAME_CODE_SEQUENCE);
    dataset.remove_element(tags::SOURCE_MANUFACTURER);
    dataset.remove_element(tags::SOURCE_SERIAL_NUMBER);
    dataset.remove_element(tags::STATION_NAME);
}

fn update_deidentification_method_element(
    dataset: &mut InMemDicomObject,
    profile: &AnonymizationProfiles,
) {
    // let mut el = dataset
    //     .element_opt(tags::DEIDENTIFICATION_METHOD)?
    //     .and_then(|f| f.to_str().ok())
    //     .unwrap_or_default()
    //     .into_owned();

    let mut element_val = match dataset.element_opt(tags::DEIDENTIFICATION_METHOD) {
        Ok(Some(el)) => el.to_str().map(|s| s.to_string()).unwrap_or_default(),
        _ => String::new(),
    };

    if !element_val.is_empty() {
        element_val.push('\\');
    }

    let profile_code = profile.profile_as_string_code();
    println!("adding {0}", profile_code);
    element_val.push_str(&profile_code);

    let _ = dataset.put_element(DataElement::new(
        tags::DEIDENTIFICATION_METHOD,
        VR::LO,
        element_val,
    ));
}

fn generate_random_string() -> String {
    let mut rng = rand::rng();
    (0..10).map(|_| rng.sample(Alphanumeric) as char).collect()
}

pub fn generate_uid(root: &str) -> String {
    let uid = uuid::Uuid::now_v7().to_u128_le();
    if root.ends_with(".") {
        format!("{0}{1}", root, uid)
    } else {
        format!("{0}.{1}", root, uid)
    }
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

            anonymize_basic_profile("TST0", &mut dataset);

            let true_dataset = dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::PATIENT_ID, VR::LO, "TST0"),
                DataElement::new(tags::PATIENT_NAME, VR::PN, "TST0"),
                DataElement::new(tags::PATIENT_SEX, VR::CS, "O"),
                DataElement::new(tags::PATIENT_IDENTITY_REMOVED, VR::CS, "YES"),
            ]);
            assert_eq!(dataset, true_dataset);
        }

        #[test]
        fn patient() {
            let mut dataset = dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::ALLERGIES, VR::LO, "Some Allergies"),
                DataElement::new(tags::PATIENT_AGE, VR::AS, "063Y"),
                DataElement::new(tags::PATIENT_SEX_NEUTERED, VR::CS, "ALTERED"),
                DataElement::new(tags::PATIENT_SIZE, VR::DS, "1.6256"),
                DataElement::new(tags::PATIENT_WEIGHT, VR::DS, "68.025"),
                DataElement::new(tags::PATIENT_STATE, VR::LO, "comatose"),
                DataElement::new(tags::PREGNANCY_STATUS, VR::US, "0004"),
                DataElement::new(tags::SMOKING_STATUS, VR::CS, "YES"),
            ]);

            anonymize_patient_characteristic_profile(&mut dataset);

            let true_dataset = dicom_object::InMemDicomObject::new_empty();
            assert_eq!(dataset, true_dataset);
        }

        #[test]
        fn institution() {
            let mut dataset = dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::INSTITUTION_ADDRESS, VR::ST, "Some address"),
                DataElement::new(
                    tags::INSTITUTIONAL_DEPARTMENT_NAME,
                    VR::LO,
                    "Some Department Name",
                ),
                DataElement::new(tags::INSTITUTION_NAME, VR::LO, "Some Institution Name"),
            ]);

            anonymize_institution_profile(&mut dataset);

            let true_dataset = dicom_object::InMemDicomObject::new_empty();
            assert_eq!(dataset, true_dataset);
        }

        #[test]
        fn device() {
            let mut dataset = dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::DEVICE_DESCRIPTION, VR::LO, "device description"),
                DataElement::new(tags::DEVICE_LABEL, VR::LO, "device label"),
                DataElement::new(tags::DEVICE_SERIAL_NUMBER, VR::LO, "device serial number"),
                DataElement::new(
                    tags::MANUFACTURER_DEVICE_IDENTIFIER,
                    VR::ST,
                    "device identifier",
                ),
                DataElement::new(tags::PERFORMED_STATION_NAME, VR::SH, "station name"),
                DataElement::new(
                    tags::SCHEDULED_STATION_NAME,
                    VR::SH,
                    "scheduled station name",
                ),
                DataElement::new(tags::SOURCE_MANUFACTURER, VR::LO, "source manufacturer"),
                DataElement::new(tags::SOURCE_SERIAL_NUMBER, VR::LO, "123-456789"),
                DataElement::new(tags::STATION_NAME, VR::SH, "station name"),
            ]);

            anonymize_device_profile(&mut dataset);

            let true_dataset = dicom_object::InMemDicomObject::new_empty();
            assert_eq!(dataset, true_dataset);
        }
    }

    #[test]
    fn test_anonym_profiles() {
        let add_profiles = [
            AnonymizationProfiles::Patient,
            AnonymizationProfiles::Institution,
            AnonymizationProfiles::Device,
        ];

        let mut dataset = dicom_object::InMemDicomObject::from_element_iter([
            DataElement::new(tags::PATIENT_ID, VR::LO, "012345"),
            DataElement::new(tags::PATIENT_NAME, VR::PN, "Some^Name"),
            DataElement::new(tags::PATIENT_SEX, VR::CS, "M"),
            DataElement::new(tags::ALLERGIES, VR::LO, "Some Allergies"),
            DataElement::new(tags::PATIENT_AGE, VR::AS, "063Y"),
            DataElement::new(tags::PATIENT_SEX_NEUTERED, VR::CS, "ALTERED"),
            DataElement::new(tags::PATIENT_SIZE, VR::DS, "1.6256"),
            DataElement::new(tags::PATIENT_WEIGHT, VR::DS, "68.025"),
            DataElement::new(tags::PATIENT_STATE, VR::LO, "comatose"),
            DataElement::new(tags::PREGNANCY_STATUS, VR::US, "0004"),
            DataElement::new(tags::SMOKING_STATUS, VR::CS, "YES"),
            DataElement::new(tags::INSTITUTION_ADDRESS, VR::ST, "Some address"),
            DataElement::new(
                tags::INSTITUTIONAL_DEPARTMENT_NAME,
                VR::LO,
                "Some Department Name",
            ),
            DataElement::new(tags::INSTITUTION_NAME, VR::LO, "Some Institution Name"),
            DataElement::new(tags::DEVICE_DESCRIPTION, VR::LO, "device description"),
            DataElement::new(tags::DEVICE_LABEL, VR::LO, "device label"),
            DataElement::new(tags::DEVICE_SERIAL_NUMBER, VR::LO, "device serial number"),
            DataElement::new(
                tags::MANUFACTURER_DEVICE_IDENTIFIER,
                VR::ST,
                "device identifier",
            ),
            DataElement::new(tags::PERFORMED_STATION_NAME, VR::SH, "station name"),
            DataElement::new(
                tags::SCHEDULED_STATION_NAME,
                VR::SH,
                "scheduled station name",
            ),
            DataElement::new(tags::SOURCE_MANUFACTURER, VR::LO, "source manufacturer"),
            DataElement::new(tags::SOURCE_SERIAL_NUMBER, VR::LO, "123-456789"),
            DataElement::new(tags::STATION_NAME, VR::SH, "station name"),
        ]);

        anonymize_basic_profile("TST0", &mut dataset);
        for profile in add_profiles {
            profile.apply(&mut dataset);
        }

        let true_dataset = dicom_object::InMemDicomObject::from_element_iter([
            DataElement::new(tags::PATIENT_ID, VR::LO, "TST0"),
            DataElement::new(tags::PATIENT_NAME, VR::PN, "TST0"),
            DataElement::new(tags::PATIENT_SEX, VR::CS, "O"),
            DataElement::new(tags::PATIENT_IDENTITY_REMOVED, VR::CS, "YES"),
        ]);
        assert_eq!(dataset, true_dataset);
    }

    #[test]
    fn profile_codes() {
        let profiles = [
            AnonymizationProfiles::Patient,
            AnonymizationProfiles::Institution,
            AnonymizationProfiles::Device,
        ];

        let mut dataset = dicom_object::InMemDicomObject::new_empty();

        for profile in profiles {
            update_deidentification_method_element(&mut dataset, &profile);
            match dataset.element(tags::DEIDENTIFICATION_METHOD) {
                Ok(el) => match el.to_str() {
                    Ok(val) => {
                        dbg!(&val);
                    }
                    Err(err) => println!("conversion error: {err}"),
                },
                Err(err) => println!("access error: {err}"),
            }
        }

        if let Ok(el) = dataset.element(tags::DEIDENTIFICATION_METHOD) {
            if let Ok(val) = el.to_str() {
                assert_eq!(val, "DCM_01\\DCM_02\\DCM_03");
            }
        } else {
            panic!("element not found");
        }
    }

    #[test]
    fn dicom_uid() {
        let root = "1.2.840.43.34.34.";

        let uid = generate_uid(root);
        println!("{uid}");
        assert!(uid.starts_with(&root));

        let uid = generate_uid("2.25");
        println!("{uid}");
        assert!(uid.starts_with("2.25."));
    }
}
