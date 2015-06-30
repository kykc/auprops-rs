extern crate audsp;
extern crate aunorm;
use aunorm::{Normalizer, NormalizerProvider};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct PropInfo<'a, TSample: audsp::Numeric> where TSample: 'a {
    id: u32,
    min: TSample,
    max: TSample,
    default: TSample,
    value: TSample,
    norm_value: TSample,
    caption: &'a str,
    measure: &'a str,
    norm: Box<Normalizer<TSample> + 'a>,
}

#[derive(Default)]
pub struct PropStorage<'a, TSample: audsp::Numeric> where TSample: 'a {
    properties: HashMap<u32, PropInfo<'a, TSample>>
}

impl<'a, TSample: audsp::Numeric> PropInfo<'a, TSample> where TSample: 'a {
    pub fn set_from_norm(&mut self, norm: TSample) {
        self.norm_value = norm;
        self.value = self.norm.from_normal(norm);
    }

    pub fn get_value(&self) -> TSample {
        return self.value
    }

    pub fn get_norm_value(&self) -> TSample {
        return self.norm_value
    }

    pub fn get_caption(&self) -> String {
        self.caption.to_string()
    }

    pub fn get_measure(&self) -> String {
        self.measure.to_string()
    }
}

impl<'a, TSample: audsp::Numeric> PropStorage<'a, TSample> where TSample: 'a {
    pub fn new() -> Self {
        PropStorage::<TSample>{properties: HashMap::new()}
    }

    pub fn len(&self) -> usize {
        self.properties.len()
    }

    pub fn add_prop<TProv>(&mut self, index: u32, min: TSample, max: TSample, default: TSample, caption: &'a str, measure: &'a str) where TProv: NormalizerProvider<'a, TSample> {
        let norm = TProv::boxed(min, max);

        self.properties.insert(index, PropInfo::<TSample>{
            id: index,
            min: min,
            max: max,
            default: default,
            value: default,
            norm_value: norm.to_normal(default),
            caption: caption,
            measure: measure,
            norm: norm,
            });
    }

    pub fn get_value(&self, i: u32) -> TSample {
        return self.properties.get(&i).map(|x| x.value).unwrap_or(TSample::zero())
    }

    pub fn get_propinfo(&self, index: i32) -> Option<&PropInfo<TSample>> {
        let i:u32 = index as u32;
        self.properties.get(&i)
    }

    pub fn get_mut_propinfo(&mut self, index: i32) -> Option<&mut PropInfo<'a, TSample>> {
        let i:u32 = index as u32;
        self.properties.get_mut(&i)
    }
}
