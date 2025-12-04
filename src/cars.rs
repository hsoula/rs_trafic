pub struct Car {
    x : i32,
    lane : i32,
    speed : u32,
    variability : f64
}

impl Car {
    pub fn new(x:i32, lane:i32, speed:u32, variability: f64) -> Car {
        Car{x:x, lane:lane, speed:speed, variability:variability}
    }

}