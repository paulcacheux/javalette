use codemap::{Span, Spanned};
use common::{Field, Literal};

#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Struct(Struct),
    ExternFunction(ExternFunction),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<(Spanned<String>, Spanned<Type>)>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ExternFunction {
    pub return_ty: Spanned<Type>,
    pub name: String,
    pub parameters: Vec<Spanned<Type>>,
    pub is_vararg: bool,
    pub span: Span,
}

impl ExternFunction {
    pub fn get_type(&self) -> FunctionType {
        let return_ty = self.return_ty.clone();
        let parameters_ty = self.parameters.clone();
        FunctionType {
            return_ty,
            parameters_ty,
            is_vararg: self.is_vararg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub return_ty: Spanned<Type>,
    pub name: String,
    pub parameters: Vec<(String, Spanned<Type>)>,
    pub body: BlockStatement,
    pub span: Span,
}

impl Function {
    pub fn get_type(&self) -> FunctionType {
        let return_ty = self.return_ty.clone();
        let parameters_ty = self.parameters
            .iter()
            .map(|&(_, ref a)| a.clone())
            .collect();
        FunctionType {
            return_ty,
            parameters_ty,
            is_vararg: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Empty,
    Block(BlockStatement),
    Let(LetStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Return(Option<Spanned<Expression>>),
    Expression(Spanned<Expression>),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Spanned<Statement>>,
}

impl BlockStatement {
    pub fn from_vec(statements: Vec<Spanned<Statement>>) -> Self {
        BlockStatement { statements }
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: String,
    pub ty: Option<Spanned<Type>>,
    pub value: Spanned<Expression>,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Spanned<Expression>,
    pub body: Box<Spanned<Statement>>,
    pub else_clause: Option<Box<Spanned<Statement>>>,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Spanned<Expression>,
    pub body: Box<Spanned<Statement>>,
}

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub init: Box<Spanned<Statement>>,
    pub condition: Spanned<Expression>,
    pub step: Option<Spanned<Expression>>,
    pub body: Box<Spanned<Statement>>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Parenthesis(Box<Spanned<Expression>>),
    Assign {
        lhs: Box<Spanned<Expression>>,
        rhs: Box<Spanned<Expression>>,
    },
    BinaryOperator {
        binop: BinaryOperatorKind,
        lhs: Box<Spanned<Expression>>,
        rhs: Box<Spanned<Expression>>,
    },
    LazyOperator {
        lazyop: LazyOperatorKind,
        lhs: Box<Spanned<Expression>>,
        rhs: Box<Spanned<Expression>>,
    },
    UnaryOperator {
        unop: UnaryOperatorKind,
        sub: Box<Spanned<Expression>>,
    },
    LValueUnaryOperator {
        lvalue_unop: LValueUnaryOperatorKind,
        sub: Box<Spanned<Expression>>,
    },
    Cast {
        as_ty: Spanned<Type>,
        sub: Box<Spanned<Expression>>,
    },
    Subscript {
        array: Box<Spanned<Expression>>,
        index: Box<Spanned<Expression>>,
    },
    FunctionCall {
        function: Box<Spanned<Expression>>,
        args: Vec<Spanned<Expression>>,
    },
    TupleLiteral {
        values: Vec<Spanned<Expression>>,
    },
    ArrayLiteral {
        values: Vec<Spanned<Expression>>,
    },
    ArrayFillLiteral {
        value: Box<Spanned<Expression>>,
        size: usize,
    },
    StructLiteral {
        struct_name: String,
        fields: Vec<(Spanned<String>, Spanned<Expression>)>,
    },
    FieldAccess {
        expr: Box<Spanned<Expression>>,
        field: Field,
    },
    Nullptr,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, Copy)]
pub enum LazyOperatorKind {
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperatorKind {
    Minus,
    LogicalNot,
    PtrDeref,
}

#[derive(Debug, Clone, Copy)]
pub enum LValueUnaryOperatorKind {
    Increment,
    Decrement,
    AddressOf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Void,
    Identifier(String),
    Pointer(Box<Spanned<Type>>),
    Array(Box<Spanned<Type>>, usize),
    Function(Box<FunctionType>),
    Tuple(Vec<Spanned<Type>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    pub return_ty: Spanned<Type>,
    pub parameters_ty: Vec<Spanned<Type>>,
    pub is_vararg: bool,
}
