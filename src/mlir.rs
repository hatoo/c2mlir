use melior::{
    dialect::{
        arith, func,
        llvm::{self, AllocaOptions},
    },
    ir::{
        attribute::{IntegerAttribute, StringAttribute, TypeAttribute},
        r#type::FunctionType,
        Block, Location, Module, OperationRef, Region, Type,
    },
    Context,
};

use crate::parser::{
    declaration::Declaration,
    expression::{AdditiveExpression, Expression, MultiplicativeExpression, PrimaryExpression},
    statement::{BlockItem, JumpStatement, UnlabeledStatement},
    Constant, FunctionDefinition,
};

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
                    item.add_block(context, &block);
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
            Expression::AdditiveExpression(additive_expression) => {
                additive_expression.add_block(context, block)
            }
        }
    }
}

impl AddBlock for PrimaryExpression {
    fn add_block<'c, 'a>(
        &self,
        context: &'c Context,
        block: &'a Block<'c>,
    ) -> OperationRef<'c, 'a> {
        match self {
            PrimaryExpression::Constant {
                value: Constant::Integer(value),
                location,
            } => block.append_operation(arith::constant(
                context,
                IntegerAttribute::new(Type::index(context), *value).into(),
                location.mlir_location(context),
            )),
        }
    }
}

impl AddBlock for MultiplicativeExpression {
    fn add_block<'c, 'a>(
        &self,
        context: &'c Context,
        block: &'a Block<'c>,
    ) -> OperationRef<'c, 'a> {
        match self {
            MultiplicativeExpression::PrimaryExpression(primary_expression) => {
                primary_expression.add_block(context, block)
            }
            MultiplicativeExpression::Mul { lhs, rhs, location } => {
                let v0 = lhs.add_block(context, block);
                let v1 = rhs.add_block(context, block);
                block.append_operation(arith::muli(
                    v0.result(0).unwrap().into(),
                    v1.result(0).unwrap().into(),
                    location.mlir_location(context),
                ))
            }
            MultiplicativeExpression::Div { lhs, rhs, location } => {
                let v0 = lhs.add_block(context, block);
                let v1 = rhs.add_block(context, block);
                block.append_operation(arith::divsi(
                    v0.result(0).unwrap().into(),
                    v1.result(0).unwrap().into(),
                    location.mlir_location(context),
                ))
            }
            MultiplicativeExpression::Rem { lhs, rhs, location } => {
                let v0 = lhs.add_block(context, block);
                let v1 = rhs.add_block(context, block);
                block.append_operation(arith::remsi(
                    v0.result(0).unwrap().into(),
                    v1.result(0).unwrap().into(),
                    location.mlir_location(context),
                ))
            }
        }
    }
}

impl AddBlock for AdditiveExpression {
    fn add_block<'c, 'a>(
        &self,
        context: &'c Context,
        block: &'a Block<'c>,
    ) -> OperationRef<'c, 'a> {
        match self {
            AdditiveExpression::PrimaryExpression(primary_expression) => {
                primary_expression.add_block(context, block)
            }
            AdditiveExpression::Add { lhs, rhs, location } => {
                let v0 = lhs.add_block(context, block);
                let v1 = rhs.add_block(context, block);
                block.append_operation(arith::addi(
                    v0.result(0).unwrap().into(),
                    v1.result(0).unwrap().into(),
                    location.mlir_location(context),
                ))
            }
            AdditiveExpression::Minus { lhs, rhs, location } => {
                let v0 = lhs.add_block(context, block);
                let v1 = rhs.add_block(context, block);
                block.append_operation(arith::subi(
                    v0.result(0).unwrap().into(),
                    v1.result(0).unwrap().into(),
                    location.mlir_location(context),
                ))
            }
        }
    }
}

impl AddBlock for Declaration {
    fn add_block<'c, 'a>(
        &self,
        context: &'c Context,
        block: &'a Block<'c>,
    ) -> OperationRef<'c, 'a> {
        match self {
            Declaration::NoAttr { .. } => {
                let isize_type = Type::parse(context, "i64").unwrap();
                let index_type = Type::index(context);
                let one = block.append_operation(arith::constant(
                    context,
                    IntegerAttribute::new(isize_type, 1).into(),
                    Location::unknown(context),
                ));
                block.append_operation(llvm::alloca(
                    context,
                    one.result(0).unwrap().into(),
                    llvm::r#type::pointer(context, 0),
                    Location::unknown(context),
                    AllocaOptions::default().elem_type(Some(TypeAttribute::new(index_type).into())),
                ))
            }
        }
    }
}

impl AddBlock for UnlabeledStatement {
    fn add_block<'c, 'a>(
        &self,
        context: &'c Context,
        block: &'a Block<'c>,
    ) -> OperationRef<'c, 'a> {
        match self {
            UnlabeledStatement::JumpStatement(jump_statement) => {
                jump_statement.add_block(context, block)
            }
        }
    }
}

impl AddBlock for BlockItem {
    fn add_block<'c, 'a>(
        &self,
        context: &'c Context,
        block: &'a Block<'c>,
    ) -> OperationRef<'c, 'a> {
        match self {
            BlockItem::Declaration(declaration) => declaration.add_block(context, block),
            BlockItem::UnlabeledStatement(unlabeled_statement) => {
                unlabeled_statement.add_block(context, block)
            }
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
            JumpStatement::Return {
                expression,
                location,
            } => {
                let v0 = expression.add_block(context, block);
                block.append_operation(func::r#return(
                    &[v0.result(0).unwrap().into()],
                    location.mlir_location(context),
                ))
            }
        }
    }
}
