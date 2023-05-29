use rand::Rng;

#[derive(Clone, Copy, Default)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    fn random() -> Self {
        Self {
            red: rand::thread_rng().gen_range(0..255),
            green: rand::thread_rng().gen_range(0..255),
            blue: rand::thread_rng().gen_range(0..255),
        }
    }
}

/// Interprets the supplied vector as an `n x n` matrix, and rotates it 90
/// degrees counter-clockwise.
///
/// # Panics
/// If the supplied vector is not `n x n` length.
fn rotate_vec_counter_clockwise(vec: &mut [Pixel], frame_width: usize) {
    let mut rotated_vec = vec![Pixel::default(); vec.len()];

    // The rotation is achieved by mapping a row in the interpretted input matrix,
    // to a column in the rotated interpretted matrix.
    for i in 0..frame_width {
        for (normal_j, rev_j) in (0..frame_width).rev().enumerate() {
            // On the first iteration the index on rotated_vec is
            // frame_width * (frame_width - 1) + 0 which represents the
            // very last index of the 0th column of the interpretted matrix.
            // Due to the inner loop providing both a reverse and normal index
            // each iteration of the inner loop uses the reverse index to up one row up,
            // but stay in same column, so that row 0 in the input vector is mapped to
            // column 0 (although in reverse order, from end to start); the same applied
            // for the other row/column pairs in the interpretted matrices.
            //
            // A visual representation would be the following; x is the input, y is the output.
            //
            //          x1 x2 x3       y3 y6 y9
            //          x4 x5 x6   =>  y2 y5 y8
            //          x7 x8 x9       y1 y4 y7
            //
            rotated_vec[frame_width * rev_j + i] = vec[frame_width * i + normal_j];
        }
    }

    vec.clone_from_slice(&rotated_vec[..]);
}

pub fn generate_frame(input: &mut Vec<u16>, frames: &mut Vec<gif::Frame>) {
    let black_pixel = Pixel::new(0, 0, 0);
    let gray_pixel = Pixel::new(128, 128, 128);
    let white_pixel = Pixel::new(255, 255, 255);

    let mut pixels: Vec<Pixel> = Vec::new();
    let vector_length = input.len();
    for num in input.iter() {
        // Inserting black pixels for the value of the num
        for _ in 0..*num {
            if num % 2 == 0 {
                pixels.push(black_pixel);
            } else {
                pixels.push(gray_pixel);
            }

            // for disco mode
            // pixels.push(Pixel::random());
        }
        // Doing the same for white, but the negative space instead
        for _ in 0..vector_length - *num as usize {
            pixels.push(white_pixel);
        }
    }

    rotate_vec_counter_clockwise(&mut pixels, vector_length);
    let rgb_vec = convert_pixel_vec_to_rgb_vec(pixels);

    let frame = gif::Frame::from_rgb(vector_length as u16, vector_length as u16, &rgb_vec);
    frames.push(frame);
}

fn convert_pixel_vec_to_rgb_vec(pixel_vec: Vec<Pixel>) -> Vec<u8> {
    let mut rgb_vec = Vec::with_capacity(pixel_vec.len() * 3);

    for pixel in pixel_vec {
        rgb_vec.push(pixel.red);
        rgb_vec.push(pixel.green);
        rgb_vec.push(pixel.blue);
        //rgb_vec.append(&mut vec![pixel.red, pixel.green, pixel.blue]);
    }

    rgb_vec
}
