use globset::{self, Glob, GlobSet, GlobSetBuilder};
use regex::RegexSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CheckerOptions {
    pub ignore_bin_package: bool,
    pub ignore_patterns: Vec<String>,
    pub ignore_matches: Vec<String>,
    pub skip_missing: bool,
}

impl Default for CheckerOptions {
    fn default() -> Self {
        let ignore_patterns = [
            r".git",
            r".svn",
            r".hg",
            r".idea",
            r"node_modules",
            r"dist",
            r"build",
            r"bower_components",
            // Images
            r"*.png",
            r"*.gif",
            r"*.jpg",
            r"*.jpeg",
            r"*.svg",
            // Fonts
            r"*.woff",
            r"*.woff2",
            r"*.eot",
            r"*.ttf",
            // Archives
            r"*.zip",
            r"*.gz",
            // Videos
            r"*.mp4",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        CheckerOptions {
            ignore_patterns,
            skip_missing: Default::default(),
            ignore_bin_package: Default::default(),
            ignore_matches: Default::default(),
        }
    }
}

impl CheckerOptions {
    pub fn with_ignore_patterns(mut self, ignore_patterns: Vec<String>) -> Self {
        self.ignore_patterns = ignore_patterns;
        self
    }

    pub fn with_skip_missing(mut self, skip_missing: bool) -> Self {
        self.skip_missing = skip_missing;
        self
    }

    pub fn with_ignore_bin_package(mut self, ignore_bin_package: bool) -> Self {
        self.ignore_bin_package = ignore_bin_package;
        self
    }

    pub fn with_ignore_matches(mut self, ignore_matches: Vec<String>) -> Self {
        self.ignore_matches = ignore_matches;
        self
    }
}

impl CheckerOptions {
    pub fn get_ignore_patterns(&self) -> Result<GlobSet, globset::Error> {
        let mut builder = GlobSetBuilder::new();

        for pattern in &self.ignore_patterns {
            builder.add(Glob::new(pattern.as_str())?);
        }

        builder.build()
    }

    pub fn get_ignore_matches(&self) -> RegexSet {
        RegexSet::new(&self.ignore_matches).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::CheckerOptions;

    #[test]
    fn should_ignore_png() {
        let options = CheckerOptions::default();

        let is_match = options.get_ignore_patterns().unwrap().is_match("image.png");
        assert!(is_match);

        let is_match = options
            .get_ignore_patterns()
            .unwrap()
            .is_match("image.png.");
        assert!(!is_match);

        let is_match = options.get_ignore_patterns().unwrap().is_match("imagepng");
        assert!(!is_match);
    }

    #[test]
    fn should_ignore_node_modules() {
        let options = CheckerOptions::default();

        let is_match = options
            .get_ignore_patterns()
            .unwrap()
            .is_match("node_modules");
        assert!(is_match);

        let is_match = options
            .get_ignore_patterns()
            .unwrap()
            .is_match("node_module");
        assert!(!is_match);

        let is_match = options
            .get_ignore_patterns()
            .unwrap()
            .is_match("ode_modules");
        assert!(!is_match);
    }

    #[test]
    fn should_ignore_dist() {
        let options = CheckerOptions::default();

        let is_match = options.get_ignore_patterns().unwrap().is_match("dist");
        assert!(is_match);

        let is_match = options.get_ignore_patterns().unwrap().is_match("distt");
        assert!(!is_match);

        let is_match = options.get_ignore_patterns().unwrap().is_match("ist");
        assert!(!is_match);
    }

    #[test]
    fn should_ignore_build() {
        let options = CheckerOptions::default();

        let is_match = options.get_ignore_patterns().unwrap().is_match("build");
        assert!(is_match);

        let is_match = options.get_ignore_patterns().unwrap().is_match("bbuild");
        assert!(!is_match);

        let is_match = options.get_ignore_patterns().unwrap().is_match("uild");
        assert!(!is_match);
    }
}
