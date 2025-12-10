
use crate::utils::{ObjectId, generate_id};

pub struct Car {
    id: ObjectId,
    x : i32,
    lane : i32,
    speed : u32,
    variability : f64
}

impl Car {
    pub fn new(x:i32, lane:i32, speed:u32, variability: f64) -> Car {
        let id = generate_id();
        Car{id: id, x:x, lane:lane, speed:speed, variability:variability}
    }

    pub fn id(&self) -> ObjectId {
        self.id
    }
    pub fn get_x(&self) -> i32 { self.x }
    pub fn get_lane(&self) -> i32 { self.lane }
    pub fn get_speed(&self) -> u32 { self.speed }
    pub fn get_variability(&self) -> f64 { self.variability }



}