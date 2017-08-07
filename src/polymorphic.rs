use std::mem;
use std::any::Any;

use futures::sync::oneshot::{self, Sender, Receiver};

use endpoint::{Simple, Dispenser};

#[derive(Debug, Clone, Getters)]
pub struct Record {
    text: String,
    num: u64,
}

impl Record {
    pub fn new(t: String, n: u64) -> Record {
        Record {
            text: t,
            num: n,
        }
    }
}

/*
impl Any for Record {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Record>()
    }
}
 */

#[derive(Debug, Copy, Clone, Getters)]
pub struct Entry {
    truth: bool,
    floater: f64,
}

impl Entry {
    pub fn new(t: bool, f: f64) -> Entry {
        Entry {
            truth: t,
            floater: f,
        }
    }

    pub fn tellit(&self) {
        if self.truth {
            println!("It's the truth! {}", &self.floater);
        } else {
            println!("You lie");
        }
    }
}

pub trait Operation {
    fn execute(&mut self, dispenser: &mut Any);
}

#[derive(Debug, Clone, Getters)]
pub struct InsertRecord {
    rec: Record,
}

impl InsertRecord {
    pub fn new(rec: Record) -> InsertRecord {
        InsertRecord {
            rec: rec,
        }
    }
}

impl Operation for InsertRecord {
    fn execute(&mut self, dispenser: &mut Any) {
        if let Some(mut dsp) = dispenser.downcast_mut::<Simple<Record>>() {
            dsp.store(self.rec.clone());
        } else {
            panic!("Wrong dispenser");
        }
    }
}

pub fn prepare_getrecord_op() -> (GetRecord, Receiver<Record>) {
    let (tx, rx) = oneshot::channel();
    (GetRecord::new(tx), rx)
}

pub struct GetRecord {
    sender: Vec<Sender<Record>>,
}

impl GetRecord {
    fn new(tx: Sender<Record>) -> GetRecord {
        GetRecord {
            sender: vec![tx],
        }
    }
}

impl Operation for GetRecord {
    fn execute(&mut self, dispenser: &mut Any) {
        if let Some(mut dsp) = dispenser.downcast_mut::<Simple<Record>>() {
            let rec = dsp.dispense();
            let mut sender = mem::replace(&mut self.sender, Vec::with_capacity(0));
            let sender = sender.pop().unwrap();
            sender.send(rec).unwrap();
        } else {
            panic!("Wrong dispenser");
        }
    }
}
