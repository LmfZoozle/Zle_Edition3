pub fn err_unknown_token(line:i32,what:char){
    unsafe {
        super::increment_errors();
    }
    super::longline();
    eprintln!("An error occurred during lex.");
    eprintln!("Unknown token '{}' found, at {}th token.",what,line);
    super::longline()
}