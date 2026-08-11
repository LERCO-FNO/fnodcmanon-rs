use std::collections::HashSet;

use clap::ValueEnum;
use dicom_core::value::Value;
use dicom_core::{DataElement, PrimitiveValue, Tag, VR};
use dicom_dictionary_std::tags;
use dicom_object::InMemDicomObject;

use crate::error::AnonymizerError;
use crate::uid::UidMap;
// use crate::utils::generate_random_string;

// add tag profile groups here, definitions and action to take here

// --- ACTIONS ---
// https://dicom.nema.org/medical/dicom/current/output/chtml/part15/chapter_E.html
// Table E.1-1 Application Level Confidentiality Profiel Attributes

// Z: replace with a zero length value, or a non-zero length value that may be a dummy value and consistent with the VR
// X: remove Attribute, and if the Attribute is a Sequence, remove all Sequence Items and their contained Attributes
// D: replace with a non-zero length value that may be a dummy value and consistent with the VR
// K: keep (unchanged for non-Sequence Attributes, cleaned for Sequences)
// C: clean, that is replace with values of similar meaning known not to contain identifying information and consistent with the VR
// U: replace with a non-zero length UID that is internally consistent within a set of Instances
// Z/D: Z unless D is required to maintain IOD conformance (Type 2 versus Type 1)
// X/Z: X unless Z is required to maintain IOD conformance (Type 3 versus Type 2)
// X/D: X unless D is required to maintain IOD conformance (Type 3 versus Type 1)
// X/Z/D: X unless Z or D is required to maintain IOD conformance (Type 3 versus Type 2 versus Type 1)
// X/Z/U*: X unless Z or replacement of contained instance UIDs (U) is required to maintain IOD conformance (Type 3 versus Type 2 versus Type 1 sequences containing UID references)

// --- Rust equivalent enum variants
// Z - ZeroLength
// X - Remove
// D - DummyValue
// K - Keep
// C - Clean
// U - UIDReplace
//TODO: possibly decision logic for multiple actions
//example: X/Z/D - X (Remove) unless Z (ZeroLength) or D (DummyValue) is requried to maintain IOD
//conformance (Type 3 vs Type 2 vs Type 1)

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, ValueEnum)]
pub enum DeidentifyProfile {
    #[value(skip)] // never show as CLI option, is always active/present
    BasicConfidentiality,
    /// Retain Patient Characteristics Option
    RetainPatientCharacteristics,
    /// Retain Device Identity Option
    RetainDeviceIdentity,
    /// Retain Institution Identity Option
    RetainInstitutionIdentity,
    /// Retain UID Option
    RetainUID,
    /// Retain Retired Tags Option
    RetainRetired,
}

impl From<DeidentifyProfile> for String {
    fn from(value: DeidentifyProfile) -> Self {
        match value {
            DeidentifyProfile::BasicConfidentiality => "DCM_113100",
            DeidentifyProfile::RetainPatientCharacteristics => "DCM_113108",
            DeidentifyProfile::RetainDeviceIdentity => "DCM_113109",
            DeidentifyProfile::RetainUID => "DCM_113110",
            DeidentifyProfile::RetainInstitutionIdentity => "DCM_113112",
            DeidentifyProfile::RetainRetired => "RETAIN_RETIRED",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DeidentifyAction {
    ZeroLength, // Z
    Remove,     // X
    DummyValue, // D
    Keep,       // K
    Clean,      // C
    UIDReplace, // U
}

#[derive(Debug, Clone, Copy)]
pub struct TagRule {
    pub vr: VR,
    pub actions: &'static [(DeidentifyProfile, DeidentifyAction)],
}

impl TagRule {
    pub fn resolve_action(&self, active_profiles: &HashSet<DeidentifyProfile>) -> DeidentifyAction {
        self.actions
            .iter()
            .find(|(profile, _)| active_profiles.contains(profile)) // optional profiles take priority over Basic profile
            .map(|(_, action)| *action)
            .unwrap_or(DeidentifyAction::Keep) // keep the tag if no action is found for requested profiles
    }

    pub fn apply_action(
        &self,
        ds: &mut InMemDicomObject,
        tag: Tag,
        action: DeidentifyAction,
        uid_map: &mut UidMap,
    ) -> Result<(), AnonymizerError> {
        match (action, self.vr) {
            (DeidentifyAction::Keep, _) => Ok(()),
            (DeidentifyAction::Remove, _) => {
                ds.remove_element(tag);
                Ok(())
            }
            (DeidentifyAction::ZeroLength, _) => {
                ds.put(DataElement::new(tag, self.vr, PrimitiveValue::Empty));
                Ok(())
            }
            (DeidentifyAction::DummyValue, _) => {
                // TODO: add dummy value generation per VR or dummy_value override per tag as part of deidentify_rules.csv (probably the later for better comparison of changes though harder to maintain)
                // - some tags may have multiple acceptable dummy values and may be modality dependent
                // - probably instead use a fixed dummy_value for all modalities
                match (self.vr, tag) {
                    (_, tags::CONTENT_SEQUENCE) => {
                        // TODO: implement walk_item()
                        // Ok(())

                        ds.update_value(tag, |value| {
                            if let Value::Sequence(seq) = value {
                                for item in seq.items_mut() {
                                    if let Err(e) = walk_item(item, uid_map) {
                                        log::error!("failed to walk sequence: {e}");
                                    }
                                }
                            }
                        });
                        Ok(())
                    }
                    // intentionaly skip other sequences as they may not be part of
                    // deidentify_rules.csv
                    // TODO: verify!!
                    (VR::SQ, _) => Ok(()),
                    (_, _) => {
                        log::debug!(
                            "No implemented behaviour for tag {tag}, vr {vr}, action {action:?}",
                            vr = self.vr
                        );
                        // TODO:: implement dummy value generation/assignment for other VRs!
                        // ds.put_str(tag, self.vr, generate_random_string());
                        Ok(())
                    }
                }
            }
            (DeidentifyAction::Clean, _) => {
                log::warn!("Rule defined, but DeidentifyAction::Clean not implemented yet");
                Ok(())
            }
            (DeidentifyAction::UIDReplace, VR::UI) => {
                let old_uid = ds.element(tag)?.to_str()?;
                let new_uid = uid_map.get_or_insert(&old_uid);
                ds.put_str(tag, self.vr, new_uid);
                Ok(())
            }
            (DeidentifyAction::UIDReplace, _) => {
                log::warn!(
                    "non UI VR, tag {tag}, vr {vr}, action {action:?}",
                    vr = self.vr
                );
                Ok(())
            }
        }
    }
}

fn walk_item(item: &mut InMemDicomObject, uid_map: &mut UidMap) -> Result<(), AnonymizerError> {
    if let Some(concept_item) = item
        .element(tags::CONCEPT_NAME_CODE_SEQUENCE)
        .ok()
        .and_then(|e| e.items())
        .and_then(|i| i.first())
    {
        let code_value = concept_item.element(tags::CODE_VALUE)?.to_str()?;

        // TODO: future: possibly add deidentification of other items based on CodeValue + possibly
        // ValueType
        match code_value.as_ref() {
            "110180" | "113769" | "112002" => modify_sequence_uid_item(item, uid_map),
            _ => {}
        }
    }

    item.update_value(tags::CONTENT_SEQUENCE, |value| {
        if let Value::Sequence(seq) = value {
            for subitem in seq.items_mut() {
                if let Err(e) = walk_item(subitem, uid_map) {
                    log::error!("failed to walk subitem: {e}");
                }
            }
        }
    });

    Ok(())
}

fn modify_sequence_uid_item(item: &mut InMemDicomObject, uid_map: &mut UidMap) {
    let old_uid = item.element(tags::UID).ok().and_then(|e| e.to_str().ok());
    if let Some(old_uid) = old_uid {
        let old_uid = old_uid.trim();
        let new_uid = uid_map.get_or_insert(old_uid);
        item.put_str(tags::UID, VR::UI, new_uid);
    }
}

#[allow(
    deprecated,
    reason = "suppress warning `use of deprecated constant...`"
)]
pub static RETIRED_TAGS: &[Tag] = &[
    tags::COMMAND_LENGTH_TO_END,
    tags::COMMAND_RECOGNITION_CODE,
    tags::INITIATOR,
    tags::RECEIVER,
    tags::FIND_LOCATION,
    tags::NUMBER_OF_MATCHES,
    tags::RESPONSE_SEQUENCE_NUMBER,
    tags::DIALOG_RECEIVER,
    tags::TERMINAL_TYPE,
    tags::MESSAGE_SET_ID,
    tags::END_MESSAGE_ID,
    tags::DISPLAY_FORMAT,
    tags::PAGE_POSITION_ID,
    tags::TEXT_FORMAT_ID,
    tags::NORMAL_REVERSE,
    tags::ADD_GRAY_SCALE,
    tags::BORDERS,
    tags::COPIES,
    tags::COMMAND_MAGNIFICATION_TYPE,
    tags::ERASE,
    tags::PRINT,
    tags::OVERLAYS,
    tags::MRDR_DIRECTORY_RECORD_OFFSET,
    tags::NUMBER_OF_REFERENCES,
    tags::LENGTH_TO_END,
    tags::RECOGNITION_CODE,
    tags::OVERLAY_DATE,
    tags::CURVE_DATE,
    tags::OVERLAY_TIME,
    tags::CURVE_TIME,
    tags::DATA_SET_TYPE,
    tags::DATA_SET_SUBTYPE,
    tags::NUCLEAR_MEDICINE_SERIES_TYPE,
    tags::NETWORK_ID,
    tags::REFERENCED_RESULTS_SEQUENCE,
    tags::REFERENCED_OVERLAY_SEQUENCE,
    tags::REFERENCED_CURVE_SEQUENCE,
    tags::LOSSY_IMAGE_COMPRESSION_RETIRED,
    tags::TRANSDUCER_POSITION,
    tags::TRANSDUCER_ORIENTATION,
    tags::ANATOMIC_STRUCTURE,
    tags::ANATOMIC_STRUCTURE_SPACE_OR_REGION_SEQUENCE,
    tags::TRANSDUCER_POSITION_SEQUENCE,
    tags::TRANSDUCER_POSITION_MODIFIER_SEQUENCE,
    tags::TRANSDUCER_ORIENTATION_SEQUENCE,
    tags::TRANSDUCER_ORIENTATION_MODIFIER_SEQUENCE,
    tags::ANATOMIC_STRUCTURE_SPACE_OR_REGION_CODE_SEQUENCE_TRIAL,
    tags::ANATOMIC_PORTAL_OF_ENTRANCE_CODE_SEQUENCE_TRIAL,
    tags::ANATOMIC_APPROACH_DIRECTION_CODE_SEQUENCE_TRIAL,
    tags::ANATOMIC_PERSPECTIVE_DESCRIPTION_TRIAL,
    tags::ANATOMIC_PERSPECTIVE_CODE_SEQUENCE_TRIAL,
    tags::ANATOMIC_LOCATION_OF_EXAMINING_INSTRUMENT_DESCRIPTION_TRIAL,
    tags::ANATOMIC_LOCATION_OF_EXAMINING_INSTRUMENT_CODE_SEQUENCE_TRIAL,
    tags::ANATOMIC_STRUCTURE_SPACE_OR_REGION_MODIFIER_CODE_SEQUENCE_TRIAL,
    tags::ON_AXIS_BACKGROUND_ANATOMIC_STRUCTURE_CODE_SEQUENCE_TRIAL,
    tags::IDENTIFYING_COMMENTS,
    tags::OTHER_PATIENT_I_DS,
    tags::INSURANCE_PLAN_IDENTIFICATION,
    tags::MEDICAL_RECORD_LOCATOR,
    tags::ETHNIC_GROUP,
    tags::CAD_FILE_FORMAT,
    tags::COMPONENT_REFERENCE_SYSTEM,
    tags::MATERIAL_PROPERTIES_FILE_FORMAT_RETIRED,
    tags::RADIONUCLIDE,
    tags::ENERGY_WINDOW_CENTERLINE,
    tags::ENERGY_WINDOW_TOTAL_WIDTH,
    tags::THERAPY_TYPE,
    tags::THERAPY_DESCRIPTION,
    tags::HARDCOPY_CREATION_DEVICE_ID,
    tags::HARDCOPY_DEVICE_MANUFACTURER,
    tags::HARDCOPY_DEVICE_SOFTWARE_VERSION,
    tags::HARDCOPY_DEVICE_MANUFACTURER_MODEL_NAME,
    tags::ANGULAR_POSITION,
    tags::ROTATION_OFFSET,
    tags::UPPER_LOWER_PIXEL_VALUES,
    tags::ACQUISITION_COMMENTS,
    tags::POSTPROCESSING_FUNCTION,
    tags::DYNAMIC_RANGE,
    tags::TOTAL_GAIN,
    tags::IMAGE_TRANSFORMATION_MATRIX,
    tags::IMAGE_TRANSLATION_VECTOR,
    tags::DOPPLER_SAMPLE_VOLUME_X_POSITION_RETIRED,
    tags::DOPPLER_SAMPLE_VOLUME_Y_POSITION_RETIRED,
    tags::TM_LINE_POSITION_X0_RETIRED,
    tags::TM_LINE_POSITION_Y0_RETIRED,
    tags::TM_LINE_POSITION_X1_RETIRED,
    tags::TM_LINE_POSITION_Y1_RETIRED,
    tags::PARALLEL_REDUCTION_FACTOR_IN_PLANE_RETIRED,
    tags::BULK_MOTION_STATUS,
    tags::CHEMICAL_SHIFT_MINIMUM_INTEGRATION_LIMIT_IN_HZ,
    tags::CHEMICAL_SHIFT_MAXIMUM_INTEGRATION_LIMIT_IN_HZ,
    tags::ESTIMATED_DOSE_SAVING,
    tags::ISOTOPE_NUMBER,
    tags::PHASE_NUMBER,
    tags::INTERVAL_NUMBER,
    tags::TIME_SLOT_NUMBER,
    tags::ANGLE_NUMBER,
    tags::OVERLAY_NUMBER,
    tags::CURVE_NUMBER,
    tags::LUT_NUMBER,
    tags::IMAGE_POSITION,
    tags::IMAGE_ORIENTATION,
    tags::LOCATION,
    tags::IMAGE_GEOMETRY_TYPE,
    tags::MASKING_IMAGE,
    tags::REPORT_NUMBER,
    tags::SERIES_IN_STUDY,
    tags::ACQUISITIONS_IN_SERIES,
    tags::IMAGES_IN_SERIES,
    tags::ACQUISITIONS_IN_STUDY,
    tags::IMAGES_IN_STUDY,
    tags::REFERENCE,
    tags::OTHER_STUDY_NUMBERS,
    tags::MODIFYING_DEVICE_ID,
    tags::MODIFIED_IMAGE_ID,
    tags::MODIFIED_IMAGE_DATE,
    tags::MODIFYING_DEVICE_MANUFACTURER,
    tags::MODIFIED_IMAGE_TIME,
    tags::MODIFIED_IMAGE_DESCRIPTION,
    tags::ORIGINAL_IMAGE_IDENTIFICATION,
    tags::ORIGINAL_IMAGE_IDENTIFICATION_NOMENCLATURE,
    tags::LENS_CONSTANT_DESCRIPTION,
    tags::OPHTHALMIC_AXIAL_LENGTH_ACQUISITION_METHOD_CODE_SEQUENCE,
    tags::OPHTHALMIC_AXIAL_LENGTH_QUALITY_METRIC_TYPE_CODE_SEQUENCE,
    tags::OPHTHALMIC_AXIAL_LENGTH_QUALITY_METRIC_TYPE_DESCRIPTION,
    tags::IMAGE_DIMENSIONS,
    tags::PLANES,
    tags::IMAGE_FORMAT,
    tags::MANIPULATED_IMAGE,
    tags::COMPRESSION_RECOGNITION_CODE,
    tags::COMPRESSION_CODE,
    tags::COMPRESSION_ORIGINATOR,
    tags::COMPRESSION_LABEL,
    tags::COMPRESSION_DESCRIPTION,
    tags::COMPRESSION_SEQUENCE,
    tags::COMPRESSION_STEP_POINTERS,
    tags::REPEAT_INTERVAL,
    tags::BITS_GROUPED,
    tags::PERIMETER_TABLE,
    tags::PERIMETER_VALUE,
    tags::PREDICTOR_ROWS,
    tags::PREDICTOR_COLUMNS,
    tags::PREDICTOR_CONSTANTS,
    tags::BLOCKED_PIXELS,
    tags::BLOCK_ROWS,
    tags::BLOCK_COLUMNS,
    tags::ROW_OVERLAP,
    tags::COLUMN_OVERLAP,
    tags::SMALLEST_VALID_PIXEL_VALUE,
    tags::LARGEST_VALID_PIXEL_VALUE,
    tags::SMALLEST_IMAGE_PIXEL_VALUE_IN_PLANE,
    tags::LARGEST_IMAGE_PIXEL_VALUE_IN_PLANE,
    tags::IMAGE_LOCATION,
    tags::TRANSFORM_LABEL,
    tags::TRANSFORM_VERSION_NUMBER,
    tags::NUMBER_OF_TRANSFORM_STEPS,
    tags::SEQUENCE_OF_COMPRESSED_DATA,
    tags::DETAILS_OF_COEFFICIENTS,
    tags::DCT_LABEL,
    tags::DATA_BLOCK_DESCRIPTION,
    tags::DATA_BLOCK,
    tags::NORMALIZATION_FACTOR_FORMAT,
    tags::ZONAL_MAP_NUMBER_FORMAT,
    tags::ZONAL_MAP_LOCATION,
    tags::ZONAL_MAP_FORMAT,
    tags::ADAPTIVE_MAP_FORMAT,
    tags::CODE_NUMBER_FORMAT,
    tags::GRAY_SCALE,
    tags::GRAY_LOOKUP_TABLE_DESCRIPTOR,
    tags::LARGE_RED_PALETTE_COLOR_LOOKUP_TABLE_DESCRIPTOR,
    tags::LARGE_GREEN_PALETTE_COLOR_LOOKUP_TABLE_DESCRIPTOR,
    tags::LARGE_BLUE_PALETTE_COLOR_LOOKUP_TABLE_DESCRIPTOR,
    tags::GRAY_LOOKUP_TABLE_DATA,
    tags::LARGE_RED_PALETTE_COLOR_LOOKUP_TABLE_DATA,
    tags::LARGE_GREEN_PALETTE_COLOR_LOOKUP_TABLE_DATA,
    tags::LARGE_BLUE_PALETTE_COLOR_LOOKUP_TABLE_DATA,
    tags::LARGE_PALETTE_COLOR_LOOKUP_TABLE_UID,
    tags::IMAGE_PRESENTATION_COMMENTS,
    tags::BI_PLANE_ACQUISITION_SEQUENCE,
    tags::MASK_POINTERS,
    tags::LARGEST_MONOCHROME_PIXEL_VALUE,
    tags::STUDY_STATUS_ID,
    tags::STUDY_PRIORITY_ID,
    tags::STUDY_ID_ISSUER,
    tags::STUDY_VERIFIED_DATE,
    tags::STUDY_VERIFIED_TIME,
    tags::STUDY_READ_DATE,
    tags::STUDY_READ_TIME,
    tags::SCHEDULED_STUDY_START_DATE,
    tags::SCHEDULED_STUDY_START_TIME,
    tags::SCHEDULED_STUDY_STOP_DATE,
    tags::SCHEDULED_STUDY_STOP_TIME,
    tags::SCHEDULED_STUDY_LOCATION,
    tags::SCHEDULED_STUDY_LOCATION_AE_TITLE,
    tags::REASON_FOR_STUDY,
    tags::STUDY_ARRIVAL_DATE,
    tags::STUDY_ARRIVAL_TIME,
    tags::STUDY_COMPLETION_DATE,
    tags::STUDY_COMPLETION_TIME,
    tags::STUDY_COMPONENT_STATUS_ID,
    tags::STUDY_COMMENTS,
    tags::REFERENCED_PATIENT_ALIAS_SEQUENCE,
    tags::ISSUER_OF_ADMISSION_ID,
    tags::SCHEDULED_ADMISSION_DATE,
    tags::SCHEDULED_ADMISSION_TIME,
    tags::SCHEDULED_DISCHARGE_DATE,
    tags::SCHEDULED_DISCHARGE_TIME,
    tags::SCHEDULED_PATIENT_INSTITUTION_RESIDENCE,
    tags::DISCHARGE_DATE,
    tags::DISCHARGE_TIME,
    tags::DISCHARGE_DIAGNOSIS_DESCRIPTION,
    tags::DISCHARGE_DIAGNOSIS_CODE_SEQUENCE,
    tags::ISSUER_OF_SERVICE_EPISODE_ID,
    tags::TOTAL_TIME_OF_FLUOROSCOPY,
    tags::TOTAL_NUMBER_OF_EXPOSURES,
    tags::DISTANCE_SOURCE_TO_SUPPORT,
    tags::EXPOSURE_DOSE_SEQUENCE,
    tags::REFERENCED_PROCEDURE_STEP_SEQUENCE,
    tags::SPECIMEN_ACCESSION_NUMBER,
    tags::SPECIMEN_SEQUENCE,
    tags::SPECIMEN_DESCRIPTION_SEQUENCE_TRIAL,
    tags::SPECIMEN_DESCRIPTION_TRIAL,
    tags::SLIDE_IDENTIFIER,
    tags::PIXEL_SPACING_SEQUENCE,
    tags::COORDINATE_SYSTEM_AXIS_CODE_SEQUENCE,
    tags::VITAL_STAIN_CODE_SEQUENCE_TRIAL,
    tags::PLACER_ORDER_NUMBER_PROCEDURE,
    tags::FILLER_ORDER_NUMBER_PROCEDURE,
    tags::REQUESTED_PROCEDURE_DESCRIPTION_TRIAL,
    tags::REASON_FOR_THE_IMAGING_SERVICE_REQUEST,
    tags::PLACER_ORDER_NUMBER_IMAGING_SERVICE_REQUEST_RETIRED,
    tags::FILLER_ORDER_NUMBER_IMAGING_SERVICE_REQUEST_RETIRED,
    tags::GENERAL_PURPOSE_SCHEDULED_PROCEDURE_STEP_STATUS,
    tags::GENERAL_PURPOSE_PERFORMED_PROCEDURE_STEP_STATUS,
    tags::GENERAL_PURPOSE_SCHEDULED_PROCEDURE_STEP_PRIORITY,
    tags::SCHEDULED_PROCESSING_APPLICATIONS_CODE_SEQUENCE,
    tags::MULTIPLE_COPIES_FLAG,
    tags::PERFORMED_PROCESSING_APPLICATIONS_CODE_SEQUENCE,
    tags::RESULTING_GENERAL_PURPOSE_PERFORMED_PROCEDURE_STEPS_SEQUENCE,
    tags::REFERENCED_GENERAL_PURPOSE_SCHEDULED_PROCEDURE_STEP_SEQUENCE,
    tags::INPUT_AVAILABILITY_FLAG,
    tags::RELEVANT_INFORMATION_SEQUENCE,
    tags::REFERENCED_GENERAL_PURPOSE_SCHEDULED_PROCEDURE_STEP_TRANSACTION_UID,
    tags::REQUESTED_SUBSEQUENT_WORKITEM_CODE_SEQUENCE,
    tags::NON_DICOM_OUTPUT_CODE_SEQUENCE,
    tags::FINDINGS_FLAG_TRIAL,
    tags::FINDINGS_SEQUENCE_TRIAL,
    tags::FINDINGS_GROUP_UID_TRIAL,
    tags::REFERENCED_FINDINGS_GROUP_UID_TRIAL,
    tags::FINDINGS_GROUP_RECORDING_DATE_TRIAL,
    tags::FINDINGS_GROUP_RECORDING_TIME_TRIAL,
    tags::FINDINGS_SOURCE_CATEGORY_CODE_SEQUENCE_TRIAL,
    tags::DOCUMENTING_ORGANIZATION_IDENTIFIER_CODE_SEQUENCE_TRIAL,
    tags::MEASUREMENT_PRECISION_DESCRIPTION_TRIAL,
    tags::URGENCY_OR_PRIORITY_ALERTS_TRIAL,
    tags::SEQUENCING_INDICATOR_TRIAL,
    tags::DOCUMENT_IDENTIFIER_CODE_SEQUENCE_TRIAL,
    tags::DOCUMENT_AUTHOR_TRIAL,
    tags::DOCUMENT_AUTHOR_IDENTIFIER_CODE_SEQUENCE_TRIAL,
    tags::IDENTIFIER_CODE_SEQUENCE_TRIAL,
    tags::OBJECT_BINARY_IDENTIFIER_TRIAL,
    tags::DOCUMENTING_OBSERVER_IDENTIFIER_CODE_SEQUENCE_TRIAL,
    tags::PROCEDURE_IDENTIFIER_CODE_SEQUENCE_TRIAL,
    tags::OBJECT_DIRECTORY_BINARY_IDENTIFIER_TRIAL,
    tags::EQUIVALENT_CDA_DOCUMENT_SEQUENCE,
    tags::DATE_OF_DOCUMENT_OR_VERBAL_TRANSACTION_TRIAL,
    tags::TIME_OF_DOCUMENT_CREATION_OR_VERBAL_TRANSACTION_TRIAL,
    tags::REPORT_STATUS_ID_TRIAL,
    tags::REFERENCED_FRAME_NUMBERS,
    tags::OBSERVATION_CATEGORY_CODE_SEQUENCE_TRIAL,
    tags::BIBLIOGRAPHIC_CITATION_TRIAL,
    tags::REFERENCED_OBSERVATION_UID_TRIAL,
    tags::REFERENCED_OBSERVATION_CLASS_TRIAL,
    tags::REFERENCED_OBJECT_OBSERVATION_CLASS_TRIAL,
    tags::OBSERVATION_DATE_TRIAL,
    tags::OBSERVATION_TIME_TRIAL,
    tags::MEASUREMENT_AUTOMATION_TRIAL,
    tags::IDENTIFICATION_DESCRIPTION_TRIAL,
    tags::COORDINATES_SET_GEOMETRIC_TYPE_TRIAL,
    tags::ALGORITHM_CODE_SEQUENCE_TRIAL,
    tags::ALGORITHM_DESCRIPTION_TRIAL,
    tags::PIXEL_COORDINATES_SET_TRIAL,
    tags::CURRENT_OBSERVER_TRIAL,
    tags::REFERENCED_ACCESSION_SEQUENCE_TRIAL,
    tags::REPORT_STATUS_COMMENT_TRIAL,
    tags::PROCEDURE_CONTEXT_SEQUENCE_TRIAL,
    tags::VERBAL_SOURCE_TRIAL,
    tags::ADDRESS_TRIAL,
    tags::TELEPHONE_NUMBER_TRIAL,
    tags::VERBAL_SOURCE_IDENTIFIER_CODE_SEQUENCE_TRIAL,
    tags::REPORT_DETAIL_SEQUENCE_TRIAL,
    tags::OBSERVATION_SUBJECT_UID_TRIAL,
    tags::OBSERVATION_SUBJECT_CLASS_TRIAL,
    tags::OBSERVATION_SUBJECT_TYPE_CODE_SEQUENCE_TRIAL,
    tags::OBSERVATION_SUBJECT_CONTEXT_FLAG_TRIAL,
    tags::OBSERVER_CONTEXT_FLAG_TRIAL,
    tags::PROCEDURE_CONTEXT_FLAG_TRIAL,
    tags::RELATIONSHIP_SEQUENCE_TRIAL,
    tags::RELATIONSHIP_TYPE_CODE_SEQUENCE_TRIAL,
    tags::LANGUAGE_CODE_SEQUENCE_TRIAL,
    tags::UNIFORM_RESOURCE_LOCATOR_TRIAL,
    tags::TEMPLATE_VERSION,
    tags::TEMPLATE_LOCAL_VERSION,
    tags::TEMPLATE_EXTENSION_FLAG,
    tags::TEMPLATE_EXTENSION_ORGANIZATION_UID,
    tags::TEMPLATE_EXTENSION_CREATOR_UID,
    tags::REFERENCED_IMAGE_NAVIGATION_SEQUENCE,
    tags::TOP_LEFT_HAND_CORNER_OF_LOCALIZER_AREA,
    tags::BOTTOM_RIGHT_HAND_CORNER_OF_LOCALIZER_AREA,
    tags::COUNTS_INCLUDED,
    tags::DEAD_TIME_CORRECTION_FLAG,
    tags::TRIANGLE_POINT_INDEX_LIST,
    tags::EDGE_POINT_INDEX_LIST,
    tags::VERTEX_POINT_INDEX_LIST,
    tags::PRIMITIVE_POINT_INDEX_LIST,
    tags::IMAGE_ROTATION_RETIRED,
    tags::DISPLAYED_AREA_TOP_LEFT_HAND_CORNER_TRIAL,
    tags::DISPLAYED_AREA_BOTTOM_RIGHT_HAND_CORNER_TRIAL,
    tags::GRAPHIC_LAYER_RECOMMENDED_DISPLAY_RGB_VALUE,
    tags::COMPOSITING_METHOD,
    tags::WEIGHTING_LOOKUP_TABLE_DESCRIPTOR,
    tags::WEIGHTING_LOOKUP_TABLE_DATA,
    tags::BEAM_ORDER_INDEX_TRIAL,
    tags::DOUBLE_EXPOSURE_METERSET_TRIAL,
    tags::DOUBLE_EXPOSURE_FIELD_DELTA_TRIAL,
    tags::RELATED_PROCEDURE_STEP_SEQUENCE,
    tags::PROCEDURE_STEP_RELATIONSHIP_TYPE,
    tags::TOPIC_TITLE,
    tags::TOPIC_SUBJECT,
    tags::TOPIC_AUTHOR,
    tags::TOPIC_KEYWORDS,
    tags::COLOR_IMAGE_PRINTING_FLAG,
    tags::COLLATION_FLAG,
    tags::ANNOTATION_FLAG,
    tags::IMAGE_OVERLAY_FLAG,
    tags::PRESENTATION_LUT_FLAG,
    tags::IMAGE_BOX_PRESENTATION_LUT_FLAG,
    tags::REFERENCED_STORED_PRINT_SEQUENCE,
    tags::REFERENCED_IMAGE_OVERLAY_BOX_SEQUENCE,
    tags::REFERENCED_VOILUT_BOX_SEQUENCE,
    tags::REFERENCED_OVERLAY_PLANE_SEQUENCE,
    tags::REFERENCED_OVERLAY_PLANE_GROUPS,
    tags::OVERLAY_PIXEL_DATA_SEQUENCE,
    tags::OVERLAY_MAGNIFICATION_TYPE,
    tags::OVERLAY_SMOOTHING_TYPE,
    tags::OVERLAY_OR_IMAGE_MAGNIFICATION,
    tags::MAGNIFY_TO_NUMBER_OF_COLUMNS,
    tags::OVERLAY_FOREGROUND_DENSITY,
    tags::OVERLAY_BACKGROUND_DENSITY,
    tags::OVERLAY_MODE,
    tags::THRESHOLD_DENSITY,
    tags::REFERENCED_IMAGE_BOX_SEQUENCE_RETIRED,
    tags::PRINT_JOB_ID,
    tags::REFERENCED_PRINT_JOB_SEQUENCE_PULL_STORED_PRINT,
    tags::PRINT_QUEUE_ID,
    tags::QUEUE_STATUS,
    tags::PRINT_JOB_DESCRIPTION_SEQUENCE,
    tags::REFERENCED_PRINT_JOB_SEQUENCE,
    tags::PRINT_MANAGEMENT_CAPABILITIES_SEQUENCE,
    tags::PRINTER_CHARACTERISTICS_SEQUENCE,
    tags::FILM_BOX_CONTENT_SEQUENCE,
    tags::IMAGE_BOX_CONTENT_SEQUENCE,
    tags::ANNOTATION_CONTENT_SEQUENCE,
    tags::IMAGE_OVERLAY_BOX_CONTENT_SEQUENCE,
    tags::PRESENTATION_LUT_CONTENT_SEQUENCE,
    tags::RT_DOSE_ROI_SEQUENCE,
    tags::CONTOUR_SLAB_THICKNESS,
    tags::CONTOUR_OFFSET_VECTOR,
    tags::ATTACHED_CONTOURS,
    tags::ROI_OBSERVATION_LABEL,
    tags::ROI_OBSERVATION_DESCRIPTION,
    tags::ADDITIONAL_RTROI_IDENTIFICATION_CODE_SEQUENCE,
    tags::FRAME_OF_REFERENCE_RELATIONSHIP_SEQUENCE,
    tags::RELATED_FRAME_OF_REFERENCE_UID,
    tags::FRAME_OF_REFERENCE_TRANSFORMATION_TYPE,
    tags::TREATMENT_TERMINATION_CODE,
    tags::TREATMENT_SITES,
    tags::BEAM_DOSE_SPECIFICATION_POINT,
    tags::AVERAGE_BEAM_DOSE_POINT_DEPTH,
    tags::AVERAGE_BEAM_DOSE_POINT_EQUIVALENT_DEPTH,
    tags::AVERAGE_BEAM_DOSE_POINT_SSD,
    tags::REFERENCED_RT_PATIENT_SETUP_SEQUENCE,
    tags::PATIENT_SETUP_UID,
    tags::ROBOTIC_BASE_LOCATION_INDICATOR,
    tags::ARBITRARY,
    tags::TEXT_COMMENTS,
    tags::RESULTS_ID,
    tags::RESULTS_ID_ISSUER,
    tags::REFERENCED_INTERPRETATION_SEQUENCE,
    tags::REPORT_PRODUCTION_STATUS_TRIAL,
    tags::INTERPRETATION_RECORDED_DATE,
    tags::INTERPRETATION_RECORDED_TIME,
    tags::INTERPRETATION_RECORDER,
    tags::REFERENCE_TO_RECORDED_SOUND,
    tags::INTERPRETATION_TRANSCRIPTION_DATE,
    tags::INTERPRETATION_TRANSCRIPTION_TIME,
    tags::INTERPRETATION_TRANSCRIBER,
    tags::INTERPRETATION_TEXT,
    tags::INTERPRETATION_AUTHOR,
    tags::INTERPRETATION_APPROVER_SEQUENCE,
    tags::INTERPRETATION_APPROVAL_DATE,
    tags::INTERPRETATION_APPROVAL_TIME,
    tags::PHYSICIAN_APPROVING_INTERPRETATION,
    tags::INTERPRETATION_DIAGNOSIS_DESCRIPTION,
    tags::INTERPRETATION_DIAGNOSIS_CODE_SEQUENCE,
    tags::RESULTS_DISTRIBUTION_LIST_SEQUENCE,
    tags::DISTRIBUTION_NAME,
    tags::DISTRIBUTION_ADDRESS,
    tags::INTERPRETATION_ID,
    tags::INTERPRETATION_ID_ISSUER,
    tags::INTERPRETATION_TYPE_ID,
    tags::INTERPRETATION_STATUS_ID,
    tags::IMPRESSIONS,
    tags::RESULTS_COMMENTS,
    tags::COEFFICIENTS_SDVN,
    tags::COEFFICIENTS_SDHN,
    tags::COEFFICIENTS_SDDN,
    tags::ROWS_FOR_NTH_ORDER_COEFFICIENTS,
    tags::COLUMNS_FOR_NTH_ORDER_COEFFICIENTS,
    tags::COEFFICIENT_CODING,
    tags::COEFFICIENT_CODING_POINTERS,
    tags::CODE_LABEL,
    tags::NUMBER_OF_TABLES,
    tags::CODE_TABLE_LOCATION,
    tags::BITS_FOR_CODE_WORD,
    tags::IMAGE_DATA_LOCATION,
    tags::ESCAPE_TRIPLET,
    tags::RUN_LENGTH_TRIPLET,
    tags::HUFFMAN_TABLE_SIZE,
    tags::HUFFMAN_TABLE_TRIPLET,
    tags::SHIFT_TABLE_SIZE,
    tags::SHIFT_TABLE_TRIPLET,
    tags::ZONAL_MAP,
];
