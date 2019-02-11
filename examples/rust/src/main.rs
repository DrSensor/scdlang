mod statechart;
mod my_system;

use statechart::{Machine};

use my_system::*;

fn main() {
    // let mut traffic_light = Machine::<Light>::new();
    let mut lamp = Machine::<Switch>::new();
    let mut traffic_light = Machine::from(Light::Green);

    traffic_light.on_transition(|state, previous, event| {
        println!("{:?} -> {:?} @ {:?}", previous, state, event)
    });
    lamp.on_transition(|state, previous, event| {
        println!("{:?} -> {:?} @ {:?}", previous, state, event)
    });
    traffic_light.on_final(|state, event| println!("{:?} -> <final> @ {:?}", state, event));

    // loop {
    println!("\n{}. {:?},{:?}", 0, lamp.state, traffic_light.state);
    for x in 1..15 {
        if x == 13 {
            traffic_light.send(Event::Poweroff)
        } else {
            traffic_light.send(Event::Timer)
        }

        if x % 4 == 0 {
            lamp.send(Event::Push)
        }
        println!("\n{}. {:?},{:?}", x, lamp.state, traffic_light.state);
    }
    lamp.send(Event::Reset);
    // }
}
