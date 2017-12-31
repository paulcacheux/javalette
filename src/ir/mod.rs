use ty::*;
use string_interner::StringId;
use span::Span;

mod symbol_table;
pub mod translator;

#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub return_ty: Type,
    pub name: String,
    pub parameters: Vec<(Type, String)>,
    pub body: BlockStatement,
    pub span: Span
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(BlockStatement),
    VarDecl {
        ty: Type,
        name: String,
        value: TypedExpression,
    },
    If {
        condition: TypedExpression,
        body: BlockStatement,
        else_clause: BlockStatement,
    },
    While {
        condition: TypedExpression,
        body: BlockStatement,
    },
    Return(TypedExpression),
    Expression(TypedExpression),
    Break,
    Continue,
}

pub type BlockStatement = Vec<Statement>;

#[derive(Debug, Clone)]
pub struct TypedExpression {
    pub ty: Type,
    pub expr: Expression,
}

#[derive(Debug, Clone)]
pub enum Expression {
    DefaultValue,
    LValueToRValue(Box<TypedExpression>),
    Literal(Literal),
    Identifier(String),
    Assign {
        lhs: Box<TypedExpression>,
        rhs: Box<TypedExpression>,
    },
    BinaryOperator {
        binop: BinaryOperatorKind,
        lhs: Box<TypedExpression>,
        rhs: Box<TypedExpression>,
    },
    LazyOperator {
        lazyop: LazyOperatorKind,
        lhs: Box<TypedExpression>,
        rhs: Box<TypedExpression>,
    },
    UnaryOperator {
        unop: UnaryOperatorKind,
        sub: Box<TypedExpression>,
    },
    Increment(Box<TypedExpression>),
    Decrement(Box<TypedExpression>),
    Subscript {
        array: Box<TypedExpression>,
        index: Box<TypedExpression>,
    },
    FunctionCall {
        function: String,
        args: Vec<TypedExpression>,
    },
    NewArray {
        base_ty: Type,
        sizes: Vec<usize>
    },
    ArrayLength(Box<TypedExpression>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    IntLiteral(i64),
    DoubleLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(StringId),
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperatorKind {
    IntPlus,
    DoublePlus,
    IntMinus,
    DoubleMinus,
    IntMultiply,
    DoubleMultiply,
    IntDivide,
    DoubleDivide,
    IntModulo,
    IntEqual,
    DoubleEqual,
    BooleanEqual,
    IntNotEqual,
    DoubleNotEqual,
    BooleanNotEqual,
    IntLess,
    DoubleLess,
    IntLessEqual,
    DoubleLessEqual,
    IntGreater,
    DoubleGreater,
    IntGreaterEqual,
    DoubleGreaterEqual,
}

#[derive(Debug, Clone, Copy)]
pub enum LazyOperatorKind {
    BooleanLogicalAnd,
    BooleanLogicalOr,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperatorKind {
    IntMinus,
    DoubleMinus,
    BooleanNot,
}
