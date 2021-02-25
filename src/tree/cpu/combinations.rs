use super::eval::{Eval, Real1 as _};

pub fn union<E: Eval>(lhs: E::R1, rhs: E::R1) -> E::R1 {
    lhs.min(rhs)
}

pub fn smooth_union<E: Eval>(lhs: E::R1, rhs: E::R1, k: E::R1) -> E::R1 {
    let h = (((lhs.clone() - rhs.clone()) / k.clone()) * 0.5 + 0.5)
        .clamp(E::R1::new(0.0), E::R1::new(1.0));

    h.mix(rhs, lhs) - k * h.clone() * (-h + 1.0)
}

pub fn intersection<E: Eval>(lhs: E::R1, rhs: E::R1) -> E::R1 {
    lhs.max(rhs)
}

pub fn smooth_intersection<E: Eval>(lhs: E::R1, rhs: E::R1, k: E::R1) -> E::R1 {
    let h = (((lhs.clone() - rhs.clone()) / k.clone()) * -0.5 + 0.5)
        .clamp(E::R1::new(0.0), E::R1::new(1.0));

    h.mix(rhs, lhs) + k * h.clone() * (-h + 1.0)
}

pub fn subtraction<E: Eval>(lhs: E::R1, rhs: E::R1) -> E::R1 {
    (-lhs).max(rhs)
}

pub fn smooth_subtraction<E: Eval>(lhs: E::R1, rhs: E::R1, k: E::R1) -> E::R1 {
    let h = (((lhs.clone() - rhs.clone()) / k.clone()) * -0.5 + 0.5)
        .clamp(E::R1::new(0.0), E::R1::new(1.0));

    h.mix(rhs, -lhs) + k * h.clone() * (-h + 1.0)
}

// pub fn translate<E: Eval>(d: E::R1, by: E::R13) -> E::R1 {

// }
