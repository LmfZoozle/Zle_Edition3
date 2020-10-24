use super::lexer::*;
use super::*;

fn prim(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    if let NoOrNum::Yes(nu) = expect_num_next(token) {
        eprintln!("prim　から　num");
        new_node_num(nu)
    } else {
        panic!("なにかおかしい");
    }
}

fn mul(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    let mut result = prim(token);
    loop {
        if expect_ope_next(Operator::Mul, token) {
            eprintln!("mul　から　MUL");
            result = new_node_ope(Token::Ope(Operator::Mul), result, prim(token))
        } else if expect_ope_next(Operator::Div, token) {
            eprintln!("mul　から　DIV");
            result = new_node_ope(Token::Ope(Operator::Div), result, prim(token));
        } else {
            eprintln!("mul　から　break");
            return result;
        }
    }
}

pub fn token_into_tree(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    let mut result = mul(token);
    loop {
        if expect_ope_next(Operator::Add, token) {
            eprintln!("token　から　ADD");
            result = new_node_ope(Token::Ope(Operator::Add), result, mul(token))
        } else if expect_ope_next(Operator::Sub, token) {
            eprintln!("token　から　SUB");
            result = new_node_ope(Token::Ope(Operator::Sub), result, mul(token));
        } else {
            eprintln!("token　から　return");
            return result;
        }
    }
}

fn expect_ope_next(ope: lexer::Operator, itr: &mut std::slice::Iter<Token>) -> bool {
    let mut clo = itr.clone();
    if let Some(bb) = clo.next() {
        if let Token::Ope(a) = *bb {
            if a == ope {
                itr.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }else{
        false
    }
}

enum NoOrNum {
    No,
    Yes(i32),
}

fn expect_num_next(itr: &mut std::slice::Iter<Token>) -> NoOrNum {
    let mut clo = itr.clone();
    if let Token::Num(a) = clo.next().unwrap() {
        itr.next();
        NoOrNum::Yes(*a)
    } else {
        NoOrNum::No
    }
}

/*fn should_expr(master:&mut Box<Node>, itr:&mut std::slice::Iter<Box<Node>>)->Box<Node>{
    new_node_num(4);
}

//fn consume(itr:std::slice::Iter<Box<Node>>);
fn should_number(master:&mut Box<Node>,itr:&mut std::slice::Iter<Box<Node>>)->Box<Node>{
    if let Node::Num(n)=**itr.next().unwrap(){
       let result=new_node_num(n);
       result
    }else{
        panic!("should num");
    }
}*/

#[derive(PartialEq, Clone)]
pub enum Node {
    Num(i32),
    Ope(OpeAndNode),
}

pub fn new_node_num(num: i32) -> Box<Node> {
    Box::new(Node::Num(num))
}

fn new_node_ope(wh: lexer::Token, lf: Box<Node>, rt: Box<Node>) -> Box<Node> {
    Box::new(Node::Ope(_new_op_an_no(wh, lf, rt)))
}

#[derive(PartialEq, Clone)]
pub struct OpeAndNode {
    pub what: lexer::Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

fn _new_op_an_no(wh: lexer::Token, lf: Box<Node>, rt: Box<Node>) -> OpeAndNode {
    OpeAndNode::_new(wh, lf, rt)
}

impl OpeAndNode {
    fn _new(wh: lexer::Token, lf: Box<Node>, rt: Box<Node>) -> Self {
        OpeAndNode {
            what: wh,
            left: lf,
            right: rt,
        }
    }
}

pub mod debug {
    fn declare_debug(name: &str) {
        eprintln!("This is Debug Fn !!");
        eprintln!("From: {}", name);
    }
    use super::super::lexer;
}
