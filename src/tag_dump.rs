use serde::Serialize;

#[derive(Serialize)]
pub struct StudyTags {
    patient_id: String,
    pseudoname: String,
    study_uid: String,
}

impl StudyTags {
    pub fn new(patient_id: String, pseudoname: String, study_uid: String) -> Self {
        StudyTags {
            patient_id,
            pseudoname,
            study_uid,
        }
    }
}
