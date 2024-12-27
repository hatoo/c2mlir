use c2mlir::{
    lexer::Lexer,
    mlir::AddModule,
    parser::{Parse, Parser, TranslationUnit},
};
use clap::Parser as _;
use std::path::PathBuf;

use melior::{
    dialect::DialectRegistry,
    ir::{operation::OperationPrintingFlags, Location, Module},
    pass::{conversion::create_to_llvm, transform::create_inliner, PassManager},
    utility::register_all_dialects,
    Context,
};

#[derive(Debug, clap::Parser)]
struct Opts {
    filepath: PathBuf,
}

fn main() {
    let opts = Opts::parse();

    let source = std::fs::read(&opts.filepath).unwrap();
    let lexer = Lexer::new(opts.filepath.to_string_lossy().into(), source);
    let mut parser = Parser::new(lexer);

    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();

    let mut module = Module::new(Location::unknown(&context));

    let translation_unit = match TranslationUnit::parse(&mut parser) {
        Ok(translation_unit) => translation_unit,
        Err(parse_error) => {
            eprintln!("{}", parse_error);
            std::process::exit(1);
        }
    };
    for external_declaration in translation_unit.0 {
        match external_declaration {
            c2mlir::parser::ExternalDeclaration::FunctionDefinition(function_definition) => {
                function_definition.add_module(&context, &module);
            }
        }
    }

    assert!(module.as_operation().verify());

    // register_all_passes();
    let pass_manager = PassManager::new(&context);
    pass_manager.add_pass(create_inliner());
    // pass_manager.add_pass(create_canonicalizer());
    // pass_manager.add_pass(create_cse());
    pass_manager.add_pass(create_to_llvm());
    pass_manager.run(&mut module).unwrap();

    assert!(module.as_operation().verify());

    println!(
        "{}",
        module
            .as_operation()
            .to_string_with_flags(OperationPrintingFlags::default().enable_debug_info(true, false))
            .unwrap()
    );
}
