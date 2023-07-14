fn col2i(s: &str) -> usize {
    match &s[0..1] {
        "a" => 7,
        "b" => 6,
        "c" => 5,
        "d" => 4,
        "e" => 3,
        "f" => 2,
        "g" => 1,
        "h" => 0,
        _ => 100, //panic!(),
    }
}

// convert board move coordinates "d2d4" to tuple form
pub fn str2move(s: &str) -> (usize, usize) {
    let fy: usize = s[1..2].parse::<usize>().unwrap() - 1;
    let fx: usize = col2i(&s[0..1]);
    let ty: usize = s[3..4].parse::<usize>().unwrap() - 1;
    let tx: usize = col2i(&s[2..3]);
    (fx * 8 + fy, tx * 8 + ty)
}
