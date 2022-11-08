mod complex;
mod image;
// Based on https://www.codingame.com/playgrounds/2358/how-to-plot-the-mandelbrot-set/mandelbrot-set



fn mandelbrot(c: complex::Complex) -> i32 {
    let mut z = complex::Complex{real:0.0, imaginary:0.0};
    let mut n = 0;
    while z.abs() <= 2.0 && n < 80{
        z = z*z + c;
        n+= 1;
    }
    n
}


fn main() {
    let white_pixel = image::Pixel{r:0,g:0,b:0};
    let test_image_height = 400;
    let test_image_width = 600;
    let mut test_image = image::Image{height:test_image_height, width:test_image_width, image_body:vec![vec![white_pixel; test_image_width];test_image_height]};

    let re_start: f64 = -2.0;
    let re_end: f64 = 1.0;
    let im_start: f64 = -1.0;
    let im_end: f64 = 1.0;

    for x in 0..test_image_height{
        for y in 0..test_image_width {
            let c = complex::Complex{
                real:re_start + (x as f64 / test_image_width as f64)  * (re_end - re_start),
                imaginary:im_start + (y as f64 / test_image_height as f64)  * (im_end - im_start)};
            c.print();
            let m = mandelbrot(c);
            let color = 255 - (m * 255/80) as u8;
            test_image.set_pixel(x, y, image::Pixel{r:color, g:color, b:color});
        }
    }
    test_image.save_to_file("fractal_1.ppm");
}
