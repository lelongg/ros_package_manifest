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
                    .root_element()
                    .attribute("format")
                    .unwrap_or("1")
                    .to_string()
            })
            .map_err(PackageError::ParsingError)?;

        Ok(match format.as_ref() {
            "1" => Package::Package1(Package1::from_str(package_xml)?),
            "2" => Package::Package2(Package2::from_str(package_xml)?),
            "3" => Package::Package3(Package2::from_str(package_xml)?),
            format => return Err(PackageError::UnknownFormat(format.to_string())),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        package::{common::PackageCommon, FromStr, Package, Package1, Package2},
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

    #[test]
    fn package2_from_str_test() {
        let package_xml =r#"<?xml version="1.0"?>
            <package format="2">
                <name>rosmaster</name>
                <version>1.14.3</version>
                <description>
                    ROS <a href="http://ros.org/wiki/Master">Master</a> implementation.
                </description>
                <maintainer email="dthomas@osrfoundation.org">Dirk Thomas</maintainer>
                <license>BSD</license>

                <url>http://ros.org/wiki/rosmaster</url>
                <author>Ken Conley</author>

                <buildtool_depend version_gte="0.5.68">catkin</buildtool_depend>

                <exec_depend>rosgraph</exec_depend>
                <exec_depend>python-defusedxml</exec_depend>

                <doc_depend>doxygen</doc_depend>

                <export>
                    <rosdoc config="rosdoc.yaml"/>
                    <architecture_independent/>
                    <deprecated></deprecated>
                </export>
            </package>
        "#;

        let expected = Package::Package2(Package2 {
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
                export: Some(Export{
                    architecture_independent: true,
                    build_type: BuildType { name: "catkin".to_string(), condition: None },
                    deprecated: None, // BUG: should be Some(String::new()),
                    message_generator: None,
                    metapackage: false,
                    exported: vec![
                        Exported {
                            name: "rosdoc".to_string(),
                        }
                    ]
                }),
            },
            build_export_depend: vec![],
            buildtool_export_depend: vec![],
            doc_depend: vec![
                Dependency {
                    name: "doxygen".to_string(),
                    version_lt: None,
                    version_lte: None,
                    version_eq: None,
                    version_gte: None,
                    version_gt: None,
                    condition: None,
                },
            ],
            exec_depend: vec![
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
            depend: vec![],
        });

        let package = Package::from_str(&package_xml).unwrap();

        assert_eq!(expected, package);
    }

}
