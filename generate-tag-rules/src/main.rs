use dicom_core::{DataDictionary, Tag};
use dicom_dictionary_std::{StandardDataDictionary, tags};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Row {
    #[serde(rename = "Attribute Name")]
    name: String,
    #[serde(rename = "Tag")]
    tag: String,
    #[serde(rename = "Retd. (from\u{a0}PS3.6)")]
    retired: String,
    #[serde(rename = "In Std. Comp. IOD (from\u{a0}PS3.3)")]
    in_standard: String,
    #[serde(rename = "Basic Prof.")]
    basic_action: String,
    #[serde(rename = "Rtn. Safe Priv. Opt.")]
    private_action: String,
    #[serde(rename = "Rtn. UIDs Opt.")]
    uid_action: String,
    #[serde(rename = "Rtn. Dev. Id. Opt.")]
    device_action: String,
    #[serde(rename = "Rtn. Inst. Id. Opt.")]
    institution_action: String,
    #[serde(rename = "Rtn. Pat. Chars. Opt.")]
    patient_action: String,
}

fn main() {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_path("generate-tag-rules/dicom/deidentify_rules.csv")
        .expect("error opening deidentify_rules.csv");

    let mut out = String::from(
        "// @generated from dicom/deidentify_rules.csv by `cargo run -p generate-tag-rules`\n\
        // DO NOT EDIT BY HAND - edit deidentify_rules.csv table and regenerate\n
        use dicom_core::{VR, Tag};
        use crate::profiles::{DeidentifyAction, DeidentifyProfile, TagRule};

        pub static TAG_RULES: &[(Tag, TagRule)] = &[\n",
    );

    let dict = StandardDataDictionary;
    let mut skipped_tags: Vec<(String, String, String)> = Vec::new();
    let mut retired_tags_count: u32 = 0;
    let mut generated_tags_count: u32 = 0;

    // intentionally skip these tags; they are modified by fnodcmanon with custom values and different rules than deidentify_rules.csv or kept unmodified for clarity purpose
    // for implementation see anonymize.rs -> fn anonymize_study() -> function calls update_uids() and update_patient_name_id()
    // MODIFICATIONS:
    // - PATIENT_NAME: ZeroLength (DICOM) -> custom dummy value
    // - PATIENT_ID: ZeroLength/DummyValue (DICOM) -> same as PATIENT_NAME

    let to_skip_tags = [
        tags::PATIENT_NAME,
        tags::PATIENT_ID,
        tags::STUDY_DESCRIPTION,
        tags::SERIES_DESCRIPTION,
        tags::STUDY_ID,
        tags::SERIES_NUMBER,
        tags::STUDY_DATE,
        tags::SERIES_DATE,
    ];

    for result in reader.deserialize() {
        let row: Row = result.expect("error parsing row");
        if row.retired == "Y" {
            retired_tags_count += 1;
        }

        if row.tag.contains("(gggg,eeee)") {
            eprintln!("row contains tag ({tag})", tag = row.tag);
            skipped_tags.push((
                "(gggg,eeee)".to_string(),
                row.name.clone(),
                "invalid tag format".to_string(),
            ));
            continue;
        }

        let tag: Tag = row.tag.parse().unwrap_or_else(|e| {
            panic!(
                "{e} ({name}), expected format GGGG,EEEE or (GGGG,EEEE)",
                name = row.name
            )
        });

        let Some(entry) = dict.by_tag(tag) else {
            eprintln!("error fetching entry for tag {tag} {name}", name = row.name);
            skipped_tags.push((tag.to_string(), row.name.clone(), "no entry".to_string()));
            continue;
        };

        if to_skip_tags.contains(&tag) {
            println!(
                "intentionally skipped tag {tag} ({name})",
                name = entry.alias
            );
            continue;
        }

        assert!(
            !row.basic_action.is_empty(),
            "Tag {tag} ({name}) missing BasicConfidentiality action entry",
            name = entry.alias
        );

        let Some(vr) = entry.vr.exact() else {
            eprintln!(
                "error fetching exact VR for tag {tag} {name}",
                name = row.name
            );
            skipped_tags.push((tag.to_string(), row.name.clone(), "no exact VR".to_string()));
            continue;
        };

        let mut pairs: Vec<String> = Vec::new();
        for (profile, code) in [
            // optional profiles take priority over BasicConfidentiality and RetainRetired
            // intentionally order as in the DICOM standard Table E.1-1. Application Level
            // Confidentiality Profile Attributes
            // https://dicom.nema.org/medical/dicom/current/output/chtml/part15/chapter_E.html
            ("DeidentifyProfile::RetainUID", &row.uid_action),
            (
                "DeidentifyProfile::RetainDeviceIdentity",
                &row.device_action,
            ),
            (
                "DeidentifyProfile::RetainInstitutionIdentity",
                &row.institution_action,
            ),
            (
                "DeidentifyProfile::RetainPatientCharacteristics",
                &row.patient_action,
            ),
            ("DeidentifyProfile::RetainRetired", &row.retired),
            ("DeidentifyProfile::BasicConfidentiality", &row.basic_action),
        ] {
            match code_to_action(profile, code) {
                Ok(Some(action)) => pairs.push(format!("({profile}, {action})")),
                Ok(None) => {} // tag has no action (eg. "-") for current profile
                Err(bad_action) => panic!(
                    "Unknown action code `{bad_action}` for tag {tag} ({name}) and profile {profile}",
                    name = entry.alias
                ),
            }
        }

        out += &format!(
            "(Tag(0x{group:04X},0x{elem:04X}), TagRule {{vr: VR::{vr}, actions: &[{pairs}]}}),",
            group = tag.0,
            elem = tag.1,
            pairs = pairs.join(", ")
        );
        generated_tags_count += 1;
    }
    out += "];\n";

    println!(
        "\ngenerated {} tags including {} retired tags",
        generated_tags_count, retired_tags_count,
    );
    println!(
        "skipped {} invalid tags:\n{skipped_tags:?}",
        skipped_tags.len()
    );

    fs::write("src/tag_rules_generated.rs", out)
        .unwrap_or_else(|e| panic!("error generating tag_rules_generated.rs: {e}"));
    println!("generated src/tag_rules_generated.rs");

    std::process::Command::new("rustfmt")
        .arg("src/tag_rules_generated.rs")
        .status()
        .unwrap_or_else(|e| panic!("error formatting src/tag_rules_generated.rs: {e}"));
}

fn code_to_action(profile: &str, mut code: &str) -> Result<Option<&'static str>, String> {
    if code.contains('/') {
        code = code.split_once('/').unwrap().0;
    }
    match (profile, code) {
        (_, "Z") => Ok(Some("DeidentifyAction::ZeroLength")),
        (_, "X") => Ok(Some("DeidentifyAction::Remove")),
        (_, "D") => Ok(Some("DeidentifyAction::DummyValue")),
        (_, "K") => Ok(Some("DeidentifyAction::Keep")),
        (_, "C") => Ok(Some("DeidentifyAction::Clean")),
        (_, "U") => Ok(Some("DeidentifyAction::UIDReplace")),
        ("DeidentifyProfile::RetainRetired", "Y") => Ok(Some("DeidentifyAction::Keep")),
        (_, "-") | (_, "N") => Ok(None),
        (_, other) => Err(other.to_string()),
    }
}
