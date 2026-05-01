use crate::field::Field;

#[derive(Debug)]
pub struct Question {
    pub field: Field,
}

impl Question {
    pub fn new(name: String, f_type: u16, f_class: u16) -> Self {
        Question {
            field: Field::new(name, f_type, f_class),
        }
    }

    pub fn into_slice(&self, buf: &mut [u8]) -> usize {
        self.field.into_slice(buf)
    }

    pub fn read(buf: &[u8], pos: usize) -> (Question, usize) {
        let (field, pos) = Field::from_slice(buf, pos);
        let question = Question { field };
        (question, pos)
    }
}
