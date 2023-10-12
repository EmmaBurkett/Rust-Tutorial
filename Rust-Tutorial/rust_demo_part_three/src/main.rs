struct Hello {
    a: i32,
    b: i32
}
impl Hello {
	fn initialize_hello(a: i32, b: i32) -> Hello {
		Hello {
			a,
			b
		}
	}
    fn game(&self) {
        
    }

}
fn main() {
    let a: i32;
    let b = 0;
    let hello = Hello::initialize_hello(
		a, 
		b);
}