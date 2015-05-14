use std::env;
use rand;

pub fn get_fn() -> Option<fn(i16, i16) -> bool> {
    match env::args().nth(1) {
        Some(arg) => match &*arg {
            "random" => Some(random),
            "diehard" => Some(diehard),
            "gosper_glider" => Some(gosper_glider),
            _ => None
        },
        None => Some(random)
    }
}

pub fn random(_: i16, _: i16) -> bool {
    rand::random::<u8>() % 2 == 0
}

pub fn diehard(x: i16, y: i16) -> bool {
    match (x, y) {
        (31, 22) => true,
        (32, 21) => true,
        (32, 22) => true,
        (36, 21) => true,
        (37, 21) => true,
        (38, 21) => true,
        (37, 23) => true,
        _ => false
    }
}

pub fn gosper_glider(x: i16, y: i16) -> bool {
    match (x, y) {
        (1, 4) => true,
        (1, 5) => true,
        (2, 4) => true,
        (2, 5) => true,

        (11, 3) => true,
        (11, 4) => true,
        (11, 5) => true,

        (12, 2) => true,
        (12, 6) => true,

        (13, 1) => true,
        (13, 7) => true,
        (14, 1) => true,
        (14, 7) => true,

        (15, 4) => true,

        (16, 2) => true,
        (16, 6) => true,

        (17, 3) => true,
        (17, 4) => true,
        (17, 5) => true,

        (18, 4) => true,

        (21, 5) => true,
        (21, 6) => true,
        (21, 7) => true,

        (22, 5) => true,
        (22, 6) => true,
        (22, 7) => true,

        (23, 4) => true,
        (23, 8) => true,

        (25, 3) => true,
        (25, 4) => true,
        (25, 8) => true,
        (25, 9) => true,

        (35, 6) => true,
        (35, 7) => true,
        (36, 6) => true,
        (36, 7) => true,

        _ => false
    }
}
