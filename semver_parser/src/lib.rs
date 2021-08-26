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

        let version = SemVer::parse_version(version_without_prerelease)?;

        let sv = SemVer {
            major: version[0],
            minor: version[1],
            patch: version[2],
            prerelease: vec![],
            build_metadata: vec![],
            is_development: false
        };

        Ok(sv)
    }

    fn parse_version(version: &str) -> Result<Vec<u32>, &str> {
        let version_split: Vec<u32> = version
            .split('.')
            .filter(|part| part.chars().count() > 0)
            .filter(|part| part.chars().all(char::is_numeric))
            .filter(|part| part.chars().next().unwrap() != '0')
            .map(|part| part.parse::<u32>().unwrap())
            .collect();

        if version_split.len() != 3 {
            return Err("incorrect number of version parts, expected 3");
        }

        Ok(version_split)
    }
}