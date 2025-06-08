//! Contains the functionality to calculate the current Cargo version and make it available
//! to the rest of the application through a `semver` `Version`.
use semver::Version;
use std::sync::LazyLock;

/// Internal static reference to the parsed current version.
///
/// *Remark*: I considered making this a `const` instead of `static`, but then we'd have to either
/// somehow do the const parsing ourselves, or hardcode it in a `Version::new` call and risk having it
/// out of date with Cargo's version.
static VERSION: LazyLock<Version, fn() -> Version> =
    LazyLock::<Version>::new(|| match Version::parse(env!("CARGO_PKG_VERSION")) {
        Ok(version) => version,
        Err(_) => panic!("invalid semver version"),
    });

/// Returns the `Version` of the software as defined in the workspace.
pub fn version() -> &'static Version {
    &*VERSION
}
