/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */

use lazy_static::lazy_static;
use rdev::{listen, Event};
use std::sync::RwLock;
use std::thread;

use tiger_controller_ros::*;

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


fn main() {
    // Initialize node
    env_logger::init();
    rosrust::init("tiger_controls_publisher");

    rosrust::ros_info!("Starting tiger_controller");

    // Create publishers
    let drivetrain_pub = rosrust::publish("/tiger_car/control/drivetrain", 100).unwrap(); // TODO: remove unwrap
    let steering_pub = rosrust::publish("/tiger_car/control/steering", 100).unwrap(); // TODO: remove unwrap

    // spawn new thread because listen blocks
    let _listener = thread::spawn(move || {
        listen(process_event).expect("Could not listen");
    });

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        
        let mut drive_msg = rosrust_msg::std_msgs::Float64::default();
        let mut steering_msg = rosrust_msg::std_msgs::Float64::default();

        {
            // Get controller state
            let state = CONTROLLER_STATE.read().expect("Failed to unlock Mutex");

            drive_msg.data = state.get_drive();
            steering_msg.data = state.get_steering();
        }

        drivetrain_pub.send(drive_msg).unwrap(); // TODO: remove unwrap
        steering_pub.send(steering_msg).unwrap(); // TODO: remove unwrap

        // Sleep to maintain rate
        rate.sleep();
    }
}

// fn main2() {
//     // Initialize node
//     rosrust::init("talker");

//     // Create publisher
//     let chatter_pub = rosrust::publish("/", 100).unwrap();

//     let mut count = 0;

//     // Create object that maintains 10Hz between sleep requests
//     let rate = rosrust::rate(10.0);

//     // Breaks when a shutdown signal is sent
//     while rosrust::is_ok() {
//         // Create string message
//         let mut msg = rosrust_msg::std_msgs::String::default();
//         msg.data = format!("hello world {}", count);

//         // Send string message to topic via publisher
//         chatter_pub.send(msg).unwrap();

//         // Sleep to maintain 10Hz rate
//         rate.sleep();

//         count += 1;
//     }
// }
