trait MyTrait {
    fn get(&self) -> usize;
}

impl<T> MyTrait for T {
    fn get(&self) -> usize {
        0
    }
}

struct Foo {
    value: usize,
}

impl MyTrait for Foo {
    // error: conflicting implementations of trait
    //        `MyTrait` for type `Foo`
    fn get(&self) -> usize {
        self.value
    }
}

fn main() {
    println!("Hello, world!");
}
