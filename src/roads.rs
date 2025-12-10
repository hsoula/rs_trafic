use std::collections::HashMap;
use crate::utils::{ObjectId};
use ndarray::{arr2, array};
pub struct Road {
    length: u32,
    n_cars: u32,
    n_lanes: u32,
    cars : HashMap<ObjectId, u32>,
    lanes : arr2<u32,u32>
}