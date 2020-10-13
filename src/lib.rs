extern crate byteorder;

pub fn foo() {
    println!("Foo")
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
