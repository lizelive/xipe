use serde_yaml::Value;

type Statement = Box<AnyStatement>;
type Expr = Box<AnyExpr>;


enum BinaryOperator {
    Part,
    
}

type Integer = isize;
type Symbol = String;

enum AnyExpr {
    Value(Value),
    Get(Symbol),
    BinaryOperator(AnyExpr, BinaryOperator, AnyExpr),
    Call(Symbol, Expr)
}


enum AnyStatement {
    Set(Symbol, Expr),
    If(Expr, Statement, Statement),
    CleanUp(Statement, Statement),
}