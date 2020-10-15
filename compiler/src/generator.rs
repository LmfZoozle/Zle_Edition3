use super::parser::*;
use super::*;

pub fn resolve_prival(from:Vec<PriorityVal>)->Vec<PriorityVal>{
    let mut result=Vec::new();
    
    let mut count=from.len()-1;
    loop {
        match from[count].get_level() {
            parser::Priority::Lev0=>{

            }

            parser::Priority::Lev1=>{

            }
        
            parser::Priority::Lev2=>{

            }
        }



        if count==0{
            break;
        }
        count-=1;
    }

    result
}


pub fn prival_into_bytecode(mut prival: Vec<PriorityVal>) {

}
