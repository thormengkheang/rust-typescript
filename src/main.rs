fn main() {
    let list: Vec<_> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
    println!("{:?}", list);
}
