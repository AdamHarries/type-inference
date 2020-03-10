use std::vec::*;

pub type EVar = String;
pub type TVar = String;
pub type TEVar = String;

#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    Unit,
    Var(EVar),
    Ann(Box<Expr>, String),
    Lam(EVar, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Unit,
    Var(TVar),
    EVar(TEVar),
    Arr(Box<Type>, Box<Type>),
    All(TVar, Box<Type>),
}

impl Type {
    fn isMono(&self) -> bool {
        match self {
            Type::Unit => true,
            Type::Var(_) => true,
            Type::EVar(_) => true,
            Type::Arr(a, b) => a.isMono() && b.isMono(),
            Type::All(_, _) => false,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CtxMember {
    Var(TVar),
    Assump(EVar, Type),
    EVar(TEVar),
    Solved(TEVar, Type),
    Marker(TEVar),
}

impl CtxMember {
    pub fn mkvar<T: Into<TVar>>(s: T) -> CtxMember {
        CtxMember::Var(s.into())
    }
}

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
    /// Add a context member to the "left", i.e. "front" of the context
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    ///     CtxMember::mkvar("C"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    ///     CtxMember::mkvar("C"),
    ///     CtxMember::mkvar("D"),
    /// ]);
    /// ctx.tr_l(CtxMember::mkvar("D"));
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn tr_l(&mut self, c: CtxMember) -> () {
        self.0.push(c);
    }

    /// Add a context member to the "right", i.e. the "back" of the context
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    ///     CtxMember::mkvar("C"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     CtxMember::mkvar("D"),
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    ///     CtxMember::mkvar("C"),
    /// ]);
    /// ctx.tr_r(CtxMember::mkvar("D"));
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn tr_r(&mut self, c: CtxMember) -> () {
        self.0.insert(0, c);
    }
    /// Check to see if a context member is a member of this context
    /// ```
    /// use infer::*;
    /// let ctx: Context = Context(vec![
    ///     CtxMember::mkvar("D"),
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    ///     CtxMember::mkvar("C"),
    /// ]);
    /// assert!(ctx.elem(&CtxMember::mkvar("D")));
    /// ```
    pub fn elem(&self, c: &CtxMember) -> bool {
        self.0.contains(c)
    }
    /// Drop a single element from the "front" of the list
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    ///     CtxMember::mkvar("C"),
    ///     CtxMember::mkvar("D"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    ///     CtxMember::mkvar("C"),
    /// ]);
    /// ctx.drop1();
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn drop1(&mut self) -> () {
        self.0.pop();
    }
    /// Drop N elements from the "front" of the list
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    ///     CtxMember::mkvar("C"),
    ///     CtxMember::mkvar("D"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     CtxMember::mkvar("A"),
    ///     CtxMember::mkvar("B"),
    /// ]);
    /// ctx.dropN(2);
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn dropN(&mut self, e: usize) -> () {
        if e >= self.0.len() {
            self.0.clear();
        } else {
            let remaining = self.0.len() - e;
            self.0.truncate(remaining);
        }
    }

    pub fn test() -> () {
        // use infer::*;
        let mut ctx: Context = Context(vec![
            CtxMember::mkvar("A"),
            CtxMember::mkvar("B"),
            CtxMember::mkvar("C"),
            CtxMember::mkvar("D"),
            CtxMember::mkvar("E"),
            CtxMember::mkvar("F"),
            CtxMember::mkvar("G"),
            CtxMember::mkvar("H"),
            CtxMember::mkvar("I"),
        ]);
        let prefix: Context = Context(vec![
            CtxMember::mkvar("F"),
            CtxMember::mkvar("G"),
            CtxMember::mkvar("H"),
            CtxMember::mkvar("I"),
        ]);
        let remainder: Context = Context(vec![
            CtxMember::mkvar("A"),
            CtxMember::mkvar("B"),
            CtxMember::mkvar("C"),
            CtxMember::mkvar("D"),
            CtxMember::mkvar("E"),
        ]);
        let (p, r) = ctx.hole(&CtxMember::mkvar("E")).unwrap();
        assert_eq!(prefix, p);
        assert_eq!(remainder, r);
    }
    pub fn hole(&self, c: &CtxMember) -> Option<(Context, Context)> {
        if self.0.contains(c) {
            let prefix: Context = self
                .0
                .iter()
                .rev() // reverse the iterator, so we view it from the "left"
                .take_while(|&e| e != c) // Take the elements that _don't_ match
                .cloned() // Make them concrete values
                .collect::<Vec<CtxMember>>() // Collect them into a vector
                .iter() // Iterate over _that_ vector
                .rev() // Reverse it back to the original direction
                .cloned() // Again, make them concrete
                .collect::<Vec<CtxMember>>() // And again, collect them into a vector
                .into();
            let remainder: Context = self
                .0
                .iter()
                .rev()
                .skip_while(|&e| e != c)
                .cloned()
                .collect::<Vec<CtxMember>>()
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<CtxMember>>()
                .into();
            Some((prefix, remainder))
        } else {
            None
        }
    }
}
