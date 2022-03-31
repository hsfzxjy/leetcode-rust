use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

type ExprRef = usize;
type Lit = i32;
type VarName<'a> = Cow<'a, [u8]>;

#[derive(Debug)]
struct LetExpr<'a> {
    vars: RefCell<HashMap<VarName<'a>, Lit>>,
    assigns: Vec<(VarName<'a>, ExprRef)>,
    last: ExprRef,
}

#[derive(Debug)]
enum ArithType {
    Add,
    Multi,
}

#[derive(Debug)]
struct ArithExpr {
    lhs: ExprRef,
    rhs: ExprRef,
    typ: ArithType,
}

#[derive(Debug)]
enum InnerExpr<'a> {
    Arith(ArithExpr),
    Let(LetExpr<'a>),
    Literal(Lit),
    Name(VarName<'a>),
}
use InnerExpr::*;

#[derive(Debug)]
struct Expr<'a> {
    parent: Option<usize>,
    inner: InnerExpr<'a>,
}

impl<'a> Expr<'a> {
    fn eval<'b: 'a>(&'a self, exprs: &'b [Self]) -> Lit {
        match self.inner {
            Literal(x) => x,
            Name(ref x) => self.lookup(x, exprs),
            Arith(ref x) => {
                let L = exprs[x.lhs].eval(exprs);
                let R = exprs[x.rhs].eval(exprs);
                match x.typ {
                    ArithType::Add => L + R,
                    ArithType::Multi => L * R,
                }
            }
            Let(ref x) => {
                for (name, id) in &x.assigns {
                    let val = exprs[*id].eval(exprs);
                    x.vars.borrow_mut().insert(name.clone(), val);
                }
                exprs[x.last].eval(exprs)
            }
        }
    }
    fn lookup<'b: 'a>(&'a self, name: &VarName<'a>, exprs: &'b [Self]) -> Lit {
        match self.inner {
            Let(ref x) => {
                if let Some(v) = x.vars.borrow().get(name) {
                    return v.clone();
                } else {
                    let parent = &exprs[self.parent.unwrap()];
                    return parent.lookup(name, exprs);
                }
            }
            _ => exprs[self.parent.expect(&format!("{:?}", self.inner))].lookup(name, exprs),
        }
    }
}

struct Program<'a> {
    exprs: Vec<Expr<'a>>,
}

fn parse_int(mut s: &[u8]) -> Lit {
    let mut res = 0;
    let neg = if s[0] == '-' as u8 {
        s = &s[1..];
        true
    } else {
        false
    };
    for c in s {
        res = res * 10 + c.clone() as Lit - '0' as u8 as Lit;
    }
    if neg {
        -res
    } else {
        res
    }
}

impl<'a> Program<'a> {
    fn new() -> Self {
        Self { exprs: vec![] }
    }
    fn cur_id(&self) -> ExprRef {
        self.exprs.len() - 1
    }
    fn parse<'b: 'a>(&mut self, code: &'b [u8]) -> &'b [u8] {
        let start = if code[0] == '(' as u8 { 1 } else { 0 };
        let mut i = start;
        while code[i] != ' ' as u8 && code[i] != ')' as u8 {
            i += 1
        }
        let rest = &code[i..];
        match &code[start..i] {
            b"let" => self.parse_let(&rest[1..]),
            b"add" => self.parse_arith(&rest[1..], ArithType::Add),
            b"mult" => self.parse_arith(&rest[1..], ArithType::Multi),
            x if x[0].is_ascii_digit() || x[0] == '-' as u8 => {
                self.exprs.push(Expr {
                    parent: None,
                    inner: Literal(parse_int(x)),
                });
                rest
            }
            x => {
                self.exprs.push(Expr {
                    parent: None,
                    inner: Name(Cow::Borrowed(x)),
                });
                rest
            }
        }
    }
    fn parse_arith<'b: 'a>(&mut self, code: &'b [u8], typ: ArithType) -> &'b [u8] {
        let code = self.parse(code);
        let lhs_id = self.cur_id();
        let code = self.parse(&code[1..]);
        let rhs_id = self.cur_id();
        self.exprs.push(Expr {
            parent: None,
            inner: Arith(ArithExpr {
                typ,
                lhs: lhs_id,
                rhs: rhs_id,
            }),
        });
        let my_id = self.cur_id();
        self.exprs[lhs_id].parent.replace(my_id);
        self.exprs[rhs_id].parent.replace(my_id);
        &code[1..]
    }
    fn parse_let<'b: 'a>(&mut self, mut code: &'b [u8]) -> &'b [u8] {
        let mut assigns = vec![];
        let mut ids = vec![];
        let last;
        loop {
            code = self.parse(code);
            let i = self.cur_id();
            if code[0] == ')' as u8 {
                last = i;
                ids.push(last);
                break;
            }
            let name = match &self.exprs[i].inner {
                Name(x) => x.clone(),
                _ => panic!(),
            };
            code = self.parse(&code[1..]);
            let j = self.cur_id();
            ids.push(j);
            assigns.push((name, j));
            code = &code[1..];
        }
        self.exprs.push(Expr {
            parent: None,
            inner: Let(LetExpr {
                assigns,
                last,
                vars: Default::default(),
            }),
        });
        let my_id = self.cur_id();
        for id in ids {
            self.exprs[id].parent.replace(my_id);
        }
        &code[1..]
    }
    fn eval(&'a self) -> Lit {
        self.exprs.last().unwrap().eval(&self.exprs)
    }
}

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let mut p = Program::new();
        p.parse(expression.as_bytes());
        p.eval()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! make_test {
        ($name: ident, $expr: expr,  $res: expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::evaluate($expr.into()), $res);
            }
        };
    }

    make_test!(test1, "(let x 2 (mult x (let x 3 y 4 (add x y))))", 14);
    make_test!(test2, "(let x 7 -12)", -12);
}
