use std::mem::size_of;

fn main() {
    assert_eq!(
        size_of::<Vec<i32>>(),
        size_of::<usize>() * 3
    );

    println!("{}",size_of::<Vec<i32>>());

    let mut xs = vec![1,2,3];
    xs.push(4);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[2], 3);
    println!("{}", xs.capacity());

    let ys;
    ys = xs.clone();
}
