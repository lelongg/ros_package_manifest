use crate::tags::{
    AuthorError, DependencyError, LicenseError, MaintainerError, UrlError, VersionError,
};

#[derive(Debug)]
pub enum PackageError {
    UnknownFormat(String),
    NoValue,
    ParsingError(roxmltree::Error),
    NoName,
    InvalidName(String),
    NoVersion,
    InvalidVersion(VersionError),
    NoDescription,
    NoMaintainer,
    InvalidMaintainer(MaintainerError),
    NoLicense,
    InvalidLicense(LicenseError),
    InvalidUrl(UrlError),
    InvalidAuthor(AuthorError),
    InvalidDependency(DependencyError),
}
