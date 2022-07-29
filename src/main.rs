use MyRustSym::sym::*;
use MyRustSym::expr::*;
use MyRustSym::parse::*;


fn main() {
    let s_test = "Sin[Plus[1,Multiply[2,x],3]]";
    dbg!("{:?}", lexer(s_test));
}
