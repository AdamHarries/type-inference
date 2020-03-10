use super::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Context(pub Vec<CtxMember>);

impl From<Vec<CtxMember>> for Context {
    fn from(v: Vec<CtxMember>) -> Self {
        Self(v)
    }
}

impl Into<Vec<CtxMember>> for Context {
    fn into(self) -> Vec<CtxMember> {
        self.0
    }
}

impl Context {
    /// Add a context member to the right of the context.
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    /// ]);
    /// ctx = ctx.add(ctx_var!("D"));
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn add(mut self, c: CtxMember) -> Self {
        self.0.push(c);
        self
    }
    /// Check to see if a context member is a member of this context
    /// ```
    /// use infer::*;
    /// let ctx: Context = Context(vec![
    ///     ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    /// ]);
    /// assert!(ctx.elem(&ctx_var!("D")));
    /// ```
    pub fn elem(&self, c: &CtxMember) -> bool {
        self.0.contains(c)
    }
    /// Drop a single element from the "front" of the list
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    /// ]);
    /// let ctx2: Context = Context(vec![

    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    /// ]);
    /// ctx = ctx.drop1();
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn drop1(mut self) -> Self {
        self.0.remove(0);
        self
    }

    /// Split a context at a given CtxMember, returning the elements that are non-equal to c in the "prefix", and the remaining elements including c in the "remainder"
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    ///     ctx_var!("E"),
    ///     ctx_var!("F"),
    ///     ctx_var!("G"),
    ///     ctx_var!("H"),
    ///     ctx_var!("I"),
    /// ]);
    /// let prefix: Context = Context(vec![
    ///     ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    /// ]);
    /// let remainder: Context = Context(vec![
    ///     ctx_var!("E"),
    ///     ctx_var!("F"),
    ///     ctx_var!("G"),
    ///     ctx_var!("H"),
    ///     ctx_var!("I"),
    /// ]);
    /// let (p, r) = ctx.split_at(&ctx_var!("E")).unwrap();
    /// assert_eq!(prefix, p);
    /// assert_eq!(remainder, r);
    /// ```
    pub fn split_at(self, c: &CtxMember) -> Option<(Context, Context)> {
        if self.0.contains(c) {
            let prefix: Context = self
                .0
                .iter()
                .take_while(|&e| e != c)
                .cloned()
                .collect::<Vec<CtxMember>>()
                .into();
            let remainder: Context = self
                .0
                .iter()
                .skip_while(|&e| e != c)
                .cloned()
                .collect::<Vec<CtxMember>>()
                .into();
            Some((prefix, remainder))
        } else {
            None
        }
    }
    /// Create a "hole" in a context by removing a given CtxMember and
    /// returning the other two parts of the context
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    ///     ctx_var!("E"),
    ///     ctx_var!("F"),
    ///     ctx_var!("G"),
    ///     ctx_var!("H"),
    ///     ctx_var!("I"),
    /// ]);
    /// let prefix: Context = Context(vec![
    ///      ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    /// ]);
    /// let remainder: Context = Context(vec![
    ///     ctx_var!("F"),
    ///     ctx_var!("G"),
    ///     ctx_var!("H"),
    ///     ctx_var!("I"),
    /// ]);
    /// let (p, r) = ctx.hole(&ctx_var!("E")).unwrap();
    /// assert_eq!(prefix, p);
    /// assert_eq!(remainder, r);
    /// ```
    pub fn hole(self, m: &CtxMember) -> Option<(Context, Context)> {
        match self.split_at(m) {
            Some((p, r)) => Some((p, r.drop1())),
            None => None,
        }
    }

    /// Create two "holes" in a context by removing two given CtxMember and
    /// returning the other three parts of the context
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     ctx_var!("A"),
    ///     ctx_var!("B"),
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    ///     ctx_var!("E"),
    ///     ctx_var!("F"),
    ///     ctx_var!("G"),
    ///     ctx_var!("H"),
    ///     ctx_var!("I"),
    /// ]);
    /// let a: Context = Context(vec![
    ///     ctx_var!("A"),
    /// ]);
    /// let b: Context = Context(vec![
    ///     ctx_var!("C"),
    ///     ctx_var!("D"),
    /// ]);
    /// let c: Context = Context(vec![
    ///     ctx_var!("F"),
    ///     ctx_var!("G"),
    ///     ctx_var!("H"),
    ///     ctx_var!("I"),
    /// ]);
    /// let (ar, br, cr) = ctx.hole2(&ctx_var!("B"), &ctx_var!("E")).unwrap();
    /// assert_eq!(ar, a);
    /// assert_eq!(br, b);
    /// assert_eq!(cr, c);
    /// ```
    pub fn hole2(
        self,
        m1: &CtxMember,
        m2: &CtxMember,
    ) -> Option<(Context, Context, Context)> {
        let (a, ctx_p) = self.hole(m1)?;
        let (b, c) = ctx_p.hole(m2)?;
        Some((a, b, c))
    }
    /// Filter this context for members that match the predicate P
    pub fn filter<P>(&self, p: P) -> Vec<CtxMember>
    where
        P: FnMut(&&CtxMember) -> bool,
    {
        self.0.iter().filter(p).cloned().collect::<Vec<CtxMember>>()
    }
    /// Search this context for an assumption with this variable, and return
    /// the type of the assumption.
    pub fn has_assumption(&self, e: &EVar) -> Option<Type> {
        let assumptions = self.filter(|m| match m {
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
    /// Search this context for a solution with this variable, and return the
    /// type of the solution.
    pub fn has_solution(&self, e: &TEVar) -> Option<Type> {
        let solutions = self.filter(|m| match m {
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
    pub fn is_type_well_formed(&self, a: Type) -> bool {
        match a {
            Type::Unit => true,
            Type::Var(v) => self.elem(&ctx_var!(v.clone())),
            Type::EVar(v) => {
                self.elem(&ctx_evar!(&v)) || self.has_solution(&v).is_some()
            }
            Type::Arr(x, y) => {
                self.is_type_well_formed(*x) && self.is_type_well_formed(*y)
            }
            Type::All(v, t) => {
                let new_ctx = self.clone().add(ctx_var!(v));
                new_ctx.is_type_well_formed(*t)
            }
        }
    }
    /// Figure 8 - "Applying a context, as a substitution, to a type"
    pub fn apply_context(&self, a: Type) -> Type {
        match a {
            Type::Unit => Type::Unit,
            v @ Type::Var(_) => v,
            Type::EVar(ref alpha) => {
                if let Some(tau) = self.has_solution(&alpha) {
                    self.apply_context(tau.clone())
                } else {
                    a
                }
            }
            Type::Arr(a, b) => {
                ty_arr!(self.apply_context(*a), self.apply_context(*b))
            }
            Type::All(v, t) => ty_all!(v, self.apply_context(*t)),
            _ => Type::Unit,
        }
    }
}
