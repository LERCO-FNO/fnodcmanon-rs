use dicom_core::{Tag, VR};
use dicom_dictionary_std::tags;
use dicom_object::{InMemDicomObject, open_file};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::tag_rules_generated::TAG_RULES;
use crate::utils::{create_study_dir, find_dicom_dirs, generate_random_string, get_dicom_files};
use crate::{error::AnonymizerError, tag_dump::StudyTags};
use crate::{
    profiles::{DeidentifyProfile, TagRule},
    uid::UidMap,
};

static TAG_RULES_MAP: LazyLock<HashMap<Tag, TagRule>> = LazyLock::new(|| {
    log::debug!(
        "initializing TAG_RULES_MAP with {} entries",
        TAG_RULES.len()
    );
    TAG_RULES.iter().copied().collect()
});

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
    old_study_uid: String,
    new_study_uid: String,
    active_profiles: HashSet<DeidentifyProfile>,
    uid_root: String,
}

impl DicomAnonymizer {
    pub fn new(
        prefix: String,
        pseudoname_method: PseudonameMethod,
        active_profiles: HashSet<DeidentifyProfile>,
        uid_root: String,
    ) -> Self {
        Self {
            prefix,
            pseudoname_method,
            active_profiles,
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

    fn get_basic_tags(&mut self, filepath: &Path) -> Result<(), AnonymizerError> {
        let dicom_obj = open_file(filepath)?;

        self.old_id = dicom_obj.element(tags::PATIENT_ID)?.to_str()?.to_string();
        self.old_name = dicom_obj.element(tags::PATIENT_NAME)?.to_str()?.to_string();
        self.old_study_uid = dicom_obj
            .element(tags::STUDY_INSTANCE_UID)?
            .to_str()?
            .to_string();

        Ok(())
    }

    fn anonymize_study(
        &mut self,
        dicom_files: Vec<PathBuf>,
        output_dir: &Path,
    ) -> Result<(), AnonymizerError> {
        let mut uid_map = UidMap::new(self.uid_root.clone());
        self.new_study_uid = uid_map.get_or_insert(&self.old_study_uid);

        let study_dir = create_study_dir(output_dir, &self.new_study_uid)?;

        for file in dicom_files {
            let mut dataset = open_file(&file)?;

            // collect dataset's tags; .tags() is immutable borrow over iterable; for-loop below modifies dataset
            let tags: Vec<Tag> = dataset.tags().collect();

            for tag in tags {
                let Some(rule) = TAG_RULES_MAP.get(&tag) else {
                    continue;
                };

                let action = rule.resolve_action(&self.active_profiles);
                rule.apply_action(&mut dataset, tag, action, &mut uid_map)?;
            }

            update_patient_name_id(&mut dataset, &self.pseudoname);
            update_deidentification_method_element(&mut dataset, &self.active_profiles);

            let filepath = study_dir.join(file.file_name().unwrap());

            if filepath.exists() {
                log::warn!("file {} exists, overwriting", filepath.display());
            }

            dataset.write_to_file(filepath)?;
        }

        log::debug!(
            "old id {0} -> new id/name {1}",
            self.old_id,
            self.pseudoname
        );

        Ok(())
    }

    pub fn run_anonymization(
        &mut self,
        input_dir: PathBuf,
        output_dir: &Path,
    ) -> Result<Vec<StudyTags>, AnonymizerError> {
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
                self.old_study_uid.clone(),
                self.new_study_uid.clone(),
            ));
        }

        Ok(study_tags)
    }
}

fn update_patient_name_id(dataset: &mut InMemDicomObject, pseudoname: &str) {
    dataset.put_str(tags::PATIENT_ID, VR::PN, pseudoname);
    dataset.put_str(tags::PATIENT_NAME, VR::PN, pseudoname);
}

fn update_deidentification_method_element(
    dataset: &mut InMemDicomObject,
    active_profiles: &HashSet<DeidentifyProfile>,
) {
    let profile_codes = active_profiles
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<String>>()
        .join("\\");

    dataset.put_str(tags::DEIDENTIFICATION_METHOD, VR::LO, profile_codes);
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::uid::generate_uid;
    use dicom_core::DataElement;

    #[allow(
        deprecated,
        reason = "suppress warning `use of deprecated constant...`"
    )]
    fn test_datasets() -> [InMemDicomObject; 4] {
        [
            InMemDicomObject::from_element_iter([
                DataElement::new(tags::PATIENT_ID, VR::PN, "012345"),
                DataElement::new(tags::PATIENT_NAME, VR::PN, "Some^Name"),
                DataElement::new(tags::PATIENT_AGE, VR::AS, "091Y"),
                DataElement::new(tags::DEVICE_DESCRIPTION, VR::LO, "CT1"),
                DataElement::new(tags::INSTITUTION_ADDRESS, VR::ST, "Hospital1"),
                DataElement::new(tags::STUDY_INSTANCE_UID, VR::UI, "1.2"),
                DataElement::new(tags::SERIES_INSTANCE_UID, VR::UI, "1.2.1"),
                DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.2.1.1"),
                DataElement::new(tags::ACQUISITION_COMMENTS, VR::LT, "some comments here"),
            ]),
            InMemDicomObject::from_element_iter([
                DataElement::new(tags::PATIENT_ID, VR::PN, "012345"),
                DataElement::new(tags::PATIENT_NAME, VR::PN, "Some^Name"),
                DataElement::new(tags::PATIENT_AGE, VR::AS, "091Y"),
                DataElement::new(tags::DEVICE_DESCRIPTION, VR::LO, "CT1"),
                DataElement::new(tags::INSTITUTION_ADDRESS, VR::ST, "Hospital1"),
                DataElement::new(tags::STUDY_INSTANCE_UID, VR::UI, "1.2"),
                DataElement::new(tags::SERIES_INSTANCE_UID, VR::UI, "1.2.1"),
                DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.3.1.1"),
                DataElement::new(tags::ACQUISITION_COMMENTS, VR::LT, "some comments"),
            ]),
            InMemDicomObject::from_element_iter([
                DataElement::new(tags::PATIENT_ID, VR::PN, "012345"),
                DataElement::new(tags::PATIENT_NAME, VR::PN, "Some^Name"),
                DataElement::new(tags::PATIENT_AGE, VR::AS, "091Y"),
                DataElement::new(tags::DEVICE_DESCRIPTION, VR::LO, "CT1"),
                DataElement::new(tags::INSTITUTION_ADDRESS, VR::ST, "Hospital1"),
                DataElement::new(tags::STUDY_INSTANCE_UID, VR::UI, "1.2"),
                DataElement::new(tags::SERIES_INSTANCE_UID, VR::UI, "1.2.2"),
                DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.2.2.2"),
                DataElement::new(tags::ACQUISITION_COMMENTS, VR::LT, "some comments"),
            ]),
            InMemDicomObject::from_element_iter([
                DataElement::new(tags::PATIENT_ID, VR::PN, "98765"),
                DataElement::new(tags::PATIENT_NAME, VR::PN, "Other^Name"),
                DataElement::new(tags::PATIENT_AGE, VR::AS, "025Y"),
                DataElement::new(tags::DEVICE_DESCRIPTION, VR::LO, "CT2"),
                DataElement::new(tags::INSTITUTION_ADDRESS, VR::ST, "Hospital2"),
                DataElement::new(tags::STUDY_INSTANCE_UID, VR::UI, "1.3"),
                DataElement::new(tags::SERIES_INSTANCE_UID, VR::UI, "1.3.1"),
                DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.3.1.1"),
                DataElement::new(tags::ACQUISITION_COMMENTS, VR::LT, "some comments"),
            ]),
        ]
    }

    #[test]
    #[allow(
        deprecated,
        reason = "suppress warning `use of deprecated constant...`"
    )]
    fn deidentify() -> Result<(), Box<dyn std::error::Error>> {
        let mut anonymizer = DicomAnonymizer::new(
            "TS_".into(),
            PseudonameMethod::RandomString,
            HashSet::from_iter([
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyProfile::RetainPatientCharacteristics,
                DeidentifyProfile::RetainRetired,
            ]),
            "2.25".into(),
        );

        anonymizer.set_pseudoname();
        anonymizer.old_study_uid = generate_uid(&anonymizer.uid_root);

        let datasets = test_datasets();

        let pseudoname = generate_random_string();

        let mut uid_map = UidMap::new(anonymizer.uid_root.clone());
        for mut ds in datasets {
            for tag in ds.tags().collect::<Vec<Tag>>() {
                let Some(rule) = TAG_RULES_MAP.get(&tag) else {
                    continue;
                };

                let action = rule.resolve_action(&anonymizer.active_profiles);
                println!("applying: tag={tag}, rule={rule:?}, action={action:?}");
                rule.apply_action(&mut ds, tag, action, &mut uid_map)?;
            }

            update_patient_name_id(&mut ds, &pseudoname);
            update_deidentification_method_element(&mut ds, &anonymizer.active_profiles);
            let true_ds = InMemDicomObject::from_element_iter([
                DataElement::new(tags::PATIENT_ID, VR::PN, pseudoname.clone()),
                DataElement::new(tags::PATIENT_NAME, VR::PN, pseudoname.clone()),
                DataElement::new(tags::PATIENT_AGE, VR::AS, "091Y"),
            ]);

            assert_eq!(
                true_ds.element(tags::PATIENT_ID).unwrap(),
                ds.element(tags::PATIENT_ID).unwrap()
            );
            assert_eq!(
                true_ds.element(tags::PATIENT_NAME).unwrap(),
                ds.element(tags::PATIENT_NAME).unwrap()
            );

            assert!(ds.get(tags::ACQUISITION_COMMENTS).is_some());

            let true_deident_method = ds
                .element(tags::DEIDENTIFICATION_METHOD)
                .unwrap()
                .to_str()
                .unwrap();

            let missing_profile_codes: Vec<_> = anonymizer
                .active_profiles
                .iter()
                .map(|p| (*p, String::from(*p)))
                .filter(|(_, code)| !true_deident_method.contains(code))
                .collect();
            assert!(
                missing_profile_codes.is_empty(),
                "DeidentificationMethod '{true_deident_method}' missing codes for: {missing_profile_codes:?}"
            );
        }

        Ok(())
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

        let active_profiles = HashSet::from([DeidentifyProfile::BasicConfidentiality]);
        let root = "1.2.3.".to_string();
        // let study_uid = generate_uid(&root);
        let mut uid_map = UidMap::new(root.clone());

        for ds in &mut datasets1 {
            for tag in ds.tags().collect::<Vec<Tag>>() {
                let Some(rule) = TAG_RULES_MAP.get(&tag) else {
                    continue;
                };
                let action = rule.resolve_action(&active_profiles);
                println!("applying tag={tag}, rule={rule:?}, action={action:?}");
                rule.apply_action(ds, tag, action, &mut uid_map)?;

                dbg!("{}", &ds);
            }
        }

        assert_eq!(uid_map.len(), 2);

        Ok(())
    }
}
