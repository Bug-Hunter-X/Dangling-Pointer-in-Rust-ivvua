fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using a raw pointer, we index into the vector
    v[0] = 4; 
    println!("{:?}", v);
} 