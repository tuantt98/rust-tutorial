fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
  let logical: bool = false;
  println!("{}",logical);
  
  let a = 5.6;
  println!("{}",a);

  print_type_of(&a); 

  let b: f32 = 7.0;
  println!("{}",b);

  let c = "c";
  println!("{}",c);
}