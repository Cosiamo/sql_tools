use crate::{data_types::SQLDataTypes, Error};

use super::{Ordered, SelectBuilder, SelectProps};

#[derive(Debug)]
pub struct Grouped { pub select: SelectProps }

impl Grouped {
    pub fn order_asc(self, column: &str) -> Ordered {
        self.select.order_asc(column)
    }
    
    pub fn order_desc(self, column: &str) -> Ordered {
        self.select.order_desc(column)
    }

    pub fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> { 
        self.select.build()
    }
    
    pub fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        self.select.build_single_thread()
    }
}