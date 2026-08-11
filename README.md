# fnodcmanon-rs
A deidentification tool for a bulk of DICOM studies.
 

## Usage
```
fnodcmanon --input-dir <directory> [options]
```

**Input/output options:**
* `--input-dir (-i) <PATH>`: Input directory
* `--output-dir (-o) <PATH> (default ./output)`: Output directory
* `--write-tags`: write pre-deidentification PatientID, Pseudoname, old StudyInstanceUID, new StudyInstanceUID to CSV file at `<OUTPUT_PATH>/deidentified.csv`.

**Pseudoname options:**
* `--prefix (-p)`: set prefix to prepend to pseudoname method. Default is empty string "".

    **Pseudoname method:**
    * `--random-string (default)`: Use random 10-character alphanumeric string
    * `--integer-count <START> (default 1)`: Use incrementing counter starting from 1 (default) or `START>`, ex. `--integer-count 5 -> prefix_5, prefix_6, ...`
    * `--from-file <PATH>`: Use custom pseudonames with optional prefix from .txt file. File must have the following format:
    ```
    // example pseudonames.txt
    PatientID,pseudoname.
    01,TS_01
    02,TS_02
    ``` 
    If a study with PatientID isn't found in the file, a random string will be used instead.

**Deidentification profiles:**

A Basic Application Confidentiality Profile (code `DCM_113100`) is always applied. Other profiles are optional. Deidentification process follows the rules in [Table E.1-1. Application Level Confidentiality Profile Attributes](https://dicom.nema.org/medical/dicom/current/output/chtml/part15/chapter_E.html#table_E.1-1). For a list of all deidentification methods defined by PS3.16 2026c see [De-identification Method Table](https://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_7050.html).
* `--deidentify-profile (-d) <PROFILE>`: set profile to apply during deidentification
  * retain-patient-characteristics: Retain Patient Characteristics Options (code `DCM_113108`)
  * retain-device-identity: Retain Device Identity Option (code `DCM_113109`)
  * retain-institution-identity: Retain Institution Identity Option (code `DCM_113112`)
  * retain-uid: Retain UID Option (code `DCM_113110`)
  * retain-retired: Retain retired tags (code `DCM_RETIRED`). This option is not part of DICOM deidentification methods per the De-identification Method Table.

Profile codes are appended to tag `DEIDENTIFICATION_METHOD`.

**UID Root**
* `--uid-root (-u) <ROOT> (default 2.25)`: UID root to use for replacing StudyInstanceUID, SeriesInstanceUID, SOPInstanceUID. Must consist of period separated integer segments without leading zeros.

## Example usage
`fnodcmanon -i path/to/input/directory -o path/to/output/directory -p TST --from-file path/to/pseudonames.txt -d retain-patient-characteristics --deidentify-profile retain-institution-identity --uid-root 1.2.3`

## Additional information
The tool also modifies several items in tag `ContentSequence` to maintain UID reference consistency: `"110180", "113769", "112002"`. There is a possibility of identifying information remaining unmodified in items of sequence data elements, if such information is present. This includes institution personel, physician or operator names, institution name and/or address and other,. This may be amended in the future.
