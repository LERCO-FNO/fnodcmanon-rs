// @generated from dicom/deidentify_rules.csv by `cargo run -p generate-tag-rules`
// DO NOT EDIT BY HAND - edit deidentify_rules.csv table and regenerate

use crate::profiles::{DeidentifyAction, DeidentifyProfile, TagRule};
use dicom_core::{Tag, VR};

pub static TAG_RULES: &[(Tag, TagRule)] = &[
    (
        Tag(0x0008, 0x0050),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0018, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x0556),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0555),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0022),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x002A),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1400),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x11BB),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x9424),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0032),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0017),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x4035),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x21B0),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA353),
        TagRule {
            vr: VR::ST,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0010),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0038, 0x0020),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1084),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1080),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0038, 0x0021),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0000, 0x1000),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x2110),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xB034),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x006A, 0x0006),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x006A, 0x0005),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x006A, 0x0003),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x0044, 0x0004),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x4000, 0x0010),
        TagRule {
            vr: VR::LT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0044, 0x0104),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0044, 0x0105),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0562),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA078),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x2200, 0x0005),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x00C3),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300C, 0x0127),
        TagRule {
            vr: VR::DT,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x00DD),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1081),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0014, 0x407E),
        TagRule {
            vr: VR::DA,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1203),
        TagRule {
            vr: VR::DT,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::ZeroLength,
                ),
            ],
        },
    ),
    (
        Tag(0x0014, 0x407C),
        TagRule {
            vr: VR::TM,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0016, 0x004D),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1007),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0400, 0x0115),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0310),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x003A, 0x020C),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x003A, 0x0203),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0060),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::ZeroLength,
                ),
            ],
        },
    ),
    (
        Tag(0x0012, 0x0082),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0081),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x0012, 0x0020),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0021),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0072),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0071),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0030),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::ZeroLength,
                ),
            ],
        },
    ),
    (
        Tag(0x0012, 0x0031),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::ZeroLength,
                ),
            ],
        },
    ),
    (
        Tag(0x0012, 0x0010),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0040),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0042),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0051),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0050),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0310),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0280),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x02EB),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0020, 0x9161),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x000F),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0017),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0006),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x3001),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0013),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x009C),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0008, 0x009D),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0050, 0x001B),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x051A),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0512),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0070, 0x0086),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0070, 0x0084),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0023),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA730),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0033),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0107),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0106),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x0010),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1042),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1043),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0xA002),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0xA003),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x2150),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x2100, 0x0040),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x2100, 0x0050),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA307),
        TagRule {
            vr: VR::PN,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0300),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x5000, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5002, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5004, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5006, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5008, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x500A, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x500C, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x500E, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5010, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5012, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5014, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5016, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x5018, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x501A, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x501C, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x501E, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0025),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0035),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA07C),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0xFFFC, 0xFFFC),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA121),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA110),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1205),
        TagRule {
            vr: VR::DA,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1200),
        TagRule {
            vr: VR::DA,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x700C),
        TagRule {
            vr: VR::DA,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1204),
        TagRule {
            vr: VR::DA,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1012),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA120),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1202),
        TagRule {
            vr: VR::DT,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x9701),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x937F),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x2111),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x2100, 0x0140),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x700A),
        TagRule {
            vr: VR::SH,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x001B),
        TagRule {
            vr: VR::UC,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0050, 0x0020),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x002D),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1000),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0016, 0x004B),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1002),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0400, 0x0105),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0xFFFA, 0xFFFA),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0100),
        TagRule {
            vr: VR::UI,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::UIDReplace,
            )],
        },
    ),
    (
        Tag(0x0020, 0x9164),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0030),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0040),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0032),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x079A),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x4008, 0x011A),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0119),
        TagRule {
            vr: VR::PN,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0016),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0013),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x006E),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0068, 0x6226),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA034),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA035),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0042, 0x0011),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x9517),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0037),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0035),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0038),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0036),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0676),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0087),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0086),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x2160),
        TagRule {
            vr: VR::SH,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x2161),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x2162),
        TagRule {
            vr: VR::UC,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x9804),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0x4011),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0058),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0070, 0x031A),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x2017),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x003A, 0x032B),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA023),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA024),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3008, 0x0054),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0196),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0034, 0x0002),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0034, 0x0001),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3010, 0x007F),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0072),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x9074),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0020, 0x9158),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0020, 0x0052),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0034, 0x0007),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x9151),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x9623),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1008),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x0044),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0045),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0041),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1005),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0016, 0x0076),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0075),
        TagRule {
            vr: VR::US,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x008C),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x008D),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0088),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0087),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x008A),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0089),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0084),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0083),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0086),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0085),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x008E),
        TagRule {
            vr: VR::IS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x007B),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0081),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0080),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0072),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0071),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0074),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0073),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0082),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x007A),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x008B),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0078),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x007D),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x007C),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0079),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0077),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x007F),
        TagRule {
            vr: VR::DS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x007E),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x0070),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0070, 0x0001),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x000A),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1011),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1304),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xE004),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x4037),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x4036),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0088, 0x0200),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0020, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0028, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x2400),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x003A, 0x0314),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x4008, 0x0300),
        TagRule {
            vr: VR::ST,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0068, 0x6270),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0015),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0012),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0013),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0014),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0400, 0x0600),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0081),
        TagRule {
            vr: VR::ST,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1040),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1041),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0082),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0080),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x9919),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1050),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x0085),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x004D),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x004C),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1011),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0741),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0742),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0783),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x4008, 0x0112),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0113),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0111),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x010C),
        TagRule {
            vr: VR::PN,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0115),
        TagRule {
            vr: VR::LT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0200),
        TagRule {
            vr: VR::SH,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0202),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0100),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0101),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0102),
        TagRule {
            vr: VR::PN,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x010B),
        TagRule {
            vr: VR::ST,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x010A),
        TagRule {
            vr: VR::PN,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0108),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0109),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x0035),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x0027),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x3010),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x2004),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0038, 0x0011),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0014),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0022),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0073),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0032),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0041),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0043),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0012, 0x0055),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0021),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0038, 0x0061),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0064),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0513),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0562),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0040, 0x2005),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x2200, 0x0002),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0028, 0x1214),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x21D0),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x004F),
        TagRule {
            vr: VR::UT,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0016, 0x0050),
        TagRule {
            vr: VR::UT,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0016, 0x0051),
        TagRule {
            vr: VR::UT,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0016, 0x004E),
        TagRule {
            vr: VR::DS,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0050, 0x0021),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0404),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0016, 0x002B),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x100B),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x0043),
        TagRule {
            vr: VR::ST,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::ZeroLength,
                ),
            ],
        },
    ),
    (
        Tag(0x0002, 0x0003),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x2000),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1090),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x1080),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0550),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0020, 0x3403),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0020, 0x3406),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0020, 0x3405),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0020, 0x3401),
        TagRule {
            vr: VR::CS,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0400, 0x0563),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xB03F),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xB03B),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3008, 0x0056),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x937B),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x003A, 0x0020),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x003A, 0x0310),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1060),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1010),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0012),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0013),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1000),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0400, 0x0552),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0551),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA192),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA032),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA033),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA402),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA193),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA171),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x2180),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1072),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1070),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x2010),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x2011),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x2008),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x2009),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0561),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x2100, 0x0070),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0012, 0x0023),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1000),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x1002),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1001),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6000, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x6000, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6002, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6004, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6006, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6008, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x600A, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x600C, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x600E, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6010, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6012, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6014, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6016, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x6018, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x601A, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x601C, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x601E, 0x3000),
        TagRule {
            vr: VR::OW,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0024),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0034),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0760),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0028, 0x1199),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA07A),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA082),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1040),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1010),
        TagRule {
            vr: VR::AS,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x0030),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1005),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0032),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0038, 0x0400),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0050),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1060),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0101),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0102),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x21F0),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0040),
        TagRule {
            vr: VR::CS,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::ZeroLength,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x2203),
        TagRule {
            vr: VR::CS,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x1020),
        TagRule {
            vr: VR::DS,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x2155),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x2154),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x1030),
        TagRule {
            vr: VR::DS,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0794),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0650),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0500),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x1004),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0792),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x078E),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0243),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0254),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0250),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x4051),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0251),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0253),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0244),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x4050),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0245),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0241),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x4030),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x0242),
        TagRule {
            vr: VR::SH,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x4028),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1050),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1052),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1102),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1104),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1103),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1101),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA123),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0011),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1048),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1049),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1062),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x4008, 0x0114),
        TagRule {
            vr: VR::PN,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x2016),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1004),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3002, 0x0123),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3002, 0x0121),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x21C0),
        TagRule {
            vr: VR::US,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x0012),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x000E),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x007B),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0081),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0070, 0x0082),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0070, 0x0083),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0070, 0x1101),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0070, 0x1102),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1302),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1301),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0061),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x4052),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0044, 0x000B),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0015),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0016),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1030),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1088),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0020, 0x0027),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0019),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0619),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0623),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x067D),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x300A, 0x067C),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1078),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1072),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1079),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1073),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300C, 0x0113),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x100A),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0032, 0x1030),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x005C),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0565),
        TagRule {
            vr: VR::CS,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0x2001),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x1002),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0032, 0x1066),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0032, 0x1067),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0074, 0x1234),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x073A),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3010, 0x000B),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA13A),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0400, 0x0402),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0083),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x006F),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x0031),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3006, 0x0024),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x4023),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1140),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA172),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x0004),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x1100),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1120),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1111),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0400, 0x0403),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x1155),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0004, 0x1511),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1110),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0785),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0092),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0090),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0094),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0096),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x2152),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x00C2),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x0275),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0032, 0x1070),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1400),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0032, 0x1060),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1001),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x1005),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x9937),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0000, 0x1001),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0074, 0x1236),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1032),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0032, 0x1033),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x9185),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x2299),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x2297),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x4008, 0x4000),
        TagRule {
            vr: VR::ST,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0118),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0040),
        TagRule {
            vr: VR::SH,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4008, 0x0042),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0054),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300E, 0x0004),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x300E, 0x0008),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300E, 0x0005),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3006, 0x004D),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x002D),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x0028),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x0038),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x00A6),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3006, 0x004E),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x0026),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3006, 0x002E),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x0088),
        TagRule {
            vr: VR::ST,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3006, 0x0085),
        TagRule {
            vr: VR::SH,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0615),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0611),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3010, 0x005A),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0006),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0004),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0002),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0003),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0007),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0054),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x062A),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0056),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3010, 0x003B),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3008, 0x0162),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3008, 0x0164),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3008, 0x0166),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3008, 0x0168),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0038, 0x001A),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x001B),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x001C),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0038, 0x001D),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x4034),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0038, 0x001E),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x0006),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x000B),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0007),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0004),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0005),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x4008),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0009),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0011),
        TagRule {
            vr: VR::SH,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x4010),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0002),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x4005),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0003),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0001),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x4027),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x0010),
        TagRule {
            vr: VR::SH,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x4025),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1020),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1021),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1000),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1001),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1010),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1011),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1010),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1303),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xB036),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0072, 0x005E),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x0072, 0x005F),
        TagRule {
            vr: VR::AS,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x0072, 0x0061),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x0063),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x0066),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x0068),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x0065),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x006A),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x006C),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x006E),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x006B),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x006D),
        TagRule {
            vr: VR::UN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x0071),
        TagRule {
            vr: VR::UR,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0072, 0x0070),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0020, 0x000E),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0031),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0038, 0x0062),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0038, 0x0060),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x01B2),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0046),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x0042),
        TagRule {
            vr: VR::UT,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x0047),
        TagRule {
            vr: VR::UR,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x0043),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x01A6),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x06FA),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0010, 0x21A0),
        TagRule {
            vr: VR::CS,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0100, 0x0420),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0018),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x0015),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x936A),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0064, 0x0003),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0034, 0x0005),
        TagRule {
            vr: VR::OB,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0008, 0x2112),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0216),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0400, 0x0564),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainInstitutionIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::ZeroLength,
                ),
            ],
        },
    ),
    (
        Tag(0x3008, 0x0105),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x9369),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x022C),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x022E),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0038, 0x0050),
        TagRule {
            vr: VR::LO,
            actions: &[
                (
                    DeidentifyProfile::RetainPatientCharacteristics,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x050A),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0x0602),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0551),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0610),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0600),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0x0554),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x9516),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0055),
        TagRule {
            vr: VR::AE,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Clean,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1010),
        TagRule {
            vr: VR::SH,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0088, 0x0140),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3006, 0x0008),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3006, 0x0006),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x0002),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3006, 0x0004),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x3006, 0x0009),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0032, 0x1040),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1041),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1050),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x1051),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x0012),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0020, 0x000D),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x0034),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x0035),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x0030),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0032, 0x0032),
        TagRule {
            vr: VR::DA,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0032, 0x0033),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0044, 0x0010),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0020, 0x0200),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0054),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x2042),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA354),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xDB0D),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xDB0C),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xDB07),
        TagRule {
            vr: VR::DT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xDB06),
        TagRule {
            vr: VR::DT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x4000, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x2030, 0x0020),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0010, 0x0014),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA122),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA112),
        TagRule {
            vr: VR::TM,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1201),
        TagRule {
            vr: VR::TM,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x700E),
        TagRule {
            vr: VR::TM,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x1014),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0008, 0x0201),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0088, 0x0910),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0088, 0x0912),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0088, 0x0906),
        TagRule {
            vr: VR::ST,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0088, 0x0904),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0062, 0x0021),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0008, 0x1195),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x5011),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3008, 0x0024),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3008, 0x0025),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3008, 0x0250),
        TagRule {
            vr: VR::DA,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x00B2),
        TagRule {
            vr: VR::SH,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0608),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0609),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x300A, 0x0700),
        TagRule {
            vr: VR::UI,
            actions: &[
                (DeidentifyProfile::RetainUID, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::UIDReplace,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x0077),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x000B),
        TagRule {
            vr: VR::LO,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x007A),
        TagRule {
            vr: VR::UT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x3008, 0x0251),
        TagRule {
            vr: VR::TM,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0736),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x300A, 0x0734),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x100A),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA124),
        TagRule {
            vr: VR::UI,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::UIDReplace,
            )],
        },
    ),
    (
        Tag(0x0070, 0x0006),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0018, 0x1009),
        TagRule {
            vr: VR::UT,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x3010, 0x0033),
        TagRule {
            vr: VR::SH,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x3010, 0x0034),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA352),
        TagRule {
            vr: VR::PN,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA358),
        TagRule {
            vr: VR::SQ,
            actions: &[
                (DeidentifyProfile::RetainRetired, DeidentifyAction::Keep),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0040, 0xA030),
        TagRule {
            vr: VR::DT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA088),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::ZeroLength,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA075),
        TagRule {
            vr: VR::PN,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA073),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0040, 0xA027),
        TagRule {
            vr: VR::LO,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::DummyValue,
            )],
        },
    ),
    (
        Tag(0x0038, 0x4000),
        TagRule {
            vr: VR::LT,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0040, 0xB020),
        TagRule {
            vr: VR::SQ,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x003A, 0x0329),
        TagRule {
            vr: VR::ST,
            actions: &[(
                DeidentifyProfile::BasicConfidentiality,
                DeidentifyAction::Remove,
            )],
        },
    ),
    (
        Tag(0x0018, 0x9371),
        TagRule {
            vr: VR::UC,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x9373),
        TagRule {
            vr: VR::ST,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::Remove,
                ),
            ],
        },
    ),
    (
        Tag(0x0018, 0x9367),
        TagRule {
            vr: VR::UC,
            actions: &[
                (
                    DeidentifyProfile::RetainDeviceIdentity,
                    DeidentifyAction::Keep,
                ),
                (
                    DeidentifyProfile::BasicConfidentiality,
                    DeidentifyAction::DummyValue,
                ),
            ],
        },
    ),
];
