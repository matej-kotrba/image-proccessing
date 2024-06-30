use image::{GenericImage, GenericImageView, Rgba};
use image::io::Reader as ImageReader;

type Dimension = (u32, u32);

struct Pixel {
  color: Rgba<u8>,
}

impl Pixel {
  fn new(color: Rgba<u8>) -> Self {
    Self{color}
  }
}

pub struct Image {
  dimension: Dimension,
  pixels: Vec<Vec<Pixel>>
}

impl Image {
  pub fn new(src: String) -> Option<Self> {
    let img = ImageReader::open(src).unwrap().decode().unwrap();
    // let reader = image::load_from_memory(include_bytes!("../hotd.png"));
    let mut pixels = Vec::<Vec<Pixel>>::new();

    for y in 0..img.height() {
      pixels.push(Vec::new());
      for x in 0..img.width() {
        pixels[y as usize].push(Pixel::new(img.get_pixel(x, y)));         
      }
    }

    return Some(Self {
      dimension: img.dimensions(),
      pixels,
    })
  }
  pub fn save(&self, src: String) {
    let mut dynamic_image = image::DynamicImage::new(self.dimension.0, self.dimension.1, image::ColorType::Rgba8);
    for y in 0..self.dimension.1 {
      for x in 0..self.dimension.0 {
        dynamic_image.put_pixel(x, y, self.pixels[y as usize][x as usize].color);
      }
    }

    let res = dynamic_image.save(src);
    match res {
      Ok(r) => {
        println!("OK")
      }
      Err(_) => {
        println!("ERROR")
      }
    }
  }
}