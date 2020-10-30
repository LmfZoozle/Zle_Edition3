mod lexer;
mod parser;
//mod generator;
mod error;


const PATH: &str = "~/Code/Zle/Edition3/test";

fn main() {
    eprintln!("zle compiler activated.");
    let  option: Vec<String> = std::env::args().collect();
    if option.len() < 2 {
        eprintln!("lack");
        eprintln!("test.zle?");
        std::process::exit(2);
        //let timer=std::time::Duration::from_secs(1);
        //std::thread::sleep(timer);
        //option.push(format!("{}{}",PATH,"/test.zle"));
        //eprintln!("{}",&option[1]);
    }
    let code=if let Ok(a)=std::fs::read_to_string(&option[1]){
        a
    }else{
        eprintln!("No such a file.");
        std::process::exit(2);
    };
    eprintln!("This is nightly build.");
    let token=lexer::read_into_token(code);
    eprintln!("トークナイズできた");
    let a=parser::expr(&mut token.iter());
    eprintln!("パースできた");
    if error::analysis_result(){
    //    generator::gen_asm(a);
        eprintln!("コード生成したよ");
        eprint!("結果: ")
    }
}
