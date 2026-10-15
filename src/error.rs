use snafu::Snafu;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("{message}"))]
    ScanDirectory {
        message: String,
    },
    #[snafu(display("Failed reading deidentification map from file `{}`", file.display()))]
    FromFile {
        source: std::io::Error,
        file: PathBuf,
    },
    Deidentify {
        source: DeidentifyError,
    },
    #[snafu(display("Failed writing deidentified values"))]
    WriteCsvDeidentify {
        source: csv::Error,
    },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum DeidentifyError {
    CreateStudyDir {
        source: std::io::Error,
    },
    ReadDicomFile {
        source: dicom_object::ReadError,
    },
    WriteDicomFile {
        #[snafu(source(from(dicom_object::WriteError, Box::new)))]
        source: Box<dicom_object::WriteError>,
    },
    AccessElement {
        source: dicom_object::AccessError,
    },
    ConvertValue {
        source: dicom_core::value::ConvertValueError,
    },
}
