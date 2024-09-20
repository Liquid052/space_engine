use bevy::prelude::*;


pub fn nth_frame_once<const CAP: u32>(mut counter: Local<u32>) -> bool {
    *counter += 1;
    *counter = counter.clamp(0, CAP + 1);

    *counter == CAP
}