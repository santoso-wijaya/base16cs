use serde::{Deserialize, Serialize};

// Define our own Lab struct rather than using color_space::Lab because we
// need to make a serde::Serialize()d struct.
#[derive(Serialize, Deserialize, Debug)]
pub struct Lab {
    pub l: i32,
    pub a: i32,
    pub b: i32,
}

// TODO: methods on Lab using color_space::Lab
