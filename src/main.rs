mod lockvector;
use lockvector::LockVector;
use std::sync::Arc;
use std::thread;
use rand::Rng;
use std::time::{Instant, Duration};


fn test_pushback(num_threads: i32){

    println!("TEST PUSHBACK {} threads", num_threads);
    let size = (num_threads as usize) * 30;
    let v = Arc::new(LockVector::new(size));
    let mut threads = Vec::new();

    for i in 0..num_threads{
        let thread_v = v.clone();
        threads.push(
            thread::spawn(move || {

                for j in 0..30{
                    thread_v.pushback(((i as usize)+1)*100+(j as usize));
                }
                
            })
        );
    }

    for t in threads{
        t.join().unwrap();
    }

    for i in 0..v.size(){
        println!("{}", v.at(i).unwrap());
    }
}

fn test_popback(num_threads: i32){
    let LEN = 30;
    let size = (num_threads as usize) * LEN;
    println!("TEST POPBACK {} threads", num_threads);

    let v = Arc::new(LockVector::new(size));
    let good: Arc<LockVector<usize>> = Arc::new(LockVector::new(num_threads as usize));
    let mut threads = Vec::new();

    for i in 0..size{
        v.pushback(i as usize);
    }

    for i in 0..num_threads{
        let good_thread = good.clone();
        let v_thread  = v.clone();

        // Needed for fix where vec is uninitialized
        good.pushback(0);
        
        threads.push(
                thread::spawn( move || {
                    for j in 0..LEN{
                        // Needs thread id and first? check :50
                        let val = v_thread.popback().unwrap();
                        good_thread.insertat(val, i as usize);
                    }

                }
            )
        );
    }

    for t in threads {
        t.join().unwrap();
    }

    for i in 0..v.size(){
        println!("{}", v.at(i).unwrap());
    }

    println!("{}", good.size());
    for i in 0..num_threads {
        println!("{}", good.at(i as usize).unwrap());
    }

    println!("/n");
}


fn test_cwrite(num_threads: i32){

    let LEN = 44;
    let size = LEN * (num_threads as usize);

    let v = Arc::new(LockVector::new(size));
    let mut threads = Vec::new();
    for i in 0..LEN{
        v.pushback(0);
    }

    let mut cnt = Vec::new();
    for _ in 0..num_threads{
        let new_v = Arc::new(LockVector::new(LEN));
        for _ in 0..LEN {
            new_v.pushback(0);
        }
        cnt.push(new_v);
    }

    for i in 1..num_threads {
        let thread_v = v.clone();
        let thread_cnt = cnt[i as usize].clone();
        threads.push(
            thread::spawn( move || {

                for j in 0..1000 {
                    let pos = j % thread_v.size();
                    let prev = thread_v.at(pos).unwrap();

                    if thread_v.cwrite(pos,prev, prev+1) {

                        thread_cnt.addat(pos, 1);
                    }
                }
                }
            )
        );
    }

    for t in threads {
        t.join().unwrap();
    }

    let mut tot: Vec<usize> = vec![0;LEN];

    for i in 0..num_threads {
        for j in 0..LEN {
            let val = cnt[i as usize].at(j).unwrap();
            tot[j] += val;
            println!("{} ", val);
        }
        println!();
    }
    println!("-------------");

    for i in 0..LEN{
        println!("{}", v.at(i).unwrap());
    }

    println!();

}



fn main(){
    let num: i32 = 10;
    //test_all(num as usize);
    test_cwrite(num);
    test_pushback(num);
    test_popback(num);

}