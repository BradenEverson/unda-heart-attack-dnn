pub struct HeartModel {
    age: f32,
    sex: f32,
    max_hr: f32,
    old_peak: f32,

    res: f32
}

impl HeartModel {
    pub fn new(age: f32, sex: f32, max_hr: f32, old_peak: f32, res: f32) -> Self {
        Self { age, sex, max_hr, old_peak, res }
    }
    pub fn to_inp_out(&self) -> (Vec<f32>,Vec<f32>) {
        (
            vec![
            self.age,
            self.sex,
            self.max_hr,
            self.old_peak
            ], 
            vec![self.res]
        )
    }
}
