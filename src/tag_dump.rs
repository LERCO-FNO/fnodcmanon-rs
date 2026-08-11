use csv::Writer;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct StudyTags {
    patient_id: String,
    pseudoname: String,
    old_study_uid: String,
    new_study_uid: String,
}

impl StudyTags {
    pub fn new(
        patient_id: String,
        pseudoname: String,
        old_study_uid: String,
        new_study_uid: String,
    ) -> Self {
        StudyTags {
            patient_id,
            pseudoname,
            old_study_uid,
            new_study_uid,
        }
    }
}

pub fn write_tags<P: AsRef<Path>>(path: P, tags: Vec<StudyTags>) -> Result<(), csv::Error> {
    let mut writer = Writer::from_path(path)?;
    for study in tags {
        writer.serialize(study)?;
    }
    writer.flush()?;

    Ok(())
}
