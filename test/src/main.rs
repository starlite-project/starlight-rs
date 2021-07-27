fn main() {
    let val: &'static i32 = Box::leak(Box::new(10));

    dbg!(val);
}
