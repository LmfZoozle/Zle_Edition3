use super::lexer::Token::*;
use super::lexer::*;
use super::*;

fn consume_number(itr: &mut std::slice::Iter<Token>) -> Option<i32> {
    let mut clo = itr.clone();
    if let Some(actual) = clo.next() {
        if let Token::Number(num) = *actual {
            itr.next();
            Some(num)
        } else {
            None
        }
    } else {
        None
    }
}

fn consume_ident(itr: &mut std::slice::Iter<Token>) -> Option<String> {
    let mut clo = itr.clone();
    if let Some(actual) = clo.next() {
        if let Token::Ident(name) = actual {
            itr.next();
            Some(name.clone())
        } else {
            None
        }
    } else {
        None
    }
}

fn consume(itr: &mut std::slice::Iter<Token>, expect: Token) -> bool {
    let mut clo = itr.clone();
    if let Some(actual) = clo.next() {
        if *actual == expect {
            itr.next();
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn expr(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    assign(token)
}

fn assign(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    let mut result = equality(token);
    if consume(token, Assign) {
        result = assign(token);
    }
    result
}

fn equality(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    let mut result = relation(token);
    loop {
        if consume(token, Equal) {
            result = relation(token);
        } else if consume(token, NotEq) {
            result = relation(token);
        } else {
            break;
        }
    }
    result
}

fn relation(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    let mut result = add(token);
    loop {
        if consume(token, Smaller) {
            result=new_binary(Token::Smaller,result,add(token));
        } else if consume(token, SmallOrEq) {
            result=new_binary(Token::SmallOrEq, result, add(token));
        //今は左辺右辺入れ替えるけど、参照を実装するならこれは出来ない
        } else if consume(token, Bigger) {
            result=new_binary(Token::Smaller,add(token) , result);
        } else if consume(token, BigOrEq) {
            result=new_binary(Token::SmallOrEq,add(token), result);
        } else {
            break;
        }
    }
    result
}

fn add(token: &mut std::slice::Iter<Token>)->Box<Node>{
    let mut result=mul(token);
    loop {
        if consume(token,Add){
            result=new_binary(Add, result, unary(token));
        }else if consume(token,Sub){
            result=new_binary(Sub, result, unary(token));
        }else{
            break;
        }
    }
    result
}

fn mul(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    let mut result = unary(token);
    loop {
        if consume(token, Mul) {
            ////eprintln!("mul　から　MUL");
            result = new_binary(Token::Mul, result, unary(token))
        } else if consume(token, Div) {
            //eprintln!("mul　から　DIV");
            result = new_binary(Token::Div, result, unary(token));
        } else {
            //eprintln!("mul　から　break");
            return result;
        }
    }
}

fn unary(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    if consume(token, Add) {
        return unary(token);
    }
    if consume(token, Sub) {
        return new_binary(Sub, new_num(0), unary(token));
    }
    return prim(token);
}

fn prim(token: &mut std::slice::Iter<Token>) -> Box<Node> {
    if consume(token, LeftRound) {
        let result = expr(token);
        if consume(token, RightRound) {
            //eprintln!("そう！！！");
            return result;
        } else {
            panic!("ん？");
        }
    }

    if let Some(nu) = consume_number(token) {
        //eprintln!("prim　から　num");
        new_num(nu)
    } else {
        panic!("なにかおかしい");
    }
}

#[derive(PartialEq, Clone)]
pub enum Node {
    Num(i32),
    Ope(BinaryNode),
}

pub fn new_num(num: i32) -> Box<Node> {
    Box::new(Node::Num(num))
}

fn new_binary(wh: lexer::Token, lf: Box<Node>, rt: Box<Node>) -> Box<Node> {
    Box::new(Node::Ope(_new_op_an_no(wh, lf, rt)))
}

#[derive(PartialEq, Clone)]
pub struct BinaryNode {
    pub what: lexer::Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

fn _new_op_an_no(wh: lexer::Token, lf: Box<Node>, rt: Box<Node>) -> BinaryNode {
    BinaryNode::_new(wh, lf, rt)
}

impl BinaryNode {
    fn _new(wh: lexer::Token, lf: Box<Node>, rt: Box<Node>) -> Self {
        BinaryNode {
            what: wh,
            left: lf,
            right: rt,
        }
    }
}
