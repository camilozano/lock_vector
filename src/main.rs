use std::sync::{Arc, Mutex};
use std::thread;
//use std::thread;
#[derive(Debug)]
struct LockVector<T: Copy> {
    list: Mutex<Vec<T>>,
}

impl <T> LockVector<T> 
where 
    T: Copy,
{
    fn new(size: usize) -> Self {
        LockVector {
            list: Mutex::new(Vec::with_capacity(size)),
        }
    }
    fn at(&self, index: usize) -> Option<T>{
        let val = match self.list.lock() {
            Ok(list) => {
                if index < list.len(){
                    return Some(list[index]);
                }
                None
            },
            Err(_) => {None},
        };
        val
    }

    fn pushback(&self, value: T){
        let list = &mut self.list.lock().unwrap();
        list.push(value);
    }
    fn popback(&self) -> Option<T> {
        let list = &mut self.list.lock().unwrap();
        list.pop()
    }
}

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
                match t2_list.pop() {
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
