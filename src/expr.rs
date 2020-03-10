pub type EVar = String;

#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    Unit,
    Var(EVar),
    Ann(Box<Expr>, String),
    Lam(EVar, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

/// Make a unit expression
/// # Examples:
/// ```
/// use infer::*;
/// let e : Expr = Expr::Unit;
/// let u : Expr = expr_unit!();
/// assert_eq!(e, u);
/// ```
#[macro_export]
macro_rules! expr_unit {
    () => {{
        Expr::Unit
    }};
}
