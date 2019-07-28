use crate::tags::{
    AuthorError, DependencyError, LicenseError, MaintainerError, UrlError, VersionError,
};
use err_derive::Error;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error(display = "unknown format: {}", _0)]
    UnknownFormat(String),
    #[error(display = "no value")]
    NoValue,
    #[error(display = "parsing error")]
    ParsingError(#[error(cause)] roxmltree::Error),
    #[error(display = "no name")]
    NoName,
    #[error(display = "invalid name: {}", _0)]
    InvalidName(String),
    #[error(display = "no version")]
    NoVersion,
    #[error(display = "invalid version")]
    InvalidVersion(#[error(cause)] VersionError),
    #[error(display = "no description")]
    NoDescription,
    #[error(display = "no maintainer")]
    NoMaintainer,
    #[error(display = "invalid maintainer")]
    InvalidMaintainer(#[error(cause)] MaintainerError),
    #[error(display = "no license")]
    NoLicense,
    #[error(display = "invalid license")]
    InvalidLicense(#[error(cause)] LicenseError),
    #[error(display = "invalid url")]
    InvalidUrl(#[error(cause)] UrlError),
    #[error(display = "invalid author")]
    InvalidAuthor(#[error(cause)] AuthorError),
    #[error(display = "invalid dependency")]
    InvalidDependency(#[error(cause)] DependencyError),
}
