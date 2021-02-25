//! Generate GLSL from generic Eval SDFs

use super::eval::Eval;

pub struct GlslEval {

}

impl Eval for GlslEval {
    type V;
    type V2;
    type V3;
}

pub struct GlslValue {
    
}
