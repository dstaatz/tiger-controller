/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors;
mod controller;


////////////////////////////////////////////////////////////////////////////////


use std::sync::RwLock;
use std::thread;

use rosrust_msg::std_msgs::Float64;
use lazy_static::lazy_static;
use rdev::{listen, Event};

use errors::*;
use controller::*;


lazy_static! {
    static ref CONTROLLER_STATE: RwLock<ControllerState> = {
        RwLock::new(ControllerState::default())
    };
}

fn process_event(event: Event) {
    CONTROLLER_STATE
        .write()
        .expect("Failed to unock Mutex")
        .process_event(event);
}


pub fn run() -> Result<()> {

    rosrust::ros_info!("Starting tiger_controller");

    // Create publishers
    let drivetrain_pub = rosrust::publish("/tiger_car/control/drivetrain", 100)?;
    let steering_pub = rosrust::publish("/tiger_car/control/steering", 100)?;

    // spawn new thread because listen blocks
    thread::spawn(move || {
        listen(process_event).expect("Could not listen");
    });

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        
        let mut drive_msg = Float64::default();
        let mut steering_msg = Float64::default();

        {
            // Get controller state
            let state = CONTROLLER_STATE.read().expect("Failed to unlock Mutex");

            drive_msg.data = state.get_drive();
            steering_msg.data = state.get_steering();
        }

        drivetrain_pub.send(drive_msg)?;
        steering_pub.send(steering_msg)?;

        // Sleep to maintain rate
        rate.sleep();
    }

    Ok(())
}

