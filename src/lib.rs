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
        // <:Unit
        // Checking unit against unit will always return the same state.
        (Type::Unit, Type::Unit) => Some(s),
        // <:Var
        // Checking a plain var against another (with the same names) will
        // always return the same state, and be successful.
        (Type::Var(a), Type::Var(b)) if a == b => Some(s),
        // <:Exvar
        // Checking an existential variable against another (with the same
        // name) will always return the same state, and be successful.
        (Type::EVar(a), Type::EVar(b)) if a == b => Some(s),
        // <:-->
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
        // <:ForallL
        // (Type::All(v, T), t) => {
        //     // let r1 = s.fresh_evar();
        //     // let gamma = s.ctx
        //     None
        // }

        // <:ForallR
        (t, Type::All(v, T)) => {
            // Create a fresh variable for alpha in the context.
            let r1 = ctx_evar!(s.fresh_evar());
            // extend s.ctx with that new variable
            s.ctx = s.ctx.add(r1);
            // Check that t is a subtype of T with the new context.
            s = is_subtype(s, t, *T)?;
            // Split the new context at alpha, and return the first part _only_
            s.ctx = match s.ctx.split_at(&r1) {
                Some((prefix, _)) => prefix, 
                None => panic!("Variable no longer in context!");
            };

            Some(s)
        }
        _ => None,
    }
}
