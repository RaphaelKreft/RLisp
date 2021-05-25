/*
printer.rs: Holds functions to transform an expression (as RlType (AST)) back to a string.
            This is needed to make the results of the evaluator readable for a User.
*/

// load needed sibling modules, here we just need the AST-Type
use super::types::RlType;

/**
    This is the Interface that is used to convert an expression(as AST-Type) to a String.
    It uses non-public print_str_rec to build a String of an RLType.

    Arguments:  expression - the RLType that should be represented as a String
    Returns:    returns a String that represents the given RLType
*/
pub fn print_str(expression: RlType) -> String {
    let string: String = print_str_rec(expression);
    return string;
}

/**
    This is a helper that works in recursive manner to build up a String from a given String.
    Depending on the type of the given expression, a string is created and returned.

    Arguments:  expression - the expression as RLType, that should be represented as a String
    Returns:    String that represents the given expression.
*/
pub fn print_str_rec(expression: RlType) -> String {
    // check which type the given expression has
    return match expression {
        RlType::Int(value) => format!("{}", value),
        RlType::Symbol(i) => format!("{}", i),
        RlType::String(i) => format!("'{}'", i),
        // if we have list type, make recursive call to stringify elements of the list and surround
        // the elements that are separated by commas with ()-brackets
        RlType::List(vec) => {
            let iter: Vec<String> = vec.into_iter().map(print_str_rec).collect();
            let owned: String = format!("({})", iter.join(" "));
            owned
        }
        RlType::Bool(b) => {
            if b {
                String::from("#t")
            } else {
                String::from("#f")
            }
        }
        // Function definitions are nit printed out, just labeled with #function
        RlType::Func(_) | RlType::SelfDefinedFunc { .. } => String::from("#function"),
        RlType::Nil => String::from("#nil"),
    };
}
