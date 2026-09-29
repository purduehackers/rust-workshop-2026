  fn main() {
      let mut a: String = utils::read_string();

      let b = &a;
      let c = &a;
      let d = &mut a;

      // ... a bunch of complicated code

      println!("{b} {c} {d}");
  }
