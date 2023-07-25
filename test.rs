fn main() {
    println!("Hello, world!");
}

impl<T> Foo for T {
    fn foo(&self) {
        println!("Foo");
    }
}
