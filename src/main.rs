fn main() {
  println!("Hello kuro");
  println!("{0}, {1}","b",  "a");
  println!("{a:>width1$} {b} {c}", a=10,c=30, b = 20, width1=20);
  println!("{:b} of {:b} people know binary, the other half doesn't", 10, 2);
  println!("{number:>width$}", number=1, width=6);
  println!("{number:2>width$}", number=1, width=6);
  //println!("My name is {0}, {1} {0}", "Bond");

  #[allow(dead_code)]
  struct Structure(i32);
  //println!("This struct `{}` won't print...", Structure(3));
}