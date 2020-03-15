fn main() {
    let x = vec![1, 2, 3].into_boxed_slice();
    let static_ref = Box::leak(x);
    static_ref[0] = 4;
    assert_eq!(*static_ref, [4, 2, 3]);
}
