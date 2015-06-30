extern crate audsp;
extern crate aunorm;
use aunorm::{Normalizer, NormalizerProvider};
use std::collections::HashMap;

pub trait DspProcessor<'a, TSample: audsp::Numeric> {
    fn get_properties(& self) -> & PropStorage<'a, TSample, Self>;
    fn get_mut_properties(&mut self) -> &mut PropStorage<'a, TSample, Self>;
    fn param_changed(&mut self, i32);
    fn sample_rate_changed(&mut self, f32);
    fn get_plugin_name(&self) -> String;
    fn process_stereo_stereo(&mut self, [& [TSample]; 2], [&mut [TSample]; 2]);
}

#[allow(dead_code)]
pub struct PropInfo<'a, TSample: audsp::Numeric, TProcessor: DspProcessor<'a, TSample>> where TSample: 'a {
    id: u32,
    min: TSample,
    max: TSample,
    default: TSample,
    value: TSample,
    norm_value: TSample,
    caption: &'a str,
    measure: &'a str,
    norm: Box<Normalizer<TSample> + 'a>,
    callback : Box<Fn(&mut TProcessor) + 'a>,
}

#[derive(Default)]
pub struct PropStorage<'a, TSample: audsp::Numeric, TProcessor: DspProcessor<'a, TSample>> where TSample: 'a {
    properties: HashMap<u32, PropInfo<'a, TSample, TProcessor>>
}

impl<'a, TSample: audsp::Numeric, TProcessor: DspProcessor<'a, TSample>> PropInfo<'a, TSample, TProcessor> where TSample: 'a {
    pub fn set_from_norm(&mut self, norm: TSample, processor: &mut TProcessor) {
        self.norm_value = norm;
        self.value = self.norm.from_normal(norm);

        let callback = &self.callback;
        callback(processor);
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

impl<'a, TSample: audsp::Numeric, TProcessor> PropStorage<'a, TSample, TProcessor> where TSample: 'a, TProcessor: DspProcessor<'a, TSample> {
    pub fn new() -> Self {
        PropStorage::<TSample, TProcessor>{properties: HashMap::new()}
    }

    pub fn len(&self) -> usize {
        self.properties.len()
    }

    pub fn add_prop<TProv>(&mut self, index: u32, min: TSample, max: TSample, default: TSample, caption: &'a str, measure: &'a str, callback: Box<Fn(&mut TProcessor) + 'a>) where TProv: NormalizerProvider<'a, TSample> {
        let norm = TProv::boxed(min, max);

        self.properties.insert(index, PropInfo::<TSample, TProcessor>{
            id: index,
            min: min,
            max: max,
            default: default,
            value: default,
            norm_value: norm.to_normal(default),
            caption: caption,
            measure: measure,
            norm: norm,
            callback: callback,
            });
    }

    pub fn get_value(&self, i: u32) -> TSample {
        return self.properties.get(&i).map(|x| x.value).unwrap_or(TSample::zero())
    }

    pub fn get_propinfo(&self, index: i32) -> Option<&PropInfo<'a, TSample, TProcessor>> {
        let i:u32 = index as u32;
        self.properties.get(&i)
    }

    pub fn get_mut_propinfo(&mut self, index: i32) -> Option<&mut PropInfo<'a, TSample, TProcessor>> {
        let i:u32 = index as u32;
        self.properties.get_mut(&i)
    }
}
