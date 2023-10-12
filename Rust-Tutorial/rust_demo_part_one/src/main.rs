struct Text {
   name: String,
   message: String,
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
   /* Demo dereferening and for loops 
   */
   let name = String::from("Sylvie");
   let message = String::from("This is the message");
   let text = Text::initialize_text(name,message);
   text.print_text();
   println!("{}", name);
}

// Learn how to impliment an ood detail 
/* WE are going to build a tic tac toe game together
 step one is we are going to learn how to use a struct
  and we are going to print a hello world within a struct

*/

// Hello world
// with structs
// print string literal compiler error