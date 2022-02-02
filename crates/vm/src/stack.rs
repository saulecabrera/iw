use crate::val::Value ;

pub enum StackValue {
    Value(Value),
    Label,
    Activation
}

pub enum StackValueType {
    Value,
    Label,
    Activation
}

impl StackValue {
    pub fn ty(&self) -> StackValueType {
        type SVT = StackValueType;
        match *self {
            StackValue::Value(_) => SVT::Value,
            StackValue::Label => SVT::Label,
            StackValue::Activation => SVT::Activation
        }
    }
}


pub struct Stack {
}
