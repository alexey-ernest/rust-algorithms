mod sort;

fn main() {
	let mut v = vec![3, 2, 1];
	sort::quick(&mut v);
    println!("{:?}", v);
}
