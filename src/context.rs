use super::ctxmember::*;

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

    pub fn filter<P>(&self, p: P) -> Vec<CtxMember>
    where
        P: FnMut(&&CtxMember) -> bool,
    {
        self.0.iter().filter(p).cloned().collect::<Vec<CtxMember>>()
    }
}
