#[derive(Clone, PartialEq, Debug)]
pub enum Media {
    Air,
    Custom(String, f32)
}


impl std::convert::Into<f32> for &Media {
    fn into(self) -> f32 {
        match self {
            Media::Air => 1.0,
            Media::Custom(_, f) => *f
        }
    }
}
