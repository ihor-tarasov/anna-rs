use std::{collections::HashMap};

use super::{Object, Value};

pub enum Mark {
    Marked,
    Unmarked,
    Pending,
}

struct ObjectData {
    object: Object,
    mark: Mark,
}

pub struct Storage {
    objects: HashMap<usize, ObjectData>,
    counter: usize,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            counter: 0,
        }
    }

    pub fn push(&mut self, object: Object) -> Value {
        self.objects.insert(
            self.counter,
            ObjectData {
                object,
                mark: Mark::Marked,
            },
        );
        self.counter += 1;
        Value::ObjectId(self.counter - 1)
    }

    fn data(&self, id: usize) -> &ObjectData {
        self.objects.get(&id).expect("Object not exist")
    }

    fn data_mut(&mut self, id: usize) -> &mut ObjectData {
        self.objects.get_mut(&id).expect("Object not exist")
    }

    pub fn get(&self, id: usize) -> &Object {
        &self.data(id).object
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Object {
        &mut self.data_mut(id).object
    }

    pub fn mark(&mut self, id: usize) {
        let data = self.data_mut(id);
        match data.mark {
            Mark::Marked => (),
            _ => data.mark = Mark::Marked,
        }
    }

    pub fn shadow(&mut self, id: usize) {
        let data = self.data_mut(id);
        match data.mark {
            Mark::Unmarked => data.mark = Mark::Pending,
            _ => ()
        }
    }

    pub fn unmark(&mut self, id: usize) {
        self.data_mut(id).mark = Mark::Unmarked;
    }
}
