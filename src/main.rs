use c2mlir::{lexer::Lexer, mlir::Mlir, parser::Parser};
use clap::Parser as _;
use std::path::PathBuf;

use melior::{
    dialect::DialectRegistry,
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

    let func = parser.parse();

    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();

    let mut mlir = Mlir::new(&context);
    mlir.add_function(&func);

    let module = &mut mlir.module;

    assert!(module.as_operation().verify());

    // register_all_passes();
    let pass_manager = PassManager::new(&context);
    pass_manager.add_pass(create_inliner());
    // pass_manager.add_pass(create_canonicalizer());
    // pass_manager.add_pass(create_cse());
    pass_manager.add_pass(create_to_llvm());
    pass_manager.run(module).unwrap();

    assert!(module.as_operation().verify());

    println!("{}", module.as_operation());
}
