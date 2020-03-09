use std::collections::LinkedList;

type EVar = String;
type TVar = String;
type TEVar = String;

#[derive(PartialEq, Eq, Debug)]
enum Expr {
    Unit,
    Var(EVar),
    Ann(Box<Expr>, String),
    Lam(EVar, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

#[derive(PartialEq, Eq, Debug)]
enum Type {
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

#[derive(PartialEq, Eq, Debug)]
enum CtxMember {
    Var(TVar),
    Assump(EVar, Type),
    EVar(TEVar),
    Solved(TEVar, Type),
    Marker(TEVar),
}

struct Context(std::collections::LinkedList<CtxMember>);

impl Context {
    fn pipe(mut self, c: CtxMember) -> Context {
        self.0.push_back(c);
        self
    }

    fn elem(&self, c: &CtxMember) -> bool {
        self.0.contains(c)
    }

    fn hole(mut self, c: &CtxMember) -> Option<(Context, Context)> {
        if self.0.contains(c) {
            // search the list for the index.
            let mut ix: usize = 0;
            for m in self.0.iter() {
                if m == c {
                    break;
                } else {
                    ix += 1;
                }
            }

            let back = self.0.split_off(ix);

            None
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut d = LinkedList::new();

    d.push_front(7);
    d.push_front(6);
    d.push_front(5);
    d.push_front(4);
    d.push_front(3);
    d.push_front(2);
    d.push_front(1);

    let mut splitted = d.split_off(2);
    d.pop_back();

    println!("Elements of d: ");
    for a in d.iter() {
        println!("a: {:?}", a);
    }

    println!("Elements of splitted: ");
    for a in splitted.iter() {
        println!("a: {:?}", a);
    }
}
