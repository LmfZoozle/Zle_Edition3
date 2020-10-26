pub mod from_lex;
pub mod from_parse;

static mut ERRORS: i32 = 0;
static mut WARNS: i32 = 0;

unsafe fn increment_errors() {
    ERRORS += 1;
}

unsafe fn increment_warns() {
    WARNS += 1;
}

pub fn analysis_result()->bool {
    longline();
    unsafe {
        if ERRORS == 0 && WARNS == 0 {
            eprintln!("No errors or warnings are found.");
            eprintln!("Now generating the bytecode...");
            longline();
            true
        } else if ERRORS == 0 {
            eprintln!("{} warnings are found.",WARNS);
            eprintln!("Now generating the bytecode...");
            longline();
            true
        } else {
            eprintln!("{} errors and {} warnings are found.",ERRORS,WARNS);
            eprintln!("Aborting due to {} problems...",ERRORS+WARNS);
            longline();
            false
        }
    }
}

fn longline() {
    eprintln!("---------------------------------------");
}
