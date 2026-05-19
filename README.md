# fnodcmanon-rs
A deidentification tool for a bulk of DICOM studies. The tool iterates over a directory with studies, replaces `PatientID` and `PatientName` tag values with `pseudoname`. Applied pseudoname consists of `<prefix>_<pseudoname_method>`. Additionally removes tags for given profile(s).
 

## Usage
```
fnodcmanon --input-dir <directory> [options]
```

**Input/output options:**
* `--input-dir (-i) <PATH>`: Input directory
* `--output-dir (-o) <PATH> (default ./output)`: Output directory
* `--write-tags`: write pre-deidentification PatientID, Pseudoname, StudyInstanceUID to `OUTPUT_PATH/anonymized.csv` file.

**Pseudoname options:**
* `--prefix (-p)`: set prefix to prepend to pseudoname method. 

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

A Basic Application Confidentiality Profile (code `DCM_113100`) is always applied. Other profiles are optional and explained bellow.
* `--profile <PROFILE>`: set profile to apply during deidentification
  * patient: Deidentify patient characteristics tag values (code `DCM_113108`)
  * device: Deidentify device identity tag values (code `DCM_113109`)
  * institution: Deidentify institution identity tag values (code `DCM_113112`)

Profile codes are appended to tag `DEIDENTIFICATION_METHOD`

**UID Root**
* `--uid-root <ROOT> (default 2.25)`: UID root to use for replacing StudyInstanceUID, SeriesInstanceUID, SOPInstanceUID. Must consist of period separated integer segments without leading zeros.

## Example usage
`fnodcmanon -i path/to/input/directory -o path/to/output/directory -p TST --from-file path/to/pseudonames.txt --profile patient --profile institution`