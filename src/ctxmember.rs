use crate::expr::*;
use crate::types::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CtxMember {
    /// A simple type variable
    Var(TVar),
    /// An assumption about an existential type variable, with the assumed type
    Assump(EVar, Type),
    /// An unsolvd existential type variable
    EVar(TEVar),
    /// A solved existential type variable
    Solved(TEVar, Type),
    /// A marker in the context.
    Marker(TEVar),
}

/// Convenience macro for creating a context variable
/// ```
/// use infer::*;
/// let v = CtxMember::Var("a".into());
/// let u = ctx_var!("a");
/// assert_eq!(v,u)
/// ```
#[macro_export]
macro_rules! ctx_var {
    ($varname:expr) => {
        (CtxMember::Var($varname.into()))
    };
}

/// Convenience macro for creating a context assumption
/// ```
/// use infer::*;
/// let v = CtxMember::Assump("a".into(), Type::Unit);
/// let u = ctx_assump!("a", Type::Unit);
/// assert_eq!(v,u)
/// ```
#[macro_export]
macro_rules! ctx_assump {
    ($varname:expr, $ty:expr) => {
        (CtxMember::Assump($varname.into(), $ty))
    };
}

/// Convenience macro for creating a context evariable
/// ```
/// use infer::*;
/// let v = CtxMember::EVar("a".into());
/// let u = ctx_evar!("a");
/// assert_eq!(v,u)
/// ```
#[macro_export]
macro_rules! ctx_evar {
    ($varname:expr) => {
        (CtxMember::EVar($varname.into()))
    };
}

/// Convenience macro for creating a solution in a context
/// ```
/// use infer::*;
/// let v = CtxMember::Solved("a".into(), Type::Unit);
/// let u = ctx_solved!("a", Type::Unit);
/// assert_eq!(v,u)
/// ```
#[macro_export]
macro_rules! ctx_solved {
    ($varname:expr, $ty:expr) => {
        (CtxMember::Solved($varname.into(), $ty))
    };
}

/// Convenience macro for creating a context marker
/// ```
/// use infer::*;
/// let v = CtxMember::Marker("a".into());
/// let u = ctx_marker!("a");
/// assert_eq!(v,u)
/// ```
#[macro_export]
macro_rules! ctx_marker {
    ($varname:expr) => {
        (CtxMember::Marker($varname.into()))
    };
}

impl CtxMember {
    pub fn get_type(self) -> Option<Type> {
        match self {
            CtxMember::Assump(_, t) => Some(t),
            CtxMember::Solved(_, t) => Some(t),
            _ => None,
        }
    }
}
