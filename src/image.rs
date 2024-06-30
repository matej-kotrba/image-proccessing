use image::{GenericImage, GenericImageView, Rgb, Rgba};
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

pub enum FilterType {
  MergeWithColor(Rgba<u8>),
  Blur(u8)
}

impl Image {
  pub fn new(src: &str) -> Option<Self> {
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
  pub fn save(&self, src: &str) {
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
  pub fn filter(&mut self, filterType: FilterType) {
    match filterType {
      FilterType::MergeWithColor(color) => {
        for y in 0..self.dimension.1 {
          for x in 0..self.dimension.0 {
            let orig_color = self.pixels[y as usize][x as usize].color;
            self.pixels[y as usize][x as usize].color = Rgba(
            [
              (orig_color.0[0] as f32 * (color.0[0] as f32 / 255.0)) as u8,
              (orig_color.0[1] as f32 * (color.0[1] as f32 / 255 as f32)) as u8,
              (orig_color.0[2] as f32 * (color.0[2] as f32 / 255 as f32)) as u8,
              (orig_color.0[3] as f32 * (color.0[3] as f32 / 255 as f32)) as u8
            ])
          }
        }
      },
      FilterType::Blur(value) => {
        let mut new_pixels: Vec<Vec<Pixel>> = Vec::new();

        for y in 0..self.dimension.1 {
          new_pixels.push(Vec::new());

          for x in 0..self.dimension.0 {
            let mut count = 0;
            let mut new_rgba: Rgba<i32> = Rgba([0, 0, 0, 0]);
            for py in 0.max((y as i32)-10)..((self.dimension.1 as i32).min((y as i32)+10)) {
              for px in 0.max((x as i32)-10)..((self.dimension.0 as i32).min((x as i32)+10)) {
                count += 1;
                new_rgba.0[0] += self.pixels[py as usize][px as usize].color.0[0] as i32;
                new_rgba.0[1] += self.pixels[py as usize][px as usize].color.0[1] as i32;
                new_rgba.0[2] += self.pixels[py as usize][px as usize].color.0[2] as i32;
                new_rgba.0[3] += self.pixels[py as usize][px as usize].color.0[3] as i32;
              }
            }
            new_rgba.0[0] = new_rgba.0[0] / count;
            new_rgba.0[1] = new_rgba.0[1] / count;
            new_rgba.0[2] = new_rgba.0[2] / count;
            new_rgba.0[3] = new_rgba.0[3] / count;

            new_pixels[y as usize].push(Pixel{
              color: Rgba([new_rgba.0[0] as u8, new_rgba.0[1] as u8, new_rgba.0[2] as u8, new_rgba.0[3] as u8])
            });
          }
        }
        self.pixels = new_pixels;
      }
    }
  }
  // fn loop_through(&mut self, callback: fn(x: u32, y: u32, pixel: &mut Pixel)) {
  //   for y in 0..self.dimension.1 {
  //     for x in 0..self.dimension.0 {
  //       callback(x, y, &mut self.pixels[y as usize][x as usize]);
  //     }
  //   }
  // }
}