use regex::RegexSet;

pub struct CheckerOptions {
    pub ignore_patterns: RegexSet,
}

impl Default for CheckerOptions {
    fn default() -> Self {
        let ignore_patterns = RegexSet::new(&[
            r"\.git$",
            r"\.svn$",
            r"\.hg$",
            r"\.idea$",
            r"^node_modules$",
            r"^dist$",
            r"^build$",
            r"^bower_components$",
            // Images
            r"\.png$",
            r"\.gif$",
            r"\.jpg$",
            r"\.jpeg$",
            r"\.svg$",
            // Fonts
            r"\.woff$",
            r"\.woff2$",
            r"\.eot$",
            r"\.ttf$",
            // Archives
            r"\.zip$",
            r"\.gz$",
            // Videos
            r"\.mp4$",
        ])
        .unwrap();
        CheckerOptions { ignore_patterns }
    }
}

mod tests {
    use super::CheckerOptions;

    #[test]
    fn should_ignore_png() {
        let options = CheckerOptions::default();

        let is_match = options.ignore_patterns.is_match("image.png");
        assert!(is_match);

        let is_match = options.ignore_patterns.is_match("image.png.");
        assert!(!is_match);

        let is_match = options.ignore_patterns.is_match("imagepng");
        assert!(!is_match);
    }

    #[test]
    fn should_ignore_node_modules() {
        let options = CheckerOptions::default();

        let is_match = options.ignore_patterns.is_match("node_modules");
        assert!(is_match);

        let is_match = options.ignore_patterns.is_match("node_module");
        assert!(!is_match);

        let is_match = options.ignore_patterns.is_match("ode_modules");
        assert!(!is_match);
    }

    #[test]
    fn should_ignore_dist() {
        let options = CheckerOptions::default();

        let is_match = options.ignore_patterns.is_match("dist");
        assert!(is_match);

        let is_match = options.ignore_patterns.is_match("distt");
        assert!(!is_match);

        let is_match = options.ignore_patterns.is_match("ist");
        assert!(!is_match);
    }

    #[test]
    fn should_ignore_build() {
        let options = CheckerOptions::default();

        let is_match = options.ignore_patterns.is_match("build");
        assert!(is_match);

        let is_match = options.ignore_patterns.is_match("bbuild");
        assert!(!is_match);

        let is_match = options.ignore_patterns.is_match("uild");
        assert!(!is_match);
    }
}
