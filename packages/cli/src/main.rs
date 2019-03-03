fn main() {
	let sum = (0..10000000)
		.zip((0..10000000).skip(1))
		.map(|(a, b)| a * b)
		.filter(|x| {
			if x % 314159 == 0 {
				print!("{}", x);
			}
			x % 3 == 0
		})
		.sum::<u128>();
	println!("\nHello, {}", sum);
}
