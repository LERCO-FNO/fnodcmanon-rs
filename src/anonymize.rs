// use std::error::Error;
use clap::ValueEnum;
use dicom_core::VR;
use dicom_dictionary_std::tags;
use dicom_object::{InMemDicomObject, open_file};
use rand::{Rng, distr::Alphanumeric};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use crate::tag_dump::StudyTags;
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
    uid_root: String,
}

impl DicomAnonymizer {
    pub fn new(
        prefix: String,
        method: PseudonameMethod,
        profiles: HashSet<AnonymizationProfiles>,
        uid_root: String,
    ) -> Self {
        Self {
            prefix,
            pseudoname_method: method,
            additional_profiles: profiles,
            uid_root,
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
        let study_uid = generate_uid(&self.uid_root);
        let study_dir = create_study_dir(output_dir, &study_uid)?;

        let mut series_uid_map: HashMap<String, String> = HashMap::new();
        for file in dicom_files {
            let mut dataset = open_file(&file)?;

            anonymize_basic_profile(&self.pseudoname, &mut dataset);
            update_deidentification_method_element(&mut dataset, "DCM_113100".to_string());

            for profile in &self.additional_profiles {
                profile.apply(&mut dataset);
                update_deidentification_method_element(
                    &mut dataset,
                    profile.profile_as_string_code(),
                );
            }

            // TODO: possibly improve error handling
            update_uids(
                &mut dataset,
                &self.uid_root,
                &study_uid,
                &mut series_uid_map,
            )?;

            let filepath = study_dir.join(file.file_name().unwrap());

            if filepath.exists() {
                log::warn!("file {} exists, overwriting", filepath.display());
            }

            dataset.write_to_file(filepath)?;
        }

        log::debug!(
            "old id {0}, old name {1}, new id/name {2}",
            self.old_id,
            self.old_name,
            self.pseudoname
        );

        Ok(())
    }

    pub fn run_anonymization(
        &mut self,
        input_dir: PathBuf,
        output_dir: &Path,
    ) -> Result<Vec<StudyTags>, Box<dyn std::error::Error>> {
        let dicom_dirs = find_dicom_dirs(&input_dir)?;

        let mut study_tags: Vec<StudyTags> = Vec::new();
        for dir in dicom_dirs {
            let dicom_files = match get_dicom_files(&dir) {
                Some(files) => files,
                None => continue,
            };

            self.get_basic_tags(dicom_files.first().unwrap())?;
            self.set_pseudoname();
            self.anonymize_study(dicom_files, output_dir)?;

            study_tags.push(StudyTags::new(
                self.old_id.clone(),
                self.pseudoname.clone(),
                self.study_uid.clone(),
            ));
        }

        Ok(study_tags)
    }
}

// TODO: unify with the specification at https://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_7050.html
// more at https://dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_D.html#DCM_113100
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum AnonymizationProfiles {
    /// Deidentify patient tag values (DCM_113108)
    Patient,
    /// Deidentify device tag values (DCM_113109)
    Device,
    /// Deidentify institution tag values (DCM_113112)
    Institution,
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
        match self {
            Self::Patient => "DCM_113108",
            Self::Device => "DCM_113109",
            Self::Institution => "DCM_113112",
        }
        .to_string()
        // method.to_string()
    }
}

fn anonymize_basic_profile(pseudoname: &str, dataset: &mut InMemDicomObject) {
    dataset.put_str(tags::PATIENT_ID, VR::LO, pseudoname);
    dataset.put_str(tags::PATIENT_NAME, VR::PN, pseudoname);
    dataset.put_str(tags::PATIENT_SEX, VR::CS, String::from("O"));
    dataset.put_str(tags::PATIENT_IDENTITY_REMOVED, VR::CS, "YES");
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

fn generate_random_string() -> String {
    let mut rng = rand::rng();
    (0..10).map(|_| rng.sample(Alphanumeric) as char).collect()
}

fn generate_uid(root: &str) -> String {
    let uid = uuid::Uuid::now_v7().to_u128_le();
    if root.ends_with(".") {
        format!("{0}{1}", root, uid)
    } else {
        format!("{0}.{1}", root, uid)
    }
}

fn update_deidentification_method_element(dataset: &mut InMemDicomObject, profile_code: String) {
    let mut element_val = match dataset.element_opt(tags::DEIDENTIFICATION_METHOD) {
        Ok(Some(el)) => el.to_str().map(|s| s.to_string()).unwrap_or_default(),
        _ => String::new(),
    };

    if !element_val.is_empty() {
        element_val.push('\\');
    }

    log::debug!("adding deidentification code {0}", profile_code);
    element_val.push_str(&profile_code);

    dataset.put_str(tags::DEIDENTIFICATION_METHOD, VR::LO, element_val);
}

fn update_uids(
    dataset: &mut InMemDicomObject,
    uid_root: &str,
    study_uid: &str,
    series_uid_map: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let old_series_uid = dataset
        .element(tags::SERIES_INSTANCE_UID)?
        .to_str()?
        .to_string();

    let new_series_uid = series_uid_map
        .entry(old_series_uid)
        .or_insert_with(|| generate_uid(uid_root))
        .clone();

    dataset.put_str(tags::STUDY_INSTANCE_UID, VR::UI, study_uid);
    dataset.put_str(tags::SERIES_INSTANCE_UID, VR::UI, new_series_uid);
    dataset.put_str(tags::SOP_INSTANCE_UID, VR::UI, generate_uid(uid_root));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_core::DataElement;

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
                DataElement::new(tags::PATIENT_AGE, VR::AS, "099Y"),
                DataElement::new(tags::PATIENT_SEX_NEUTERED, VR::CS, "ALTERED"),
                DataElement::new(tags::PATIENT_SIZE, VR::DS, "0.0"),
                DataElement::new(tags::PATIENT_WEIGHT, VR::DS, "0.0"),
                DataElement::new(tags::PATIENT_STATE, VR::LO, "some_state"),
                DataElement::new(tags::PREGNANCY_STATUS, VR::US, "0000"),
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
    fn profile_codes() {
        let profiles = [
            AnonymizationProfiles::Patient,
            AnonymizationProfiles::Institution,
            AnonymizationProfiles::Device,
        ];

        let mut dataset = dicom_object::InMemDicomObject::new_empty();

        for profile in profiles {
            update_deidentification_method_element(&mut dataset, profile.profile_as_string_code());
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
    fn change_uid() -> Result<(), Box<dyn std::error::Error>> {
        let mut datasets1 = [
            dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::STUDY_INSTANCE_UID, VR::UI, "1.2"),
                DataElement::new(tags::SERIES_INSTANCE_UID, VR::UI, "1.2.1"),
                DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.2.1.1"),
            ]),
            dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::STUDY_INSTANCE_UID, VR::UI, "1.2"),
                DataElement::new(tags::SERIES_INSTANCE_UID, VR::UI, "1.2.1"),
                DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.2.1.2"),
            ]),
            dicom_object::InMemDicomObject::from_element_iter([
                DataElement::new(tags::STUDY_INSTANCE_UID, VR::UI, "1.2"),
                DataElement::new(tags::SERIES_INSTANCE_UID, VR::UI, "1.2.2"),
                DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.2.1.1"),
            ]),
        ];

        let root = "1.2.3.".to_string();
        let study_uid = generate_uid(&root);
        let mut series_uid_map: HashMap<String, String> = HashMap::new();

        for ds in &mut datasets1 {
            update_uids(ds, &root, &study_uid, &mut series_uid_map)?;
            dbg!("{}", ds);
        }

        assert_eq!(series_uid_map.len(), 2);

        Ok(())
    }

    #[test]
    fn test_uid() {
        let root = "1.2.3";
        let uid = generate_uid(root);
        println!("{uid}");
        assert!(uid.starts_with(root));
    }
}
