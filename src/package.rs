use std::str::FromStr;

mod common;
mod error;
mod package1;
mod package2;

pub use common::{PackageCommon, PackageCommonMethods};
pub use error::PackageError;
pub use package1::Package1;
pub use package2::Package2;

#[derive(Debug, Clone)]
pub enum Package {
    Package1(Package1),
    Package2(Package2),
    Package3(Package2),
}

impl PackageCommonMethods for Package {
    fn common(&self) -> &PackageCommon {
        match self {
            Package::Package1(package) => &package.common,
            Package::Package2(package) => &package.common,
            Package::Package3(package) => &package.common,
        }
    }
}

impl FromStr for Package {
    type Err = PackageError;
    fn from_str(package_xml: &str) -> Result<Self, Self::Err> {
        let format = roxmltree::Document::parse(&package_xml)
            .map(|xml_tree| {
                xml_tree
                    .root()
                    .attribute("format")
                    .unwrap_or("1")
                    .to_string()
            })
            .map_err(PackageError::ParsingError)?;

        Ok(match format.as_ref() {
            "1" => Package::Package1(Package1::from_str(package_xml)?),
            "2" => Package::Package2(Package2::from_str(package_xml)?),
            "3" => Package::Package3(Package2::from_str(package_xml)?),
            format => Err(PackageError::UnknownFormat(format.to_string()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_from_str_test() {
        let package_xml =
            std::fs::read_to_string("data/package.xml").expect("cannot read \"data/package.xml\"");
        let package = Package::from_str(&package_xml).unwrap();
        println!("{:#?}", package);
    }
}
