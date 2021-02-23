use super::eval::{Eval, Value as _};

pub fn union<E: Eval>(lhs: E::V, rhs: E::V) -> E::V {
    lhs.min(rhs)
}

pub fn smooth_union<E: Eval>(lhs: E::V, rhs: E::V, k: E::V) -> E::V {
    let h = (((lhs.clone() - rhs.clone()) / k.clone()) * 0.5 + 0.5)
        .clamp(E::V::new(0.0), E::V::new(1.0));

    h.mix(rhs, lhs) - k * h.clone() * (-h + 1.0)
}

pub fn intersection<E: Eval>(lhs: E::V, rhs: E::V) -> E::V {
    lhs.max(rhs)
}

pub fn smooth_intersection<E: Eval>(lhs: E::V, rhs: E::V, k: E::V) -> E::V {
    let h = (((lhs.clone() - rhs.clone()) / k.clone()) * -0.5 + 0.5)
        .clamp(E::V::new(0.0), E::V::new(1.0));

    h.mix(rhs, lhs) + k * h.clone() * (-h + 1.0)
}

pub fn subtraction<E: Eval>(lhs: E::V, rhs: E::V) -> E::V {
    (-lhs).max(rhs)
}

pub fn smooth_subtraction<E: Eval>(lhs: E::V, rhs: E::V, k: E::V) -> E::V {
    let h = (((lhs.clone() - rhs.clone()) / k.clone()) * -0.5 + 0.5)
        .clamp(E::V::new(0.0), E::V::new(1.0));

    h.mix(rhs, -lhs) + k * h.clone() * (-h + 1.0)
}

// pub fn translate<E: Eval>(d: E::V, by: E::V3) -> E::V {

// }
