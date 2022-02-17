use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use swc_common::comments::SingleThreadedComments;
use swc_common::errors::{ColorConfig, Handler};
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::Module;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{EsConfig, Parser, StringInput, Syntax, TsConfig};

pub struct Parsers {
    inner: HashMap<&'static str, Syntax>,
}

impl Default for Parsers {
    fn default() -> Self {
        let inner = HashMap::from([
            (
                r"\.tsx?$",
                Syntax::Typescript(TsConfig {
                    tsx: true,
                    dts: true,
                    ..Default::default()
                }),
            ),
            (
                r"\.jsx?$",
                Syntax::Es(EsConfig {
                    jsx: true,
                    ..Default::default()
                }),
            ),
        ]);
        Parsers { inner }
    }
}

impl Parsers {
    pub fn parse_file(&self, file: &Path) -> Option<(Module, Syntax)> {
        let file_name = file.file_name().unwrap().to_string_lossy();

        let syntax = self
            .inner
            .iter()
            .find(|value| {
                let regex = Regex::new(value.0).unwrap();
                regex.is_match(file_name.as_ref())
            })?
            .1;

        let cm: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

        let fm = cm.load_file(file).expect("failed to load");

        let comments = SingleThreadedComments::default();
        let lexer = Lexer::new(
            syntax.to_owned(),
            Default::default(),
            StringInput::from(&*fm),
            Some(&comments),
        );

        let mut parser = Parser::new_from(lexer);

        for error in parser.take_errors() {
            error.into_diagnostic(&handler).emit();
        }

        let module = parser
            .parse_module()
            .map_err(|e| e.into_diagnostic(&handler).emit())
            .expect("failed to parser module");

        Some((module, syntax.to_owned()))
    }
}
