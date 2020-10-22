use super::lexer::*;
use super::*;


pub enum Node {
    Num(lexer::Token),
    Ope(OpeAndNode),
}

struct OpeAndNode{
    what: lexer::Token,
    left: Box<Node>,
    right: Box<Node>,
}

impl Node {
    fn _new(wh:lexer::Token, lf:Box<Node>,rt:Box<Node>)->Self{
        Node{
            what:wh,
            left:lf,
            right:rt,
        }
    }
    fn new_node(wh:lexer::Token, lf:Box<Node>,rt:Box<Node>)->Box<Node>{
        Box::new(Node::_new(wh, lf, rt))
    }
}


pub fn token_into_tree(token:Box<Node>) -> Box<Node> {
    let mut result=Box::new(x);

}








pub mod debug {
    fn declare_debug(name: &str) {
        eprintln!("This is Debug Fn !!");
        eprintln!("From: {}", name);
    }
    use super::super::lexer;
}