use bmp::*;

pub fn load_map() {
    let bmp = open("assets/images/img.bmp").expect("Couldn't load BMP.");

    for i in 0..bmp.get_width() {
        for j in 0..bmp.get_height() {
            match bmp.get_pixel(i, j) {
                Pixel { r: 0, g: 0, b: 0 } => {
                    print!("Black");
                }
                Pixel { r: 255, g: 0, b: 0 } => {
                    print!("Red");
                }
                Pixel { r: 0, g: 255, b: 0 } => {
                    print!("Green");
                }
                Pixel { r: 0, g: 0, b: 255 } => {
                    print!("Blue");
                }
                Pixel { r: 255, g: 255, b: 255 } => {
                    print!("Black");
                }
                _ => print!("Other"),
            };
        }
    }
}
