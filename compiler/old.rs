use super::lexer::*;
use super::*;

fn prim(master:&mut Box<Node>,token:&mut std::slice::Iter<Token>)->Box<Node>{    
    let mut result=token.next().unwrap();
    if let lexer::Token::NUM(a)=result{
        eprintln!("prim　から　num");
        new_node_num(*a)
    }else{
        eprintln!("prim　から　おかしい");
        new_node_num(0)
    }
}


fn mul(master:&mut Box<Node>,token:&mut std::slice::Iter<Token>)->Box<Node>{
    let mut result=prim(master, token);
    loop{
        let a=token.next().unwrap();
        if *a==Token::OPE(Operator::MUL){
            eprintln!("mul　から　MUL");
            result=new_node_ope(Token::OPE(Operator::MUL), result, prim(master, token))
        }else if *a==Token::OPE(Operator::DIV){
            eprintln!("mul　から　DIV");
            result=new_node_ope(Token::OPE(Operator::DIV), result, prim(master, token));
        }else{
            eprintln!("mul　から　break");
            return result;
        }
    }
}

pub fn token_into_tree(master:&mut Box<Node>,token:&mut std::slice::Iter<Token>) -> Box<Node> {
    let mut result=mul(master, token);
    let itr=token.next().unwrap();

    loop{
        let a=token.next().unwrap();
        if *a==Token::OPE(Operator::ADD){
            eprintln!("token　から　ADD");
            result=new_node_ope(Token::OPE(Operator::ADD), result, mul(master,token))
        }else if *a==Token::OPE(Operator::SUB){
            eprintln!("token　から　SUB");
            result=new_node_ope(Token::OPE(Operator::SUB), result, mul(master, token));
        }else{
            eprintln!("token　から　return");
            return result;
        }
    }

}
pub mod debug{
    fn declare_debug(name:&str){
        eprintln!("This is Debug Fn !!");
        eprintln!("From: {}",name);
    }
    use super::super::lexer;
    pub fn show_resolved(dbg:&Vec<&super::PriorityVal>) {
        declare_debug("show_resolved");
//        eprintln!("{}だ",dbg.len());
        for run in dbg{
            match run.get_what() {
                lexer::Token::ADD=>{
                    eprintln!("ADD命令 {:?}",run.get_level());
                }
                lexer::Token::SUB=>{
                    eprintln!("SUB命令 {:?}",run.get_level());
                }
                lexer::Token::MUL=>{
                    eprintln!("MUL命令 {:?}",run.get_level());
                }
                lexer::Token::DIV=>{
                    eprintln!("DIV命令 {:?}",run.get_level());
                }
                lexer::Token::NUM(n)=>{
                    eprintln!("NUM({}) {:?}",*n,run.get_level());
                }
                
            }
        }
    }
}

