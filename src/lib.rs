//! This crate parses ROS package manifest, also known as `package.xml`, as defined in
//! [REP 127](http://www.ros.org/reps/rep-0127.html), [REP 140](http://www.ros.org/reps/rep-0140.html), [REP 149](http://www.ros.org/reps/rep-0149.html).
//!
//! # Examples
//! ```
//! # fn main() -> Result<(), Box<std::error::Error>> {
//! use ros_package_manifest::{Package, PackageCommonMethods};
//! use std::str::FromStr;
//! let package_xml = std::fs::read_to_string("data/package.xml")?;
//! let package = Package::from_str(&package_xml)?;
//! assert_eq!(package.name(), "rosmaster");
//! # Ok(())
//! # }
//! ```
//!

mod package;
mod tags;

pub use package::{Package, Package1, Package2, PackageCommon, PackageCommonMethods, PackageError};
