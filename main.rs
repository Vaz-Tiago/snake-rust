
fn main() {
  let custom_num = 98_000;    // _ is just for formatting
  let hex_num = 0xfa;         // hexadecimal number
  let bin_num = 0b0010_1011;  // binary number
  let byte_num = b'A';         // byte number

  println!("{}", custom_num); // 98000
  println!("{}", hex_num);    // 250
  println!("{}", bin_num);    // 43
  println!("{}", byte_num);   // 65

  let float_num: f32 = 3.14;
  let float_num_2: f64 = 3.2334327489;

  println!("{}", float_num);
  println!("{}", float_num_2);
}