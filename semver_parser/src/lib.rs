pub struct SemVer<'a> {
    major: u32,
    minor: u32,
    patch: u32,
    prerelease: Vec<&'a str>,
    build_metadata: Vec<&'a str>,
    is_development: bool,
}

impl<'a> SemVer<'a> {
    pub fn new(value: &str) -> Result<SemVer, &str> {
        let sv = SemVer {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: vec![],
            build_metadata: vec![],
            is_development: false
        };

        Ok(sv)
    }
}