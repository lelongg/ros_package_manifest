use std::str::FromStr;

mod common;
mod error;
mod package1;
mod package2;

pub use common::{PackageCommon, PackageCommonMethods};
pub use error::PackageError;
pub use package1::Package1;
pub use package2::Package2;

#[derive(Debug, Clone, PartialEq)]
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
    use crate::{
        package::{common::PackageCommon, FromStr, Package, Package1},
        tags::{
            Author, BuildType, Dependency, Export, Exported, License, Maintainer, Url,
            UrlType::Website, Version,
        },
    };

    #[test]
    fn package_from_str_test() {
        let expected = Package::Package1(Package1 {
            common: PackageCommon {
                name: "rosmaster".to_string(),
                version: Version {
                    version: "1.14.3".to_string(),
                    compatibility: "1.14.3".to_string(),
                },
                description: "ROS Master implementation.".to_string(),
                maintainer: vec![Maintainer {
                    name: "Dirk Thomas".to_string(),
                    email: "dthomas@osrfoundation.org".to_string(),
                }],
                license: vec![License {
                    license: "BSD".to_string(),
                    file: None,
                }],
                url: vec![Url {
                    url: "http://ros.org/wiki/rosmaster".to_string(),
                    url_type: Website,
                }],
                author: vec![Author {
                    name: "Ken Conley".to_string(),
                    email: None,
                }],
                build_depend: vec![],
                buildtool_depend: vec![Dependency {
                    name: "catkin".to_string(),
                    version_lt: None,
                    version_lte: None,
                    version_eq: None,
                    version_gte: Some("0.5.68".to_string()),
                    version_gt: None,
                    condition: None,
                }],
                test_depend: vec![],
                conflict: vec![],
                replace: vec![],
                export: Some(Export {
                    architecture_independent: true,
                    build_type: BuildType {
                        name: "catkin".to_string(),
                        condition: None,
                    },
                    deprecated: None,
                    message_generator: None,
                    metapackage: false,
                    exported: vec![Exported {
                        name: "rosdoc".to_string(),
                    }],
                }),
            },
            run_depend: vec![
                Dependency {
                    name: "rosgraph".to_string(),
                    version_lt: None,
                    version_lte: None,
                    version_eq: None,
                    version_gte: None,
                    version_gt: None,
                    condition: None,
                },
                Dependency {
                    name: "python-defusedxml".to_string(),
                    version_lt: None,
                    version_lte: None,
                    version_eq: None,
                    version_gte: None,
                    version_gt: None,
                    condition: None,
                },
            ],
        });

        let package_xml =
            std::fs::read_to_string("data/package.xml").expect("cannot read \"data/package.xml\"");
        let package = Package::from_str(&package_xml).unwrap();
        assert_eq!(expected, package);
    }
}
