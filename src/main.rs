mod fun;


pub fn main() {
    let mut kami = fun::Person{
      name: String::from("Kami"),
      age: age_gen(),
      gender: String::from("yuh uh"),
  
    };
  
    let mut deftioon = fun::Person{
      name: String::from("Deftioon"),
      age: 255, // oh
      gender: String::from("yuh uh"),
    };
  
    deftioon.name = String::from("Deftiboob");
    kami.name = String::from("Nyami")
}


fn age_gen() -> u8 {
  println!("I am however old deftioon wants me to be!");    
  return (12+21)/2; 
}
