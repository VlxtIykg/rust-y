pub struct Person{
  pub name: String,
  pub age: u8,
  pub gender: String
}

impl Person{
  fn print(&self){
    println!("Name: {}", self.name);
    println!("Age: {}", self.age);
    println!("Gender: {}", self.gender);
  }
  fn change_name(&mut self, new_name: String){
    self.name = new_name;
  }
}

fn main(){

}