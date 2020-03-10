/// The type of normal type variables
pub type TVar = String;
/// The type of existential type variables
pub type TEVar = String;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    /// Unit type
    Unit,
    /// Type variable
    Var(TVar),
    /// Existential type variable
    EVar(TEVar),
    /// Arrow (function) type
    Arr(Box<Type>, Box<Type>),
    /// for all quantification over types
    All(TVar, Box<Type>),
}

impl Type {
    fn is_mono(&self) -> bool {
        match self {
            Type::Unit => true,
            Type::Var(_) => true,
            Type::EVar(_) => true,
            Type::Arr(a, b) => a.is_mono() && b.is_mono(),
            Type::All(_, _) => false,
        }
    }
}

/// Convenience macro for creating a unit type
/// ```
/// use infer::*;
/// let v = Type::Unit;
/// let u = ty_unit!();
/// assert_eq!(v,u)
/// ```
#[macro_export]
macro_rules! ty_unit {
    () => {
        (Type::Unit)
    };
}

/// Convenience macro for creating a type variable
/// ```
/// use infer::*;
/// let v = Type::Var("a".into());
/// let u = ty_var!("a");
/// assert_eq!(v,u)
/// ```
#[macro_export]
macro_rules! ty_var {
    ($varname:expr) => {
        (Type::Var($varname.into()))
    };
}

/// Convenience macro for creating a extistential type variable
/// ```
/// use infer::*;
/// let v = Type::EVar("a".into());
/// let u = ty_evar!("a");
/// assert_eq!(v,u)
/// ```
#[macro_export]
macro_rules! ty_evar {
    ($varname:expr) => {
        (Type::EVar($varname.into()))
    };
}

/// Convenience macro for creating a type arrow
/// ```
/// use infer::*;
/// let v = Type::Arr(Box::new(ty_unit!()),Box::new(ty_unit!()));
/// let u = ty_arr!(ty_unit!(), ty_unit!());
/// assert_eq!(v, u);
/// ```
#[macro_export]
macro_rules! ty_arr {
    ($dom_ty:expr, $codom_ty:expr) => {
        (Type::Arr(Box::new($dom_ty), Box::new($codom_ty)))
    };
}

/// Convenience macro for creating a type "forall"
/// ```
/// use infer::*;
/// let v = Type::All("a".into(), Box::new(ty_unit!()));
/// let u = ty_all!("a", ty_unit!());
/// assert_eq!(v,u);
/// ```
#[macro_export]
macro_rules! ty_all {
    ($var:expr, $ty:expr) => {
        (Type::All($var.into(), Box::new($ty)))
    };
}
