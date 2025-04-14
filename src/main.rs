fn main() {
    let mut state = minirust::State::new();
    let stmt = minirust::Statement::fibonacci(10);
    println!("{stmt}");
    stmt.run(&mut state);
    for msg in state.output() {
        println!("{msg}");
    }
    println!("{state}");
}
