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
