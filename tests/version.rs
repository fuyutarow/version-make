use cli::version::{Semver, Version};

#[test]
fn set_version() {
    let mut version = Version::parse("0.1.23-pre+build").unwrap();
    assert_eq!(
        version.set(Some(1), Some(2), None, Some("".to_string()), None),
        Version::parse("1.2.23+build").unwrap()
    );
}

#[test]
fn update_version() {
    let mut version = Version::parse("0.1.23-pre+build").unwrap();
    assert_eq!(
        version.update(Some(1), Some(2), None),
        Version::parse("1.3.23-pre+build").unwrap()
    );
}
