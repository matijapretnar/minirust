fn main() {
    let mut frame = minirust::StackFrame::new();
    let stmt = minirust::Statement::fibonacci(10);
    println!("{frame}");
    println!("{stmt}");
    let output = stmt.run(&mut frame, &stmt);
    for msg in output.printouts {
        println!("{msg}");
    }
    println!("{frame}");
}
