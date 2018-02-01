#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Double,
    Boolean,
    String,
    Void,
    LValue(Box<Type>),
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
}

impl Type {
    pub fn has_default_value(&self) -> bool {
        match *self {
            Type::Int | Type::Double | Type::Boolean => true,
            _ => false,
        }
        // maybe add a default value for arrays
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub return_ty: Type,
    pub parameters_ty: Vec<Type>,
    pub is_vararg: bool,
}
