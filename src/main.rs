mod image;

fn main() {
    let img = image::Image::new(String::from("hotd.png"));

    match img {
        Some(data) => {
            data.save(String::from("idk.png"))
        },
        None => {},
    }
}
