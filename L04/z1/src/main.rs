use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Clone)]
struct Pixel {
   r: u8,
   g: u8,
   b: u8 
}

impl Pixel {
    fn format(&self) -> String{
        format!("{} {} {}\n", self.r.to_string(), self.g.to_string(), self.b.to_string())
    }
}

struct Image {
    height: usize,
    width: usize,
    image_body: Vec<Vec<Pixel>>, 
}

impl Image{

    fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel){
        // TODO: check if x < height & w < width
        assert!(x < self.height);
        assert!(y < self.width);
        self.image_body[x][y] = pixel;
    }

    fn save_to_file(&self, file_path: &str){

        // Preparing file content
        let header = format!("P3\n{0} {1}\n255\n", self.width, self.height);
        let mut body: String = "".to_string();
        for row in &self.image_body{
            for pixel in row {
                body.push_str(&pixel.format());
            }
        }
        let data = format!("{}{}", header, body);

        // Writing to file
        let f = File::create(file_path).expect("Unable to create file!");
        let mut f = BufWriter::new(f);
        f.write_all(data.as_bytes()).expect("Unable to write data!");
    }

}
fn main(){
    let white_pixel = Pixel{r:255,g:255,b:255};
    let test_image_height = 100;
    let test_image_width = 100;
    let mut test_image = Image{height:test_image_height, width:test_image_width, image_body:vec![vec![white_pixel; test_image_width];test_image_height]};
    test_image.set_pixel(1, 1, Pixel{r:128, g:10, b:10});
    test_image.set_pixel(2, 2, Pixel{r:10, g:128, b:10});
    test_image.set_pixel(3, 3, Pixel{r:10, g:128, b:128});
    test_image.save_to_file("test_img.ppm");
}
