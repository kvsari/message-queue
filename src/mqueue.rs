use std::any::Any;
use std::fmt::Debug;

use futures::{Future, BoxFuture, Sink, Stream, stream};
use futures::sync::oneshot;
use futures::sync::mpsc::{self, Receiver, Sender};

use endpoint::{self, Dispenser};
use polymorphic::{Record, Entry, Operation};

/// A place where a `Message` object gets processed or is routed onto another queue.
pub trait Terminus {
    fn process(&mut self, msg: Envelope);
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Yeeehaw,
    Alright,
}

#[derive(Debug, Copy, Clone)]
pub enum Target {
    Endpoint1,
    Endpoint2,
    Endpoint3,
}

//#[derive(Debug)]
pub struct Envelope {
    target: Target,
    message: Message,
    data: Box<Any + Send>,
    operation: Box<Operation + Send>,
}

impl Envelope {
    pub fn new(t: Target, m: Message, d: Box<Any + Send>, o: Box<Operation + Send>)
               -> Envelope {
        Envelope {
            target: t,
            message: m,
            data: d,
            operation: o,
        }
    }

    pub fn open(self) -> (Target, Message, Box<Any + Send>, Box<Operation + Send>) {
        (self.target, self.message, self.data, self.operation)
    }
}

//#[derive(Debug)]
pub struct Dispatcher {
    endp1: Sender<Envelope>,
    endp2: Sender<Envelope>,
    endp3: Sender<Envelope>,
}

impl Dispatcher {
    pub fn new(e1: Sender<Envelope>, e2: Sender<Envelope>, e3: Sender<Envelope>)
               -> Dispatcher {
        Dispatcher {
            endp1: e1,
            endp2: e2,
            endp3: e3,
        }
    }

    pub fn dispatch(&mut self, msg: Envelope) {
        match msg.target {
            Target::Endpoint1 => {
                self.endp1.start_send(msg).unwrap();
                self.endp1.poll_complete().unwrap();
            },
            Target::Endpoint2 => {
                self.endp2.start_send(msg).unwrap();
                self.endp2.poll_complete().unwrap();                
            },
            Target::Endpoint3 => {
                self.endp3.start_send(msg).unwrap();
                self.endp3.poll_complete().unwrap();                
            },
        }
    }
}

impl Terminus for Dispatcher {
    fn process(&mut self, msg: Envelope) {
        self.dispatch(msg);
    }
}

impl Terminus for endpoint::Zingle {
    fn process(&mut self, env: Envelope) {
        let (_, _, data, _) = env.open();

        if let Some(rec) = data.downcast_ref::<Record>() {
            self.zingle(rec);
        } else {
            panic!("Oh dear!");
        }
    }
}

impl Terminus for endpoint::Bongle {    
    fn process(&mut self, env: Envelope) {
        let (_, _, data, _) = env.open();

        if let Some(ent) = data.downcast_ref::<Entry>() {
            self.bongle();
            ent.tellit();
        } else {
            panic!("Oh dear!");
        }       
    }
}

/*
impl<T: Debug + Copy + Send + 'static> Terminus for endpoint::Generic<T> {
    fn process(&mut self, env: Envelope) {
        let (_, _, data, mut operation) = env.open();
        operation.execute(self);
    }
}
*/

impl<T: Debug + Send + 'static> Terminus for endpoint::Simple<T> {
    fn process(&mut self, env: Envelope) {
        let (_, _, data, mut operation) = env.open();
        operation.execute(self);
    }
}

pub fn prepare_terminus<T: Terminus + Send + 'static>(buffer: usize, mut terminus: T)
                                                      -> (Sender<Envelope>, BoxFuture<(), ()>) {
    let (tx, rx) = mpsc::channel(buffer);

    let terminal = rx.for_each( move |msg| {
        terminus.process(msg);
        Ok(())
    }).boxed();

    (tx, terminal)
}

/*
pub fn prepare_dispenser<D>(buf: usize, mut disp: D)
                            -> (Sender<Envelope>, BoxFuture<(), ()>)
    where D: Dispenser + Send + 'static
{
    let (tx, rx) = mpsc::channel(buf);

    let terminal = rx.for_each( move |msg| {
        terminus.process(msg);
        Ok(())
    }).boxed();

    (tx, terminal)
}
*/
