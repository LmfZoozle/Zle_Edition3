use super::*;
//mod lex_err;

#[derive(Clone, Copy, PartialEq)]
pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Brackets {
    LeftRound,
    RightRound,
}

#[derive(Clone,Copy,PartialEq)]
pub enum Relation{
    Equal,
    Not,
    NotEq,
    Assign,
    Bigger,
    BigOrEq,
    Smaller,
    SmallOrEq,
}


#[derive(Clone, Copy, PartialEq)]
pub enum Token {
    Ope(Operators),
    Num(i32),
    Braket(Brackets),
    Ident(char),
    Relate(Relation),
    Return,
    WhiteSpace,
}

//軽量化した
fn as_long_as_num(input: &String) -> Option<i32> {
    let mut numerics=String::new();
    for run in input.chars(){
        if run.is_digit(10){
            numerics.push(run);
        }else{
            break;
        }
    }
    if let Ok(num)=numerics.parse::<i32>(){
        Some(num)
    }else{
        None
    }
}

/*fn check_relate(itr:&mut std::slice::Iter<char>)->Option<Relation>{
    let mut clo=itr.clone();

}*/

fn solve_num_stack(flag:&mut bool,numerics:&mut String,cunter:&mut i32,vctr:&mut Vec<Token>,what:Token){
    if *flag{
        *flag=false;
        vctr.push(Token::Num(numerics.parse::<i32>().unwrap()));
        numerics.clear();
        *cunter+=1;
    }
    vctr.push(what);
    *cunter+=1;
}

pub fn read_into_token(input: String) -> Vec<Token> {
    let mut result = Vec::new();
    let aa: Vec<char> = input.chars().collect();
    let mut iter = aa.iter();
    let mut numerics = String::new();
    let mut numflag = false;
    let mut tokencount=0;
    loop {
        if let Some(run) = iter.next() {
            match run {
                _ if run.is_numeric() => {
                    eprintln!("すうじ！");
                    numerics.push(*run);
                    numflag= true;
                }

                _ if 'a'<=*run&&*run<='z'=>{
                    eprintln!("いちもじ！");
                    solve_num_stack(&mut numflag, &mut numerics,
                         &mut tokencount, &mut result, Token::Ident(*run));
                }

                '<'=>{

                }

                '='=>{

                }

                '>'=>{

                }


                '+' => {        
                    solve_num_stack(&mut numflag, &mut numerics,
                         &mut tokencount, &mut result, Token::Ope(Operators::Add));
                }
                '-' => {
                    solve_num_stack(&mut numflag, &mut numerics,
                         &mut tokencount, &mut result, Token::Ope(Operators::Sub));
                }
                '*' => {
                    solve_num_stack(&mut numflag, &mut numerics,
                         &mut tokencount, &mut result,Token::Ope(Operators::Mul));
                }
                '/' => {
                    solve_num_stack(&mut numflag, &mut numerics,
                         &mut tokencount, &mut result,Token::Ope(Operators::Div));
                }
                '(' => {
                    solve_num_stack(&mut numflag, &mut numerics,
                         &mut tokencount, &mut result,Token::Braket(Brackets::LeftRound));
                }
                ')' => {
                solve_num_stack(&mut numflag, &mut numerics,
                         &mut tokencount, &mut result,Token::Braket(Brackets::RightRound));
                }
                ' '=>{
                    if numflag {
                        numflag = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();                   
                        tokencount+=1;
                    }
                }
                _ => {
                    if numflag {
                        numflag = false;
                        result.push(Token::Num(numerics.parse::<i32>().unwrap()));
                        numerics.clear();                   
                        tokencount+=1;
                    } 
                    tokencount+=1;
                    error::from_lex::err_unknown_token(tokencount, *run);                    
                }
            }
        } else {
            if numflag{
                result.push(Token::Num(numerics.parse::<i32>().unwrap()));
            }
            break;
        }

    }
    result
}

