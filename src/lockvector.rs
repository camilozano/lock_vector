use std::sync::{Arc, Mutex};
use std::thread;
//use std::thread;
#[derive(Debug)]
pub struct LockVector<T: Copy> {
    pub list: Mutex<Vec<T>>,
    size: Mutex<usize>
}

impl <T> LockVector<T> 
where 
    T: Copy,
{
    pub fn new(size: usize) -> Self {
        LockVector {
            list: Mutex::new(Vec::with_capacity(size)),
            size: Mutex::new(size),
        }
    }
    pub fn at(&self, index: usize) -> Option<T>{
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

    pub fn pushback(&self, value: T){
        let list = &mut self.list.lock().unwrap();
        list.push(value);
    }
    
    pub fn popback(&self) -> Option<T> {
        let list = &mut self.list.lock().unwrap();
        list.pop()
    }

    pub fn erase(&self, index: usize) -> Option<T> {
        let list = &mut self.list.lock().unwrap();
        if index < list.len() {
            return Some(list.remove(index))
        }
        None
    }

    pub fn size(&self) -> usize {
        let list = &mut self.list.lock().unwrap();
        list.len()
    }



}
