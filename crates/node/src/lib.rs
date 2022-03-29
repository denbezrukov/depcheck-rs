#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}


pub struct MyStruct {
    pub field: i32,
    pub field1: f64,
}

#[napi]
impl MyStruct {
    pub fn with_field(field: i32) -> Self {
        MyStruct {
            field,
            field1: 0.,
        }
    }

    pub fn get_field(&self) -> i32 {
        self.field
    }
}
