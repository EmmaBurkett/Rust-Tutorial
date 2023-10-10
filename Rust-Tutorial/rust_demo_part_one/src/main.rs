struct Text {
   name: String,
   message: String
}
impl Text {
    pub fn initialize_text(name: String, message: String) -> Text {
        Text {name, message}
   }
   pub fn print_text(&self) {
        println!("{} There is a message for you:\n{}", self.name, self.message);
   }
}

fn main() {
   //Define ALL your variables here
   let name = String::from("Sylvie");
   let message = String::from("This is the message");
   let text = Text::initialize_text(name,message);
   text.print_text();
}
