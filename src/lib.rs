pub mod context;
pub mod ctxmember;
pub mod expr;
pub mod types;

pub use context::*;
pub use ctxmember::*;
pub use expr::*;
pub use types::*;

#[derive(Clone)]
pub struct CheckState {
    pub ctx: Context,
    pub nextEVar: usize,
}

impl CheckState {
    pub fn fresh_evar(&mut self) -> EVar {
        let var = format!("e{}", self.nextEVar);
        self.nextEVar = self.nextEVar + 1;
        var
    }
}

/// Checks if a type `a` is a subtype of type `b` under context `ctx`
pub fn is_subtype(mut s: CheckState, a: Type, b: Type) -> Option<CheckState> {
    match (a, b) {
        // Checking unit against unit will always return the same state.
        (Type::Unit, Type::Unit) => Some(s),
        // Checking a plain var against another (with the same names) will
        // always return the same state, and be successful.
        (Type::Var(a), Type::Var(b)) if a == b => Some(s),
        // Checking an existential variable against another (with the same
        // name) will always return the same state, and be successful.
        (Type::EVar(a), Type::EVar(b)) if a == b => Some(s),
        // Checking arrow types
        (Type::Arr(a, b), Type::Arr(c, d)) => {
            // Firstly, we need to check that c is a subtype of a, and get a new
            // context. This can fail, so use "?" to potentially return it.
            let s2 = is_subtype(s, *c, *a)?;
            // With that new context, use apply it to types b and d, and check
            // that b is a subtype of d. Return the resulting context.
            let b2 = s2.ctx.apply_context(*b);
            let d2 = s2.ctx.apply_context(*d);
            is_subtype(s2, b2, d2)
        }
        // (Type::All(v, T), t) => {
        //     // let r1 = s.fresh_evar();
        //     // let gamma = s.ctx
        //     None
        // }
        _ => None,
    }
}
