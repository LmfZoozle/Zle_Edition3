use super::*;
use super::lexer::*;
/*pub enum Tree{
    
}

pub fn token_into_tree(tokens:Vec<lexer::Token>){

}
*/


//木構造は前失敗したので
//優先度をつけた配列で挑んで見る
//とりあえずLevNのenumでやる
//->u32でやるよりenumのが整理しやすいかと思ったけど、()とかで優先度が変わるなら
//enumじゃ厳しいかな
pub enum Priority{
    Lev0,
    Lev1,
    Lev2,
}

pub struct PriorityVal{
    level:Priority,
    what:lexer::Token,
}

impl PriorityVal{
    pub fn new(lev:Priority,val:lexer::Token)->Self{
        PriorityVal{
            level:lev,
            what:val,
        }
    }

    pub fn set_level(&mut self,lev:Priority){
        self.level=lev;
    }

    pub fn get_level(&self)->&Priority{
        &self.level
    }
    pub fn set_what(&mut self,wh:lexer::Token){
        self.what=wh;
    }
    pub fn get_what(&self)->&lexer::Token{
        &self.what
    }
}

pub fn prival_into_sorted()->Vec<lexer::Token>{
    let sorted=Vec::new();
    
    sorted
}

pub fn token_into_priorty(tokens:Vec<lexer::Token>)->Vec<PriorityVal>{
    let mut result=Vec::new();
    for run in tokens{
        match run{
            Token::NUM(n)=>result.push(PriorityVal::new(Priority::Lev0,lexer::Token::NUM(n))),
            Token::ADD=>result.push(PriorityVal::new(Priority::Lev1, lexer::Token::ADD)),
            Token::SUB=>result.push(PriorityVal::new(Priority::Lev1, lexer::Token::SUB)),
            Token::MUL=>{
                result.push(PriorityVal::new(Priority::Lev2, lexer::Token::MUL));
            }
            Token::DIV=>{
                result.push(PriorityVal::new(Priority::Lev2, lexer::Token::DIV));
            }
        }
    }

    result
}
