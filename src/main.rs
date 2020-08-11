#[macro_use]
extern crate clap;

mod ast;
mod env;
mod eval;
mod parse;

fn main() {
    let matches = clap_app!(alone =>
        (version:   crate_version!())
        (author:    crate_authors!())
        (about:     crate_description!())
        (@arg file: "source file")
    )
    .get_matches();

    if matches.is_present("file") {
        let source =
            std::fs::read_to_string(matches.value_of("file").unwrap()).expect("Can't read file");
        print(eval::eval(parse::parse(&source)));
    } else {
        println!("{}", crate_description!());
        let mut env = env::make_global_env();
        loop {
            print(eval::eval_with_env(read(), &mut env))
        }
    }
}

fn read() -> ast::Expr {
    use std::io::prelude::*;
    let mut buf = String::new();

    print!("Alone > ");
    std::io::stdout().flush().expect("Can't flush stdout");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Can't read from stdin");
    parse::parse(&buf)
}

fn print(result: eval::EvalResult) {
    match result {
        Ok(value) => println!("{}", value),
        Err(e) => eprintln!("Error! {}", e),
    }
}
