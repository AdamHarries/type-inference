pub mod context;
pub mod ctxmember;
pub mod expr;
pub mod types;

pub use context::*;
pub use ctxmember::*;
pub use expr::*;
pub use types::*;

/// Search a given context for an assumption with this variable, and return
/// the type of the assumption.
pub fn assump(ctx: &Context, e: &EVar) -> Option<Type> {
    let assumptions = ctx.filter(|m| match m {
        CtxMember::Assump(e2, _) => e2 == e,
        _ => false,
    });
    match assumptions.len() {
        0 => None,
        1 => assumptions[0].clone().get_type(),
        _ => panic!(
            "ctxSolution: internal error - multiple types for variable: {:?}",
            assumptions
        ),
    }
}

/// Search a given context for a solution with this variable, and return the
/// type of the solution.
pub fn solution(ctx: &Context, e: &TEVar) -> Option<Type> {
    let solutions = ctx.filter(|m| match m {
        CtxMember::Solved(e2, _) => e2 == e,
        _ => false,
    });
    match solutions.len() {
        0 => None,
        1 => solutions[0].clone().get_type(),
        _ => panic!(
            "ctxSolution: internal error - multiple types for variable: {:?}",
            solutions
        ),
    }
}

/// Figure 7 - "Well formedness of types and contexwts in the algorithmic system"
/// Part one: "Under context Gamma, type A is well formed"
/// Checks if a type `a` is well formed under context `ctx`
pub fn is_type_well_formed(ctx: &Context, a: Type) -> bool {
    match a {
        Type::Unit => true,
        Type::Var(v) => ctx.elem(&ctx_var!(v.clone())),
        Type::EVar(v) => {
            ctx.elem(&ctx_evar!(&v)) || solution(ctx, &v).is_some()
        }
        Type::Arr(x, y) => {
            is_type_well_formed(ctx, *x) && is_type_well_formed(ctx, *y)
        }
        Type::All(v, t) => {
            let new_ctx = ctx.clone().add(ctx_var!(v));
            is_type_well_formed(&new_ctx, *t)
        }
    }
}

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
        /// Checking unit against unit will always return the same state.
        (Type::Unit, Type::Unit) => Some(s),
        /// Checking a plain var against another (with the same names) will
        /// always return the same state, and be successful.
        (Type::Var(a), Type::Var(b)) if a == b => Some(s),
        /// Checking an existential variable against another (with the same
        /// name) will always return the same state, and be successful.
        (Type::EVar(a), Type::EVar(b)) if a == b => Some(s),
        /// Checking arrow types
        (Type::Arr(a, b), Type::Arr(c, d)) => {
            // Firstly, we need to check that c is a subtype of a, and get a new
            // context. This can fail, so use "?" to potentially return it.
            let s2 = is_subtype(s, *c, *a)?;
            // With that new context, use apply it to types b and d, and check
            // that b is a subtype of d. Return the resulting context.
            let b2 = apply_context(&s2.ctx, *b);
            let d2 = apply_context(&s2.ctx, *d);
            is_subtype(s2, b2, d2)
        }
        (Type::All(v, T), t) => {
            // let r1 = s.fresh_evar();
            // let gamma = s.ctx
            None
        }

        _ => None,
    }
}

/// Figure 8 - "Applying a context, as a substitution, to a type"
pub fn apply_context(c: &Context, a: Type) -> Type {
    match a {
        Type::Unit => Type::Unit,
        v @ Type::Var(_) => v,
        Type::EVar(ref alpha) => {
            if let Some(tau) = solution(c, &alpha) {
                apply_context(c, tau.clone())
            } else {
                a
            }
        }
        Type::Arr(a, b) => ty_arr!(apply_context(c, *a), apply_context(c, *b)),
        Type::All(v, t) => ty_all!(v, apply_context(c, *t)),
        _ => Type::Unit,
    }
}
