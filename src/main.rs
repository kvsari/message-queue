#[macro_use] extern crate derive_getters;
extern crate futures;
extern crate tokio_core;

use std::time::Duration;
use std::thread;

use futures::{Future, Sink};

mod mqueue;
mod endpoint;
mod polymorphic;

use mqueue::{Target, Message, Envelope};
use polymorphic::{Entry, Record};

fn main() {
    // create mq endpoint1
    let zingle = endpoint::Zingle;
    let (e1s, e1t) = mqueue::prepare_terminus(10, zingle);
    
    // create mq endpoint2
    let bongle = endpoint::Bongle;
    let (e2s, e2t) = mqueue::prepare_terminus(10, bongle);

    // start enpoint1 & 2
    thread::Builder::new()
        .name("Zingle endpoint thread".into())
        .spawn( move || {
            let mut core = tokio_core::reactor::Core::new().unwrap();
            println!("Starting endpoint 1");
            core.run(e1t).unwrap();
        })
        .unwrap();

    thread::Builder::new()
        .name("Bongle endpoint thread".into())
        .spawn( move || {
            let mut core = tokio_core::reactor::Core::new().unwrap();
            println!("Starting endpoint 2");
            core.run(e2t).unwrap();
        })
        .unwrap();
     
    // create mq endpoint3
    let gen = endpoint::Simple::new(Record::new("Awooga".into(), 123));
    let (gs, gt) = mqueue::prepare_terminus(10, gen);
    thread::Builder::new()
        .name("Generic endpoint thread".into())
        .spawn( move || {
            let mut core = tokio_core::reactor::Core::new().unwrap();
            println!("Starting generic queue");
            core.run(gt).unwrap();
        })
        .unwrap();

    // create mq router and register endpoin1 & endpoint2
    let router = mqueue::Dispatcher::new(e1s, e2s, gs);
    let (mqs, mqr) = mqueue::prepare_terminus(21, router);

    // start mq router (mq in full)
    thread::Builder::new()
        .name("Message queue thread".into())
        .spawn( move || {
            let mut core = tokio_core::reactor::Core::new().unwrap();
            println!("Starting message queue");
            core.run(mqr).unwrap();
        })
        .unwrap();

    // create operations
    let (getrec1, chan1) = polymorphic::prepare_getrecord_op();
    let (getrec2, chan2) = polymorphic::prepare_getrecord_op();
    let insert = polymorphic::InsertRecord::new(Record::new("Yeeehaw".into(), 1302));    

    // push messages onto mq
    let message1 = Envelope::new(Target::Endpoint3,
                                 Message::Alright,
                                 Box::new(Record::new("ba".into(), 10)),
                                 Box::new(getrec1));

    let message2 = Envelope::new(Target::Endpoint3,
                                 Message::Alright,
                                 Box::new(Entry::new(true, 23.4)),
                                 Box::new(insert));

    let message3 = Envelope::new(Target::Endpoint3,
                                 Message::Alright,
                                 Box::new(Record::new("ba".into(), 10)),
                                 Box::new(getrec2));

    let mut mqueue_chan = mqs;
    
    mqueue_chan.start_send(message1).unwrap();
    mqueue_chan.start_send(message2).unwrap();
    mqueue_chan.start_send(message3).unwrap();

    /*
    mqueue_chan.start_send(message1).unwrap();
    mqueue_chan.start_send(message2).unwrap();
    mqueue_chan.start_send(message3).unwrap();
    mqueue_chan.start_send(message1).unwrap();
    mqueue_chan.start_send(message2).unwrap();
    mqueue_chan.start_send(message3).unwrap();
    mqueue_chan.start_send(message1).unwrap();
    mqueue_chan.start_send(message2).unwrap();
    mqueue_chan.start_send(message3).unwrap();
    mqueue_chan.start_send(message1).unwrap();
    mqueue_chan.start_send(message2).unwrap();
    mqueue_chan.start_send(message3).unwrap();
    mqueue_chan.start_send(message1).unwrap();
    mqueue_chan.start_send(message2).unwrap();
    mqueue_chan.start_send(message3).unwrap();
    mqueue_chan.start_send(message1).unwrap();
    mqueue_chan.start_send(message2).unwrap();
    mqueue_chan.start_send(message3).unwrap();
    */

    mqueue_chan.poll_complete().unwrap();

    thread::sleep(Duration::from_secs(1));

    println!("Record: {:?}", &chan1.wait().unwrap());
    println!("Record: {:?}", &chan2.wait().unwrap());
}
