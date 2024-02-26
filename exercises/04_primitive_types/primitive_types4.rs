// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // 起始索引 是切片的第一個位置，而 結束索引 在索引結尾之後的位置（所以不包含此值）

    assert_eq!([2, 3, 4], nice_slice)
}
