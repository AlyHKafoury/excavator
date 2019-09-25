pub mod vm;
pub mod instruction;
pub mod repl;

fn main() {
    let mut repl = repl::Repl::new();
    repl.run();
}
