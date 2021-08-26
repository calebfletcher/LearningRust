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
        let build_split: Vec<&str> = value.split('+').collect();
        let (version_without_build, build_metadata) = build_split.split_first().unwrap();

        let prerelease_split: Vec<&str> = version_without_build.split('-').collect();
        let (version_without_prerelease, prerelease) = prerelease_split.split_first().unwrap();

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