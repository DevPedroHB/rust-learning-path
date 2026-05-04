struct Width(u32);
struct Height(u32);

fn make_rectangle(width: Width, height: Height) {
  println!("Width: {}, height: {}", width.0, height.0);
}

fn main() {
  make_rectangle(Width(1220), Height(768));
}
