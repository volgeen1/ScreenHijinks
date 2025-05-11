pub struct Circles {
    circles: Vec<(i32, i32, i32)>
}

impl Circles {
    pub fn new() -> Circles {
        let vec_circles = vec![(10, 10, 10)];
        Circles { circles: (vec_circles) }
    }
}