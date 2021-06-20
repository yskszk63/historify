mod sub {
    pub use std::println as puts;
}
use sub::puts;

fn main() {
    use std::rc::Rc;

    let _ = Rc::new(0);
    puts!("Hello, world! {:?}", Rc::new(0));
}
