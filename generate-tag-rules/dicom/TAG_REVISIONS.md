### Revision 2025-07-16
#### Changed tag values
* Causes "Illegal hexadecimal character" error during tag rules generation:
  * OverlayData: (60xx,3000) -> (6000,3000) per dicom-rs/dictionary-std/tags.rs
  * OverlayComments: (60xx,4000) -> (6000,4000) per dicom-rs/dictionary-std/tags.rs
  * CurveData: (50xx,xxxx) -> (5000,3000) per dicom-rs/dictionary-std/tags.rs

#### Removed from code generation
* Tags below are manually modified with custom dummy values based on PseudonameMethod for PN VRs
  * PatientName (0010,0010) - modified with custom pseudoname dummy value
  * PatientID (0010,0020) - modified with custom pseudoname dummy value
