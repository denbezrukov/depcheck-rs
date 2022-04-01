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

        log::debug!("parse file {:#?}", file);

        let syntax = match extension {
            "ts" | "tsx" => Syntax::Typescript(TsConfig {
                dts: file.ends_with(".d.ts"),
                tsx: extension == "tsx",
                decorators: true,
                no_early_errors: true,
            }),
            "js" | "jsx" => Syntax::Es(EsConfig {
                jsx: true,
                fn_bind: true,
                decorators: true,
                decorators_before_export: true,
                export_default_from: true,
                import_assertions: true,
                static_blocks: true,
                private_in_object: true,
                allow_super_outside_method: true,
            }),
            _ => return None,
        };

        let cm: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

        let fm = cm
            .load_file(file)
            .map_err(|error| {
                log::error!("failed to load {:#?}", error);
            })
            .ok()?;

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
            .map_err(|error| {
                log::error!("failed to parser module {:#?}", error);
            })
            .ok()?;

        Some((module, syntax.to_owned()))
    }
}
