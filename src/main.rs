use attribute::IntegerAttribute;
use melior::{
    dialect::{arith, func, DialectRegistry},
    ir::{
        attribute::{StringAttribute, TypeAttribute},
        r#type::FunctionType,
        *,
    },
    pass::{conversion::create_to_llvm, transform::create_inliner, PassManager},
    utility::register_all_dialects,
    Context,
};

fn main() {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();

    let location = Location::unknown(&context);
    let mut module = Module::new(location);

    let index_type = Type::index(&context);

    module.body().append_operation(func::func(
        &context,
        StringAttribute::new(&context, "main"),
        TypeAttribute::new(FunctionType::new(&context, &[], &[index_type]).into()),
        {
            let block = Block::new(&[]);
            let v0 = block.append_operation(arith::constant(
                &context,
                IntegerAttribute::new(index_type, 21).into(),
                location,
            ));
            let v0 = v0.result(0).unwrap();

            block.append_operation(func::r#return(&[v0.into()], location));
            let region = Region::new();
            region.append_block(block);
            region
        },
        &[],
        location,
    ));

    assert!(module.as_operation().verify());

    // register_all_passes();
    let pass_manager = PassManager::new(&context);
    pass_manager.add_pass(create_inliner());
    // pass_manager.add_pass(create_canonicalizer());
    // pass_manager.add_pass(create_cse());
    pass_manager.add_pass(create_to_llvm());
    pass_manager.run(&mut module).unwrap();

    assert!(module.as_operation().verify());

    println!("{}", module.as_operation());
}
