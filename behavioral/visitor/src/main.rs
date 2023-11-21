use visitor::*;

struct CustomVisitor;

impl Visitor for CustomVisitor {
    fn run_dog(&self) {
        println!("CustomVisitor for Dog.")
    }

    fn run_cat(&self) {
        println!("CustomVisitor for Cat.")
    }
}
fn main() {
    let cat = Cat;
    let dog = Dog;
    
    let dvisitor = DefaultVisitor;
    let cvisitor = CustomVisitor;

    dog.run(&dvisitor);
    cat.run(&dvisitor);

    dog.run(&cvisitor);
    cat.run(&cvisitor);
}
