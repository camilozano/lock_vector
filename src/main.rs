use std::sync::{Arc, Mutex};
use std::thread;

use lockvector::LockVector;

mod lockvector;

fn main() {
    let list = Arc::new(LockVector::new(5));

    let mut handles = Vec::new();

    let t_list = list.clone();
    handles.push(
        thread::spawn(
            move || {
                t_list.pushback(0);
            }
        )
    );

    let t2_list = list.clone();
    handles.push(
        thread::spawn(
            move || {
                match t2_list.popback
                () {
                    Some(val) => {println!("Removed {}", val)},
                    None => {println!("Failed to remove")},
                };
            }
        )
    );


    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}",list.list);

}
