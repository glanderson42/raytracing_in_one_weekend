use std::vec::Vec;
use std::boxed::Box;

use crate::hitable::Hitable;

pub struct HitTableList {
    pub objects: Vec<Box<dyn Hitable>>
}

impl HitTableList {
    pub fn new() -> HitTableList {
        HitTableList { objects: Vec::<Box<dyn Hitable>>::new() }
    }

    pub fn new_object(&mut self, object: Box<dyn Hitable>) {
        self.objects.push(object);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

