use rust_vec::MyVec;

fn main() {
    let mut vec: MyVec<i32> = MyVec::<i32>::new();
    vec.push(1i32);
    vec.push(1i32);
    vec.push(1i32);
    vec.push(1i32);

    vec.push(1i32);
    vec.push(1i32);
}
