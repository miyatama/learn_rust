pub fn apply( img: DynamicImage, scale: f64) -> Result<Result<DynamicImage, CustomError>, anyhow::Error> {
    log::debug!("rotate::apply");
    let (width, height) = img.dimensions();
    let new_width = (width as f64 * sale).trunc() as u32;
    let new_height = (height as f64 * sale).trunc() as u32;
    let mut result_pixels: Vec<Rgba<u8>> = Vec::with_capacity((new_width * new_height) as usize);
    let mut hm: HashMap<usize, Rgba<u8>> = HashMap::new();
    let calc_new_pos = |x: f64, y: f64| {
        (
            (x * scale).trunc() as u32,
            (y * scale).trunc() as u32,
        )
    };
    for y in 0..height {
        for x in 0..width {
            let (new_x, new_y) = calc_new_pos(x as f64, y as f64);
            let (new_x, new_y) = (new_x.trunc(), new_y.trunc());
            let pos = (new_x + x_pad + (new_y + y_pad) * new_width as f64) as usize;
            let pixel = img.get_pixel(x, y).0;
            // result_pixels[pos] = Rgba([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8, 255])
            hm.insert(
                pos,
                Rgba([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8, 255]),
            );
        }
    }
 }