use melior::{
    dialect::{arith, func},
    ir::{
        attribute::{IntegerAttribute, StringAttribute, TypeAttribute},
        r#type::FunctionType,
        Block, Region, Type,
    },
    Context,
};

use crate::parser::FunctionDefinition;

pub struct Mlir<'c> {
    pub context: &'c Context,
    pub module: melior::ir::Module<'c>,
}

impl<'c> Mlir<'c> {
    pub fn new(context: &'c Context) -> Self {
        let location = melior::ir::Location::unknown(context);
        let module = melior::ir::Module::new(location);
        Self { context, module }
    }

    pub fn add_function(&mut self, func: &FunctionDefinition) {
        let index_type = Type::index(&self.context);
        self.module.body().append_operation(func::func(
            &self.context,
            StringAttribute::new(&self.context, func.identifier.as_str()),
            TypeAttribute::new(FunctionType::new(&self.context, &[], &[index_type]).into()),
            {
                let block = Block::new(&[]);
                let v0 = block.append_operation(arith::constant(
                    &self.context,
                    IntegerAttribute::new(index_type, 21).into(),
                    func.location.mlir_location(&self.context),
                ));
                let v0 = v0.result(0).unwrap();

                block.append_operation(func::r#return(
                    &[v0.into()],
                    func.location.mlir_location(&self.context),
                ));
                let region = Region::new();
                region.append_block(block);
                region
            },
            &[],
            func.location.mlir_location(&self.context),
        ));
    }
}
