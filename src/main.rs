extern crate rustty;

use rustty::terminal::Terminal;

fn main() {
    let mut term = Terminal::new();
    term.update_size();
    println!("{} {}", term.rows(), term.cols());
    term.update_size();
    println!("{} {}", term.rows(), term.cols());
    println!("At end");
}