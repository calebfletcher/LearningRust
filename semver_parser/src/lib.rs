pub struct SemVer<'a> {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Vec<&'a str>,
    pub build_metadata: Vec<&'a str>,
    pub is_development: bool,
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