# ros_package_manifest

This crate parses ROS package manifest, also known as `package.xml`, as defined in
[REP 127](http://www.ros.org/reps/rep-0127.html), [REP 140](http://www.ros.org/reps/rep-0140.html), [REP 149](http://www.ros.org/reps/rep-0149.html).

[![crate.io](https://img.shields.io/crates/v/ros_package_manifest.svg)](https://crates.io/crates/ros_package_manifest)
[![docs.rs](https://docs.rs/ros_package_manifest/badge.svg)](https://docs.rs/ros_package_manifest)

## Examples
```rust
use ros_package_manifest::{Package, PackageCommonMethods};
use std::str::FromStr;
let package_xml = std::fs::read_to_string("data/package.xml")?;
let package = Package::from_str(&package_xml)?;
assert_eq!(package.name(), "rosmaster");
```

