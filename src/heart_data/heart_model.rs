use std::error::Error;

use unda::{core::data::input::Input, util::csv_parser::CSVParse};

pub struct HeartModel {
    age: f32,
    sex: f32,
    max_hr: f32,
    old_peak: f32,

    res: f32
}

impl Input for HeartModel {
    ///Flattens input data into a one dimensional context
    ///Most commonly used by dense layers and output layers
    fn to_param(&self) -> Vec<f32> {
        self.to_inp_out().0
    }
    ///Flattens(or extends) data into a two dimensional context
    ///Flattens if data is a higher order, extends if its a lower order
    fn to_param_2d(&self) -> Vec<Vec<f32>> {
        vec![self.to_param()]
    }
    ///Flattens(or extends) data into a two dimensional context
    ///Flattens if data is a higher order, extends if its a lower order
    fn to_param_3d(&self) -> Vec<Vec<Vec<f32>>> {
        vec![vec![self.to_param()]]
    }
    ///Returns the underlying shape the data has when it is 
    /// not being morphed or shaped by the param methods
    fn shape(&self) -> (usize, usize, usize) {
        (4, 1, 1)
    }
    ///Wrapper method for boxing an input up
    fn to_box(&self) -> Box<dyn Input> {
        Box::new(self.to_param())
    }
}

impl HeartModel {
    pub fn new(age: f32,
        sex: f32,
        max_hr: f32,
        old_peak: f32,
        res: f32) -> Self {
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
    pub fn from_file(path: &str) -> Result<(Vec<Vec<f32>>, Vec<Vec<f32>>), Box<dyn Error>> {
        let mut float_parsed: Vec<Vec<f32>> = vec![];
        let mut res_in = vec![];
        let mut res_out = vec![];
        float_parsed.parse_elem(path)?;

        for row in &float_parsed {
            let age = row[0] / 100f32;
            let rest_ecg = row[7] / 200f32;
            let old_peak = row[9] / 5f32;
            let heart_model = HeartModel::new(age, row[1], rest_ecg, old_peak, row[row.len()-1]);

            res_in.push(heart_model.to_inp_out().0);
            res_out.push(heart_model.to_inp_out().1);
        }
        
        Ok((res_in, res_out))
    }
}
