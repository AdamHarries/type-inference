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
    // constructors
    pub fn mk_var<V: Into<TVar>>(v: V) -> CtxMember {
        CtxMember::Var(v.into())
    }
    
    pub fn mk_assump<V: Into<EVar>>(v: V, t: Type) -> CtxMember { 
        CtxMember::Assump(v.into(), t)
    }
    
    pub fn mk_evar<V: Into<TEVar>>(v: V) -> CtxMember { 
        CtxMember::EVar(v.into())
    }
    
    pub fn mk_solved<V: Into<TEVar>>(v: V, t: Type) -> CtxMember { 
        CtxMember::Solved(v.into(), t)
    }
    
    
    // getters
    pub fn get_type(self) -> Option<Type> { 
        match self { 
            CtxMember::Assump(_, t) => Some(t), 
            CtxMember::Solved(_, t) => Some(t), 
            _ => None
        }
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
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    /// ]);
    /// ctx = ctx.tr_l(CtxMember::mk_var("D"));
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn tr_l(mut self, c: CtxMember) -> Self {
        self.0.push(c);
        self
    }

    /// Add a context member to the "right", i.e. the "back" of the context
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     CtxMember::mk_var("D"),
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    /// ]);
    /// ctx = ctx.tr_r(CtxMember::mk_var("D"));
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn tr_r(mut self, c: CtxMember) -> Self {
        self.0.insert(0, c);
        self
    }
    /// Check to see if a context member is a member of this context
    /// ```
    /// use infer::*;
    /// let ctx: Context = Context(vec![
    ///     CtxMember::mk_var("D"),
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    /// ]);
    /// assert!(ctx.elem(&CtxMember::mk_var("D")));
    /// ```
    pub fn elem(&self, c: &CtxMember) -> bool {
        self.0.contains(c)
    }
    /// Drop a single element from the "front" of the list
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    /// ]);
    /// ctx = ctx.drop1();
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn drop1(mut self) -> Self {
        self.0.pop();
        self
    }
    /// Drop N elements from the "front" of the list
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    /// ]);
    /// let ctx2: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    /// ]);
    /// ctx = ctx.dropN(2);
    /// assert_eq!(ctx, ctx2);
    /// ```
    pub fn dropN(mut self, e: usize) -> Self {
        if e >= self.0.len() {
            self.0.clear();
        } else {
            let remaining = self.0.len() - e;
            self.0.truncate(remaining);
        }
        self
    }

    /// Split a context at a given CtxMember, returning the elements that are non-equal to c in the "prefix", and the remaining elements including c in the "remainder"
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    ///     CtxMember::mk_var("E"),
    ///     CtxMember::mk_var("F"),
    ///     CtxMember::mk_var("G"),
    ///     CtxMember::mk_var("H"),
    ///     CtxMember::mk_var("I"),
    /// ]);
    /// let prefix: Context = Context(vec![
    ///     CtxMember::mk_var("F"),
    ///     CtxMember::mk_var("G"),
    ///     CtxMember::mk_var("H"),
    ///     CtxMember::mk_var("I"),
    /// ]);
    /// let remainder: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    ///     CtxMember::mk_var("E"),
    /// ]);
    /// let (p, r) = ctx.split_at(&CtxMember::mk_var("E")).unwrap();
    /// assert_eq!(prefix, p);
    /// assert_eq!(remainder, r);
    /// ```
    pub fn split_at(self, c: &CtxMember) -> Option<(Context, Context)> {
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
    /// Create a "hole" in a context by removing a given CtxMember and
    /// returning the other two parts of the context
    /// ```
    /// use infer::*;
    /// let mut ctx: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    ///     CtxMember::mk_var("E"),
    ///     CtxMember::mk_var("F"),
    ///     CtxMember::mk_var("G"),
    ///     CtxMember::mk_var("H"),
    ///     CtxMember::mk_var("I"),
    /// ]);
    /// let prefix: Context = Context(vec![
    ///     CtxMember::mk_var("F"),
    ///     CtxMember::mk_var("G"),
    ///     CtxMember::mk_var("H"),
    ///     CtxMember::mk_var("I"),
    /// ]);
    /// let remainder: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    /// ]);
    /// let (p, r) = ctx.hole(&CtxMember::mk_var("E")).unwrap();
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
    ///     CtxMember::mk_var("A"),
    ///     CtxMember::mk_var("B"),
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    ///     CtxMember::mk_var("E"),
    ///     CtxMember::mk_var("F"),
    ///     CtxMember::mk_var("G"),
    ///     CtxMember::mk_var("H"),
    ///     CtxMember::mk_var("I"),
    /// ]);
    /// let a: Context = Context(vec![
    ///     CtxMember::mk_var("F"),
    ///     CtxMember::mk_var("G"),
    ///     CtxMember::mk_var("H"),
    ///     CtxMember::mk_var("I"),
    /// ]);
    /// let b: Context = Context(vec![
    ///     CtxMember::mk_var("C"),
    ///     CtxMember::mk_var("D"),
    /// ]);
    /// let c: Context = Context(vec![
    ///     CtxMember::mk_var("A"),
    /// ]);
    /// let (ar, br, cr) = ctx.hole2(&CtxMember::mk_var("E"), &CtxMember::mk_var("B")).unwrap();
    /// assert_eq!(ar, a);
    /// assert_eq!(br, b);
    /// assert_eq!(cr, c);
    /// ```
    pub fn hole2(
        self,
        m1: &CtxMember,
        m2: &CtxMember,
    ) -> Option<(Context, Context, Context)> {
        let (a, ctxP) = self.hole(m1)?;
        let (b, c) = ctxP.hole(m2)?;
        Some((a, b, c))
    }

    pub fn filter<P>(&self, p: P) -> Vec<CtxMember>
    where
        P: FnMut(&&CtxMember) -> bool,
    {
        self.0.iter().filter(p).cloned().collect::<Vec<CtxMember>>()
    }

    /// Search a given context for an assumption with this variable, and return 
    /// the type of the assumption. 
    pub fn assump(&self, e: &EVar) -> Option<Type> {
        let assumptions = self.filter(|m| match m {
            CtxMember::Assump(e2, _) => e2 == e,
            _ => false,
        });
        match assumptions.len() { 
            0 => None, 
            1 => assumptions[0].clone().get_type(),
            _ => panic!("ctxSolution: internal error - multiple types for variable: {:?}", assumptions)
        }
    }
    
    /// Search a given context for a solution with this variable, and return the 
    /// type of the solution. 
    pub fn solution(&self, e: &TEVar) -> Option<Type> { 
        let solutions = self.filter(|m| match m { 
            CtxMember::Solved(e2, _) => e2 == e, 
            _ => false,
        });
        match solutions.len() { 
            0 => None, 
            1 => solutions[0].clone().get_type(), 
            _ => panic!("ctxSolution: internal error - multiple types for variable: {:?}", solutions)
        }        
    }
    
    // pub fn type_wf(&self, e: Type) -> Result<(), String> { 
    //     match e { 
    //         Type::Unit => Ok(()), 
    //         Type::Var(v) => { 
    //             if self.elem(&CtxMember::Var(v)) { 
                    
    //             }
                
          
    //         }
    //     };
    //     Ok(())
    // }
}
