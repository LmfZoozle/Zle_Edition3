pub fn err_unknown_token(line:i32,what:&str){
    unsafe {
        super::increment_errors();
    }
    eprintln!();
    super::longline();
    eprintln!("An error occurred during lex.");
    eprintln!("Unknown token '{}' found, at {}th token.",what,line);
    super::longline()
}