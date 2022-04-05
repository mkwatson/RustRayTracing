fn main() {

  // Image

  let image_width = 256;
  let image_height = 256;

  // Render

  print!("P3\n{} {}\n255\n", image_width, image_height);

  let b = 0.25;
  for j in (0..image_height).rev() {
    eprint!("\rScanlines remaining: {}", j);
    for i in 0..image_width {
      let r = f64::from(i) / f64::from(image_width-1);
      let g = f64::from(j) / f64::from(image_height-1);

      let ir = (255.999 * r) as i32;
      let ig = (255.999 * g) as i32;
      let ib = (255.999 * b) as i32;

      println!("{} {} {}", ir, ig, ib);
    }
  }
  eprint!("\nDone.\n");
}