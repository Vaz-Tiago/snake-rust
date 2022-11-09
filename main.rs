
fn main() {
  let tup:(&str, f32, u8)  = ("Hello", 3.12, 1);
  println!("{}", tup.1);

  let (string, float, number) = tup;
  println!("{}", string);
  println!("{}", float);
  println!("{}", number);

  let x = [1, 5, 9, 10];
  println!("{}", x[2]);

  let y = [2; 4]; // [2,2,2,2]
  println!("{}", y[1]);
}