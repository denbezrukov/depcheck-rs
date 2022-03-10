use std::path::Path;

use swc_common::comments::SingleThreadedComments;
use swc_common::errors::{ColorConfig, Handler};
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::Module;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{EsConfig, StringInput, Syntax, TsConfig};

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse_file(&self, file: &Path) -> Option<(Module, Syntax)> {
        let extension = file.extension()?.to_str()?;

        let syntax = match extension {
            "ts" | "tsx" => Syntax::Typescript(TsConfig {
                tsx: true,
                dts: true,
                ..Default::default()
            }),
            "js" | "jsx" => Syntax::Es(EsConfig {
                jsx: true,
                ..Default::default()
            }),
            _ => return None,
        };

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

        let mut parser = swc_ecma_parser::Parser::new_from(lexer);

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
