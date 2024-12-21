use crate::{ImVec2, ImVec4};

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for ImVec2 {
    fn zero() -> Self {
        ImVec2 { x: 0.0, y: 0.0 }
    }
}

impl From<ImVec2> for [f32; 2] {
    fn from(value: ImVec2) -> Self {
        [value.x, value.y]
    }
}

impl From<[f32; 2]> for ImVec2 {
    fn from(value: [f32; 2]) -> Self {
        ImVec2 {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<mint::Vector2<f32>> for ImVec2 {
    fn from(value: mint::Vector2<f32>) -> Self {
        ImVec2 {
            x: value.x,
            y: value.y,
        }
    }
}

impl PartialEq for ImVec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for ImVec2 {}

impl From<mint::Vector4<f32>> for ImVec4 {
    fn from(value: mint::Vector4<f32>) -> Self {
        ImVec4 {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<[f32; 4]> for ImVec4 {
    fn from(value: [f32; 4]) -> Self {
        ImVec4 {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
        }
    }
}

impl From<ImVec4> for [f32; 4] {
    fn from(value: ImVec4) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}
