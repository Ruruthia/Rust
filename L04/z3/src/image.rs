use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8 
}

impl Pixel {
    fn format(&self) -> String{
        format!("{} {} {}\n", self.r.to_string(), self.g.to_string(), self.b.to_string())
    }
}

pub struct Image {
    pub height: usize,
    pub width: usize,
    pub image_body: Vec<Vec<Pixel>>, 
}

impl Image{

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel){
        // TODO: check if x < height & w < width
        assert!(x < self.height);
        assert!(y < self.width);
        self.image_body[x][y] = pixel;
    }

    pub fn save_to_file(&self, file_path: &str){

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
