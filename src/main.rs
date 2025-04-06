fn main() {
    let mut frame = minirust::StackFrame::new();
    let stmt = minirust::Statement::fibonacci(10);
    let output = stmt.run(&mut frame);
    for msg in output.printouts {
        println!("{msg}");
    }
}
