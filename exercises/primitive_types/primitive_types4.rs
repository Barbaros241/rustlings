// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

fn main(){
    let a = [1, 2, 3, 4, 5];
    let my_slice = slice_out_of_array(&a);
    println!("{}",my_slice[1]);
}
//#[test]
fn slice_out_of_array(a:&[i32;5]) -> &[i32] {
    let nice_slice = &a[1..4];
    nice_slice
 //   assert_eq!([2, 3, 4], nice_slice)
}
