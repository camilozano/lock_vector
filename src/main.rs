mod lockvector;
use lockvector::LockVector;
use std::sync::Arc;
use std::thread;
use rand::Rng;
use std::time::Instant;


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
    let len = 30;
    let size = (num_threads as usize) * len;
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
                    for _ in 0..len{
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

    let len = 44;
    let size = len * (num_threads as usize);

    let v = Arc::new(LockVector::new(size));
    let mut threads = Vec::new();
    for _ in 0..len{
        v.pushback(0);
    }

    let mut cnt = Vec::new();
    for _ in 0..num_threads{
        let new_v = Arc::new(LockVector::new(len));
        for _ in 0..len {
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

    let mut tot: Vec<usize> = vec![0;len];

    for i in 0..num_threads {
        for j in 0..len {
            let val = cnt[i as usize].at(j).unwrap();
            tot[j] += val;
            println!("{} ", val);
        }
        println!();
    }
    println!("-------------");

    for i in 0..len{
        println!("{}", v.at(i).unwrap());
    }

    println!();

}


fn test_all(max_num_threads: usize){

    let max_ops = 6400;
    let insert = 0;
    let erase = 1;
    let limit = 25;

    for num_threads in 1..max_num_threads {

        println!("{}",num_threads);

        for t in vec![insert, erase]{
            let v = Arc::new(LockVector::new(100));

            let each_thread = max_ops/num_threads;
            let extra = max_ops % num_threads;
            let ops_per_thread = Arc::new(LockVector::new(each_thread));

            for i in 0..num_threads {
                ops_per_thread.insertat(i, each_thread);
                if i <= extra{
                    ops_per_thread.addat(i, 1);
                }
            }

            let start_time = Instant::now();
            for i in 0..10 {
                v.pushback(i);
            }

            let mut threads = Vec::new();

            for i in 1..num_threads{
                let thread_ops_per_thread = ops_per_thread.clone();
                let thread_v = v.clone();

                if t == insert {
                    threads.push(
                        thread::spawn(
                        move || {
                            let mut rng = rand::thread_rng();
                            let mut r = || -> usize {
                                rng.gen_range(0, i)
                            };

                            //TODO: Check why None value
                            // i.e why this wasn't working
                            //let tot_ops = thread_ops_per_thread.at(i).unwrap();
                            let tot_ops = match thread_ops_per_thread.at(i) {
                                Some(x) => {x},
                                None => {1000},
                            };

                            for _ in 0..tot_ops {
                                let cur_op = r() % 3;
                                let do_pushack = (r()%100+100)%100 < limit;

                                let x = r();
                                let size = thread_v.size();
                                if do_pushack {
                                    thread_v.pushback(x);
                                } else {
                                    if cur_op == 0 && size > 0 {
                                        thread_v.insertat(r() % size, x);
                                    } else if cur_op == 1 && size > 0 {
                                        thread_v.at(r() % size);
                                    } else if cur_op == 2 && size > 0 {
                                        let pos = r() % size;
                                        let old = thread_v.at(pos).unwrap();
                                        thread_v.cwrite(pos, old, x);
                                    }
                                }
                            }
                        }
                    )
                    );
                }
                else if t == erase {
                    threads.push(
                        thread::spawn(
                            move || {
                                let mut rng = rand::thread_rng();
                                let mut r = || -> usize {
                                    rng.gen_range(0, i)
                                };
                                //let tot_ops = thread_ops_per_thread.at(i).unwrap();

                                //TODO: Check why None value
                                let tot_ops = match thread_ops_per_thread.at(i) {
                                    Some(x) => {x},
                                    None => {0},
                                };
                                for _ in 0..tot_ops {
                                    let cur_op = r() % 3;
                                    let do_pushback = (r()%100+100)%100 < limit;

                                    let x = r();
                                    let size = thread_v.size();

                                    if do_pushback {
                                        thread_v.pushback(x);
                                    } else {
                                        if cur_op == 0 && size > 0 {
                                            thread_v.erase(r()%size);
                                        } else if cur_op == 1 && size > 0 {
                                            thread_v.at(r()%size);
                                        } else if cur_op == 2 && size > 0 {
                                            let pos = r () % size;
                                            let old = thread_v.at(pos).unwrap();
                                            thread_v.cwrite(pos, old, x);
                                        }
                                    }



                                }


                            }
                        )

                    );
                }
            }

            for t in threads {
                t.join().unwrap();
            }

            let end_time = Instant::now();
            let elapsed_time = end_time.duration_since(start_time);

            println!("{:?}", elapsed_time);
        }
        println!("");
    }



}


fn main(){
    let num: i32 = 10;
    test_all(num as usize);
    test_cwrite(num);
    test_pushback(num);
    test_popback(num);

}