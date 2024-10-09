use crate::SQLTypes;


#[derive(Debug)]
pub struct UpdateProps {
    pub connect: SQLTypes,
    pub columns: Vec<String>,
    pub table: String,
    pub clause: Option<String>,
}