pub trait Animal {
    fn run(&self, visitor: &impl Visitor) {

    }
}

pub struct Dog;
pub struct Cat;

impl Animal for Cat {
    fn run(&self, visitor: &impl Visitor) {
        visitor.run_dog();
    }
}

impl Animal for Dog {
    fn run(&self, visitor: &impl Visitor) {
        visitor.run_cat();
    }
}

pub trait Visitor {
    fn run_dog(&self);
    fn run_cat(&self);
}

pub struct DefaultVisitor;

impl Visitor for DefaultVisitor {
    fn run_dog(&self) {
        println!("DefaultVisitor for Dog.")
    }

    fn run_cat(&self) {
        println!("DefaultVisitor for Cat.")
    }
}