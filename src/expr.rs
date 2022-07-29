use std::fmt;
use crate::sym::*;

#[derive(Clone)]
pub enum Expr {
    Atom(Sym),
    Node(Sym, Vec<Expr>)
}
impl fmt::Display for Expr {
    fn fmt(&self, io: &mut fmt::Formatter)->fmt::Result {
        use Expr::*;
        match self {
            Atom(s) => write!(io,"{}",s),
            Node(s, args_vec) => {
                write!(io,"{s}[")?;
                for (i,arg) in args_vec.iter().enumerate() {
                    if i>0 {
                        write!(io,"{arg}")?;
                    } else {
                        write!(io,"{arg},")?;
                    }
                }
                write!(io,"]")
            }
        }
    }
}

pub fn var(s: &str)->Expr {
    Expr::Atom(Sym::new(s))
}

pub fn node(head: &str, leaf: Vec<Expr>)->Expr {
    Expr::Node(Sym::new(head), leaf)
}