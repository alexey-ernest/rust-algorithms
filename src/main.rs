mod sort;

fn main() {
	let mut v = vec![3, 2, 1];
	sort::insertion(&mut v);
    println!("{:?}", v);
}
