#[derive(Clone, Debug, PartialEq)]
pub enum GroundType {
    Normal,
    Custom(String, f32)
}

impl std::convert::Into<f32> for &GroundType {
    fn into(self) -> f32 {
        match self {
            GroundType::Normal => 1.0,
            GroundType::Custom(_, f) => *f,
        }
    }
}
