use crate::tags::{
    AuthorError, DependencyError, LicenseError, MaintainerError, UrlError, VersionError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("unknown format: {}", _0)]
    UnknownFormat(String),
    #[error("no value")]
    NoValue,
    #[error("parsing error")]
    ParsingError(#[from] roxmltree::Error),
    #[error("no name")]
    NoName,
    #[error("invalid name: {}", _0)]
    InvalidName(String),
    #[error("no version")]
    NoVersion,
    #[error("invalid version")]
    InvalidVersion(#[from] VersionError),
    #[error("no description")]
    NoDescription,
    #[error("no maintainer")]
    NoMaintainer,
    #[error("invalid maintainer")]
    InvalidMaintainer(#[from] MaintainerError),
    #[error("no license")]
    NoLicense,
    #[error("invalid license")]
    InvalidLicense(#[from] LicenseError),
    #[error("invalid url")]
    InvalidUrl(#[from] UrlError),
    #[error("invalid author")]
    InvalidAuthor(#[from] AuthorError),
    #[error("invalid dependency")]
    InvalidDependency(#[from] DependencyError),
}
