struct Text {
   name: String,
   message: String
}

impl Text {
   pub fn initialize_text(
      name: String,
      message: String
   ) -> Text {
      Text {
         name,
         message
      }
   }

   pub fn run_program(&mut self) {
      self.define_strings();
      self.print_text();
   }

   pub fn define_strings(&mut self) {
      self.name = "Emma".to_string();
      self.message = "How much wood could a woodchuck chuck if a woodchuck could chuck wood?".to_string();
   }

   pub fn print_text(& self) {
      println!("{} There is a message for you:\n{}", self.name, self.message);

   }
}

fn main() {
   //Define ALL your variables here
   let name = String::new();
   let message = String::new();
   
   let mut text = Text::initialize_text(
      name,
      message
   );

   text.run_program();
}
