use std::fmt;

#[derive(Clone)]
pub struct Sym(String);

impl fmt::Display for Sym {
    fn fmt(&self, io: &mut fmt::Formatter)->fmt::Result {
        write!(io, "{}", self.0)
    }
}
impl Sym {
    pub fn new(s: &str)->Sym {
        Sym(s.to_string())
    }
}
