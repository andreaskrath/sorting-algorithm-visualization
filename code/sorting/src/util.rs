use rand::Rng;

pub fn is_sorted(v: &[u16]) -> bool {
    for slice in v.windows(2) {
        if slice[0] > slice[1] {
            return false;
        }
    }

    true
}

pub fn generate_mixed_vector(size: u16) -> Vec<u16> {
    let mut vec: Vec<u16> = Vec::new();
    for num in 1..=size {
        vec.push(num);
    }

    mix_vector(&mut vec);
    vec
}

pub fn generate_random_vector(size: u16) -> Vec<u16> {
    let mut vec: Vec<u16> = Vec::new();
    for _ in 0..size {
        let random_number = rand::thread_rng().gen_range(1..=size);
        vec.push(random_number);
    }

    mix_vector(&mut vec);
    vec
}

pub fn mix_vector(vec: &mut Vec<u16>) {
    for _ in 0..vec.len() * 25 {
        let a = rand::thread_rng().gen_range(0..vec.len());
        let b = rand::thread_rng().gen_range(0..vec.len());
        vec.swap(a, b);
    }
}

fn transpose_vector(vec: &mut Vec<u8>, frame_width: usize) {
    let mut temp_vector: Vec<Vec<u8>> = Vec::new();
    for (_, pixel_line) in vec.chunks_exact(frame_width * 3).enumerate() {
        temp_vector.push(pixel_line.to_vec());
    }

    vec.drain(..);
    let mut index = 0;
    for _ in 0..temp_vector.first().unwrap().len() / 3 {
        for vector in temp_vector.iter().rev() {
            let pixel = &vector[index..index + 3];
            for rgb in pixel {
                vec.push(*rgb);
            }
        }
        index += 3;
    }
}

pub fn generate_frame(input: &mut Vec<u16>, frames: &mut Vec<gif::Frame>) {
    let black_pixel: Vec<u8> = vec![0, 0, 0];
    let gray_pixel: Vec<u8> = vec![128, 128, 128];
    let white_pixel: Vec<u8> = vec![255, 255, 255];

    let mut pixels: Vec<u8> = Vec::new();
    let vector_length = input.len();
    for num in input.iter() {
        // Inserting black pixels for the value of the num
        for _ in 0..*num {
            if num % 2 == 0 {
                pixels.append(&mut black_pixel.clone());
            } else {
                pixels.append(&mut gray_pixel.clone());
            }
        }
        // Doing the same for white, but the negative space instead
        for _ in 0..vector_length - *num as usize {
            pixels.append(&mut white_pixel.clone());
        }
    }

    pixels.reverse();
    transpose_vector(&mut pixels, vector_length);

    let frame = gif::Frame::from_rgb(vector_length as u16, vector_length as u16, &pixels);
    frames.push(frame);
}
