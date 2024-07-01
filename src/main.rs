use image::FilterType;
use ::image::Rgba;

mod image;

fn main() {
    let mut img = image::Image::new("hotd.png");

    match &mut img {
        Some(data) => {
            // data.filter(FilterType::MergeWithColor(Rgba([84, 66, 245, 255])));
            // data.filter(FilterType::Blur(5));
            data.filter(FilterType::Scale((0.7)));
            data.save("idk.png")
        },
        None => {},
    }
}
