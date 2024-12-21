fn main() {
	let s1 = String::from("Hello World!");
	print_value(&s1);
	println!("{}", &s1);
	print_loop(&s1);
}

fn print_value(s: &str) {
	println!("{s}");
}

fn print_loop(s: &str) {
	for c in s.chars() {
		println!("{}", c);
	}
}
