use minirust::{expr, statement, Function};

fn main() {
    let mut state = minirust::State::new();
    let stmt = statement!(print(gcd(42, 15)));
    state.add_function(Function::gcd());
    println!("{stmt}");
    stmt.run(&mut state);
    for msg in state.output() {
        println!("{msg}");
    }
    println!("{state}");
}
