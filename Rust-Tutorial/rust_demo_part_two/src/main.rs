struct Hello<'a> {
   name: &'a str,
   message: &'a str
}
impl<'a> Hello<'a> {
  fn initialize_hello(name: &'a str, message: &'a str) -> Hello<'a> {
     Hello {
        name,
        message
     }
  }
  fn display(&self) {
     println!("{} {}!", self.message, self.name);
  }
}
fn main() {
   let my_name = "Emma";
   let my_message = "Hello";
   let hello = Hello::initialize_hello(
     my_name, 
     my_message);
   hello.display();
}