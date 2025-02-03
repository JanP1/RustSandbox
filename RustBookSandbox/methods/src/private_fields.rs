#![allow(dead_code)]

pub struct EncStruct {
    field1_gr0: isize,
}


impl EncStruct {
    pub fn encstruct_constructor(a:isize) -> Self {
        if a <= 0 {
            Self{
                field1_gr0: 1,
            }
        }
        else {
            Self{
                field1_gr0: a,
            }
        }
    }

    pub fn get_field1(&self) -> &isize {
        &self.field1_gr0
    }
}
