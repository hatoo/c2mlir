use melior::{
    dialect::{arith, func},
    ir::{
        attribute::{IntegerAttribute, StringAttribute, TypeAttribute},
        r#type::FunctionType,
        Block, Location, Module, OperationRef, Region, Type,
    },
    Context,
};

use crate::parser::{BlockItem, Expression, FunctionDefinition, JumpStatement, UnlabeledStatement};

pub trait AddModule {
    fn add_module(&self, context: &Context, module: &Module);
}

pub trait AddBlock {
    fn add_block<'c, 'a>(&self, context: &'c Context, block: &'a Block<'c>)
        -> OperationRef<'c, 'a>;
}

impl AddModule for FunctionDefinition {
    fn add_module(&self, context: &Context, module: &Module) {
        let index_type = Type::index(context);
        module.body().append_operation(func::func(
            context,
            StringAttribute::new(context, self.identifier.as_str()),
            TypeAttribute::new(FunctionType::new(context, &[], &[index_type]).into()),
            {
                let block = Block::new(&[]);
                for item in &self.body.block_items {
                    let BlockItem::UnlabeledStatement(UnlabeledStatement::JumpStatement(
                        jump_statement,
                    )) = item;
                    jump_statement.add_block(context, &block);
                }

                let region = Region::new();
                region.append_block(block);
                region
            },
            &[],
            self.location.mlir_location(context),
        ));
    }
}

impl AddBlock for Expression {
    fn add_block<'c, 'a>(
        &self,
        context: &'c Context,
        block: &'a Block<'c>,
    ) -> OperationRef<'c, 'a> {
        match self {
            Expression::Constant(value) => block.append_operation(arith::constant(
                context,
                IntegerAttribute::new(Type::index(context), *value).into(),
                Location::unknown(context),
            )),
        }
    }
}

impl AddBlock for JumpStatement {
    fn add_block<'c, 'a>(
        &self,
        context: &'c Context,
        block: &'a Block<'c>,
    ) -> OperationRef<'c, 'a> {
        match self {
            JumpStatement::Return(expression) => {
                let v0 = expression.add_block(context, block);
                block.append_operation(func::r#return(
                    &[v0.result(0).unwrap().into()],
                    Location::unknown(context),
                ))
            }
        }
    }
}
