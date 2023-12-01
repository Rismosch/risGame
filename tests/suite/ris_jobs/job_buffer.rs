use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use ris_jobs::job::Job;
use ris_jobs::job_buffer::JobBuffer;
use ris_util::testing::duplicate;
use ris_util::testing::retry;

//-----------------------------//
//                             //
//    single threaded tests    //
//                             //
//-----------------------------//

#[test]
fn should_push_and_pop() {
    let job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data = data.clone();
    let job = Job::new(move || *moved_data.borrow_mut() = 42);

    let push = unsafe { job_buffer.push(job) };
    assert!(push.is_ok());

    let job = unsafe { job_buffer.wait_and_pop() };
    assert!(job.is_ok());
    job.unwrap().invoke();
    assert_eq!(*data.borrow(), 42);
}

#[test]
fn should_push_and_steal() {
    let job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data = data.clone();
    let job = Job::new(move || *moved_data.borrow_mut() = 42);

    let push = unsafe { job_buffer.push(job) };
    assert!(push.is_ok());

    let job = job_buffer.steal();
    assert!(job.is_ok());
    job.unwrap().invoke();
    assert_eq!(*data.borrow(), 42);
}

#[test]
fn should_push_till_full() {
    let job_buffer = JobBuffer::new(2);

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();
    let moved_data3 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);
    let job3 = Job::new(move || *moved_data3.borrow_mut() = 3);

    let push1 = unsafe { job_buffer.push(job1) };
    let push2 = unsafe { job_buffer.push(job2) };
    let push3 = unsafe { job_buffer.push(job3) };

    assert!(push1.is_ok());
    assert!(push2.is_ok());
    assert!(push3.is_err());

    push3.err().unwrap().not_pushed.invoke();
    assert_eq!(*data.borrow(), 3);
}

#[test]
fn should_pop_till_empty() {
    let job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);

    let _ = unsafe { job_buffer.push(job1) };
    let _ = unsafe { job_buffer.push(job2) };

    let pop1 = unsafe { job_buffer.wait_and_pop() };
    let pop2 = unsafe { job_buffer.wait_and_pop() };
    let pop3 = unsafe { job_buffer.wait_and_pop() };

    assert!(pop1.is_ok());
    assert!(pop2.is_ok());
    assert!(pop3.is_err());

    pop1.ok().unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
    pop2.ok().unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
}

#[test]
fn should_steal_till_empty() {
    let job_buffer = JobBuffer::new(4);

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);

    let _ = unsafe { job_buffer.push(job1) };
    let _ = unsafe { job_buffer.push(job2) };

    let steal1 = job_buffer.steal();
    let steal2 = job_buffer.steal();
    let steal3 = job_buffer.steal();

    assert!(steal1.is_ok());
    assert!(steal2.is_ok());
    assert!(steal3.is_err());

    steal1.ok().unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
    steal2.ok().unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
}

#[test]
fn should_push_pop_and_steal_multiple_times() {
    let job_buffer = JobBuffer::new(5);

    for _ in 0..5 {
        let data = Rc::new(RefCell::new(0));
        let moved_data1 = data.clone();
        let moved_data2 = data.clone();
        let moved_data3 = data.clone();
        let moved_data4 = data.clone();
        let moved_data5 = data.clone();
        let moved_data6 = data.clone();

        let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
        let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);
        let job3 = Job::new(move || *moved_data3.borrow_mut() = 3);
        let job4 = Job::new(move || *moved_data4.borrow_mut() = 4);
        let job5 = Job::new(move || *moved_data5.borrow_mut() = 5);
        let job6 = Job::new(move || *moved_data6.borrow_mut() = 6);

        let push1 = unsafe { job_buffer.push(job1) };
        let push2 = unsafe { job_buffer.push(job2) };
        let push3 = unsafe { job_buffer.push(job3) };
        let push4 = unsafe { job_buffer.push(job4) };
        let push5 = unsafe { job_buffer.push(job5) };
        let push6 = unsafe { job_buffer.push(job6) };

        assert!(push1.is_ok());
        assert!(push2.is_ok());
        assert!(push3.is_ok());
        assert!(push4.is_ok());
        assert!(push5.is_ok());
        assert!(push6.is_err());

        push6.err().unwrap().not_pushed.invoke();
        assert_eq!(*data.borrow(), 6);

        let steal1 = job_buffer.steal();
        let pop2 = unsafe { job_buffer.wait_and_pop() };
        let steal3 = job_buffer.steal();
        let pop4 = unsafe { job_buffer.wait_and_pop() };
        let steal5 = job_buffer.steal();
        let pop6 = unsafe { job_buffer.wait_and_pop() };
        let steal7 = job_buffer.steal();

        assert!(steal1.is_ok());
        assert!(pop2.is_ok());
        assert!(steal3.is_ok());
        assert!(pop4.is_ok());
        assert!(steal5.is_ok());
        assert!(pop6.is_err());
        assert!(steal7.is_err());

        steal1.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 1);
        pop2.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 5);
        steal3.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 2);
        pop4.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 4);
        steal5.ok().unwrap().invoke();
        assert_eq!(*data.borrow(), 3);
    }
}

#[test]
fn should_push_to_original_and_pop_from_duplicate() {
    let original_buffer = JobBuffer::new(4);
    let duplicated_buffer = &(*original_buffer);

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);

    let push1 = unsafe { original_buffer.push(job1) };
    let push2 = unsafe { original_buffer.push(job2) };

    assert!(push1.is_ok());
    assert!(push2.is_ok());

    let pop1 = unsafe { duplicated_buffer.wait_and_pop() };
    let steal2 = duplicated_buffer.steal();

    assert!(pop1.is_ok());
    assert!(steal2.is_ok());

    pop1.unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
    steal2.unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
}

#[test]
fn should_push_to_duplicate_and_pop_from_original() {
    let original_buffer = JobBuffer::new(4);
    let duplicated_buffer = duplicate(&original_buffer);

    let data = Rc::new(RefCell::new(0));
    let moved_data1 = data.clone();
    let moved_data2 = data.clone();

    let job1 = Job::new(move || *moved_data1.borrow_mut() = 1);
    let job2 = Job::new(move || *moved_data2.borrow_mut() = 2);

    let push1 = unsafe { duplicated_buffer.push(job1) };
    let push2 = unsafe { duplicated_buffer.push(job2) };

    assert!(push1.is_ok());
    assert!(push2.is_ok());

    let pop1 = original_buffer.steal();
    let steal2 = unsafe { original_buffer.wait_and_pop() };

    assert!(pop1.is_ok());
    assert!(steal2.is_ok());

    pop1.unwrap().invoke();
    assert_eq!(*data.borrow(), 1);
    steal2.unwrap().invoke();
    assert_eq!(*data.borrow(), 2);
}

//----------------------------//
//                            //
//    multi threaded tests    //
//                            //
//----------------------------//

#[cfg(not(miri))]
const LOOP_ITERATIONS_1: usize = 1_000;

#[cfg(miri)]
const LOOP_ITERATIONS_1: usize = 100;

#[test]
fn should_steal_from_empty_buffer_from_multiple_threads() {
    let buffer = JobBuffer::new(LOOP_ITERATIONS_1);
    let mut handles = Vec::new();
    let results = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..LOOP_ITERATIONS_1 {
        let copied_buffer = buffer.clone();
        let copied_results = results.clone();
        let handle = thread::spawn(move || {
            let result = copied_buffer.steal();
            copied_results.lock().unwrap().push(result.is_err());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    for i in 0..LOOP_ITERATIONS_1 {
        assert!(results[i], "{:?}", results);
    }

    let mut unsuccessful_steals = 0;
    while unsafe { buffer.wait_and_pop() }.is_ok() {
        unsuccessful_steals += 1;
    }

    assert_eq!(unsuccessful_steals, 0);
}

#[cfg(not(miri))]
const LOOP_ITERATIONS_2: usize = 1_000;

#[cfg(miri)]
const LOOP_ITERATIONS_2: usize = 10;

#[cfg(not(miri))]
const EXPECTED_STEALS: usize = 950;

#[cfg(miri)]
const EXPECTED_STEALS: usize = 9;

#[test]
fn should_steal_from_full_buffer_from_multiple_threads() {
    retry(10, || {
        let buffer = JobBuffer::new(LOOP_ITERATIONS_2);
        let mut handles = Vec::new();
        let results = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..LOOP_ITERATIONS_2 {
            let job = Job::new(|| {});
            unsafe { buffer.push(job) }.unwrap();
        }

        for _ in 0..LOOP_ITERATIONS_2 {
            let copied_buffer = buffer.clone();
            let copied_results = results.clone();
            let handle = thread::spawn(move || {
                let result = copied_buffer.steal();
                copied_results.lock().unwrap().push(result.is_ok());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let results = results.lock().unwrap();
        let mut successful_steals = 0;
        for i in 0..LOOP_ITERATIONS_2 {
            assert_eq!(results.len(), LOOP_ITERATIONS_2);
            if results[i] {
                successful_steals += 1;
            }
        }

        let mut unsuccessful_steals = 0;
        while unsafe { buffer.wait_and_pop() }.is_ok() {
            unsuccessful_steals += 1;
        }

        assert!(
            successful_steals > EXPECTED_STEALS,
            "successful_steals {}",
            successful_steals
        );
        assert_eq!(successful_steals + unsuccessful_steals, LOOP_ITERATIONS_2);
    });
}

#[test]
fn should_steal_from_partially_filled_buffer_from_multiple_threads() {
    let buffer = JobBuffer::new(LOOP_ITERATIONS_1);
    let mut handles = Vec::new();
    let results = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..50 {
        let job = Job::new(|| {});
        unsafe { buffer.push(job) }.unwrap();
    }

    for _ in 0..LOOP_ITERATIONS_1 {
        let copied_buffer = buffer.clone();
        let copied_results = results.clone();
        let handle = thread::spawn(move || {
            let result = copied_buffer.steal();
            copied_results.lock().unwrap().push(result.is_ok());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    let mut successful_steals = 0;
    for i in 0..LOOP_ITERATIONS_1 {
        assert_eq!(results.len(), LOOP_ITERATIONS_1);
        if results[i] {
            successful_steals += 1;
        }
    }

    let mut unsuccessful_steals = 0;
    while unsafe { buffer.wait_and_pop() }.is_ok() {
        unsuccessful_steals += 1;
    }

    assert_eq!(successful_steals, 50);
    assert_eq!(unsuccessful_steals, 0);
}

#[test]
fn should_push_from_one_thread_while_one_is_stealing_on_empty_buffer() {
    retry(10, || {
        let buffer = JobBuffer::new(LOOP_ITERATIONS_2);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));

        let push_buffer = buffer.clone();
        let push_results_copy = push_results.clone();
        let push_handle = thread::spawn(move || {
            for _ in 0..LOOP_ITERATIONS_2 {
                let result = unsafe { push_buffer.push(Job::new(|| {})) };
                push_results_copy.lock().unwrap().push(result.is_ok());
            }
        });

        let steal_buffer = duplicate(&buffer);
        let steal_results_copy = steal_results.clone();
        let steal_handle = thread::spawn(move || {
            for _ in 0..LOOP_ITERATIONS_2 {
                let result = steal_buffer.steal();
                steal_results_copy.lock().unwrap().push(result.is_ok());
            }
        });

        push_handle.join().unwrap();
        steal_handle.join().unwrap();

        let push_results = push_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();

        assert_eq!(push_results.len(), LOOP_ITERATIONS_2);
        assert_eq!(steal_results.len(), LOOP_ITERATIONS_2);

        let mut successful_pushes = 0;
        let mut successful_steals = 0;
        for i in 0..LOOP_ITERATIONS_2 {
            if push_results[i] {
                successful_pushes += 1;
            }

            if steal_results[i] {
                successful_steals += 1;
            }
        }

        assert!(
            successful_pushes > EXPECTED_STEALS,
            "successful_pushes: {}",
            successful_pushes
        );
        assert!(
            successful_steals > EXPECTED_STEALS,
            "successful_steals: {}",
            successful_steals
        );
    });
}

#[test]
fn should_push_from_one_thread_while_one_is_stealing_on_full_buffer() {
    retry(10, || {
        let buffer = JobBuffer::new(LOOP_ITERATIONS_2);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..LOOP_ITERATIONS_2 {
            unsafe { buffer.push(Job::new(|| {})) }.unwrap();
        }

        let push_buffer = buffer.clone();
        let push_results_copy = push_results.clone();
        let push_handle = thread::spawn(move || {
            for _ in 0..LOOP_ITERATIONS_2 {
                let result = unsafe { push_buffer.push(Job::new(|| {})) };
                push_results_copy.lock().unwrap().push(result.is_ok());
            }
        });

        let steal_buffer = duplicate(&buffer);
        let steal_results_copy = steal_results.clone();
        let steal_handle = thread::spawn(move || {
            for _ in 0..LOOP_ITERATIONS_2 {
                let result = steal_buffer.steal();
                steal_results_copy.lock().unwrap().push(result.is_ok());
            }
        });

        push_handle.join().unwrap();
        steal_handle.join().unwrap();

        let push_results = push_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();

        assert_eq!(push_results.len(), LOOP_ITERATIONS_2);
        assert_eq!(steal_results.len(), LOOP_ITERATIONS_2);

        let mut _successful_pushes = 0;
        let mut successful_steals = 0;
        for i in 0..LOOP_ITERATIONS_2 {
            if push_results[i] {
                _successful_pushes += 1;
            }

            if steal_results[i] {
                successful_steals += 1;
            }
        }

        // pushes are super unreliable. successful_pushes can be everything between
        // 0 or 1000. in this edgecase, i am just happy that nothing panics
        assert!(
            successful_steals > EXPECTED_STEALS,
            "successful_steals: {}",
            successful_steals
        );
    });
}

#[test]
fn should_push_from_one_thread_while_multiple_are_stealing_on_empty_buffer() {
    retry(10, || {
        let buffer = JobBuffer::new(LOOP_ITERATIONS_2);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));

        let push_buffer = buffer.clone();
        let push_results_copy = push_results.clone();
        let push_handle = thread::spawn(move || {
            for _ in 0..LOOP_ITERATIONS_2 {
                let result = unsafe { push_buffer.push(Job::new(|| {})) };
                push_results_copy.lock().unwrap().push(result.is_ok());
            }
        });

        let mut steal_handles = Vec::new();
        for _ in 0..100 {
            let steal_buffer = buffer.clone();
            let steal_results_copy = steal_results.clone();
            let steal_handle = thread::spawn(move || {
                for _ in 0..10 {
                    let result = steal_buffer.steal();
                    steal_results_copy.lock().unwrap().push(result.is_ok());
                }
            });
            steal_handles.push(steal_handle);
        }

        push_handle.join().unwrap();
        for handle in steal_handles {
            handle.join().unwrap();
        }

        let push_results = push_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();

        assert_eq!(push_results.len(), LOOP_ITERATIONS_2);
        assert_eq!(steal_results.len(), LOOP_ITERATIONS_2);

        let mut successful_pushes = 0;
        let mut successful_steals = 0;
        for i in 0..LOOP_ITERATIONS_2 {
            if push_results[i] {
                successful_pushes += 1;
            }

            if steal_results[i] {
                successful_steals += 1;
            }
        }

        assert!(
            successful_pushes > EXPECTED_STEALS,
            "successful_pushes: {}",
            successful_pushes
        );
        assert!(
            successful_steals > EXPECTED_STEALS,
            "successful_steals: {}",
            successful_steals
        );
    });
}

#[test]
fn should_push_from_one_thread_while_multiple_are_stealing_on_full_buffer() {
    retry(10, || {
        let buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..1000 {
            unsafe { buffer.push(Job::new(|| {})) }.unwrap();
        }

        let push_buffer = buffer.clone();
        let push_results_copy = push_results.clone();
        let push_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = unsafe { push_buffer.push(Job::new(|| {})) };
                push_results_copy.lock().unwrap().push(result.is_ok());
            }
        });

        let mut steal_handles = Vec::new();
        for _ in 0..100 {
            let steal_buffer = buffer.clone();
            let steal_results_copy = steal_results.clone();
            let steal_handle = thread::spawn(move || {
                for _ in 0..10 {
                    let result = steal_buffer.steal();
                    steal_results_copy.lock().unwrap().push(result.is_ok());
                }
            });
            steal_handles.push(steal_handle);
        }

        push_handle.join().unwrap();
        for handle in steal_handles {
            handle.join().unwrap();
        }

        let push_results = push_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();

        assert_eq!(push_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);

        let mut _successful_pushes = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                _successful_pushes += 1;
            }

            if steal_results[i] {
                successful_steals += 1;
            }
        }

        // pushes are super unreliable. successful_pushes can be everything between
        // 0 or 1000. in this edgecase, i am just happy that nothing panics
        assert!(
            successful_steals > 950,
            "successful_steals: {}",
            successful_steals
        );
    });
}

#[test]
fn should_pop_from_one_thread_while_one_is_stealing_on_empty_buffer() {
    let buffer = JobBuffer::new(1000);
    let pop_results = Arc::new(Mutex::new(Vec::new()));
    let steal_results = Arc::new(Mutex::new(Vec::new()));

    let pop_buffer = buffer.clone();
    let pop_results_copy = pop_results.clone();
    let pop_handle = thread::spawn(move || {
        for _ in 0..1000 {
            let result = unsafe { pop_buffer.wait_and_pop() };
            pop_results_copy.lock().unwrap().push(result.is_err());
        }
    });

    let steal_buffer = duplicate(&buffer);
    let steal_results_copy = steal_results.clone();
    let steal_handle = thread::spawn(move || {
        for _ in 0..1000 {
            let result = steal_buffer.steal();
            steal_results_copy.lock().unwrap().push(result.is_err());
        }
    });

    pop_handle.join().unwrap();
    steal_handle.join().unwrap();

    let pop_results = pop_results.lock().unwrap();
    let steal_results = steal_results.lock().unwrap();

    assert_eq!(pop_results.len(), 1000);
    assert_eq!(steal_results.len(), 1000);

    for i in 0..1000 {
        assert!(pop_results[i]);
        assert!(steal_results[i]);
    }
}

#[test]
fn should_pop_from_one_thread_while_one_is_stealing_on_full_buffer() {
    let buffer = JobBuffer::new(1000);
    let pop_results = Arc::new(Mutex::new(Vec::new()));
    let steal_results = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..1000 {
        unsafe { buffer.push(Job::new(|| {})) }.unwrap();
    }

    let pop_buffer = buffer.clone();
    let pop_results_copy = pop_results.clone();
    let pop_handle = thread::spawn(move || {
        for _ in 0..1000 {
            let result = unsafe { pop_buffer.wait_and_pop() };
            pop_results_copy.lock().unwrap().push(result.is_err());
        }
    });

    let steal_buffer = duplicate(&buffer);
    let steal_results_copy = steal_results.clone();
    let steal_handle = thread::spawn(move || {
        for _ in 0..1000 {
            let result = steal_buffer.steal();
            steal_results_copy.lock().unwrap().push(result.is_err());
        }
    });

    pop_handle.join().unwrap();
    steal_handle.join().unwrap();

    let pop_results = pop_results.lock().unwrap();
    let steal_results = steal_results.lock().unwrap();

    assert_eq!(pop_results.len(), 1000);
    assert_eq!(steal_results.len(), 1000);

    let mut successful_pops = 0;
    let mut successful_steals = 0;
    for i in 0..1000 {
        if pop_results[i] {
            successful_pops += 1;
        }
        if steal_results[i] {
            successful_steals += 1;
        }
    }

    assert_eq!(
        successful_pops + successful_steals,
        1000,
        "successful_pops: {}, successful_steals: {}",
        successful_pops,
        successful_steals
    );
}

#[test]
fn should_pop_from_one_thread_while_multiple_are_stealing_on_empty_buffer() {
    let buffer = JobBuffer::new(1000);
    let pop_results = Arc::new(Mutex::new(Vec::new()));
    let steal_results = Arc::new(Mutex::new(Vec::new()));

    let pop_buffer = buffer.clone();
    let pop_results_copy = pop_results.clone();
    let pop_handle = thread::spawn(move || {
        for _ in 0..1000 {
            let result = unsafe { pop_buffer.wait_and_pop() };
            pop_results_copy.lock().unwrap().push(result.is_err());
        }
    });

    let mut steal_handles = Vec::new();
    for _ in 0..10 {
        let steal_buffer = buffer.clone();
        let steal_results_copy = steal_results.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let result = steal_buffer.steal();
                steal_results_copy.lock().unwrap().push(result.is_err());
            }
        });
        steal_handles.push(handle);
    }

    pop_handle.join().unwrap();

    for handle in steal_handles {
        handle.join().unwrap();
    }

    let pop_results = pop_results.lock().unwrap();
    let steal_results = steal_results.lock().unwrap();

    assert_eq!(pop_results.len(), 1000);
    assert_eq!(steal_results.len(), 1000);

    for i in 0..1000 {
        assert!(pop_results[i]);
        assert!(steal_results[i]);
    }
}

#[test]
fn should_pop_from_one_thread_while_multiple_are_stealing_on_full_buffer() {
    let buffer = JobBuffer::new(1000);
    let pop_results = Arc::new(Mutex::new(Vec::new()));
    let steal_results = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..1000 {
        unsafe { buffer.push(Job::new(|| {})) }.unwrap();
    }

    let pop_buffer = buffer.clone();
    let pop_results_copy = pop_results.clone();
    let pop_handle = thread::spawn(move || {
        for _ in 0..1000 {
            let result = unsafe { pop_buffer.wait_and_pop() };
            pop_results_copy.lock().unwrap().push(result.is_err());
        }
    });

    let mut steal_handles = Vec::new();
    for _ in 0..10 {
        let steal_buffer = buffer.clone();
        let steal_results_copy = steal_results.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let result = steal_buffer.steal();
                steal_results_copy.lock().unwrap().push(result.is_err());
            }
        });
        steal_handles.push(handle);
    }

    pop_handle.join().unwrap();

    for handle in steal_handles {
        handle.join().unwrap();
    }

    let pop_results = pop_results.lock().unwrap();
    let steal_results = steal_results.lock().unwrap();

    assert_eq!(pop_results.len(), 1000);
    assert_eq!(steal_results.len(), 1000);

    let mut successful_pops = 0;
    let mut successful_steals = 0;
    for i in 0..1000 {
        if pop_results[i] {
            successful_pops += 1;
        }
        if steal_results[i] {
            successful_steals += 1;
        }
    }

    assert_eq!(
        successful_pops + successful_steals,
        1000,
        "successful_pops: {}, successful_steals: {}",
        successful_pops,
        successful_steals
    );
}

#[test]
fn should_push_and_pop_from_one_thread_while_one_is_stealing_on_empty_buffer() {
    retry(10, || {
        let buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let pop_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));

        let push_pop_buffer = buffer.clone();
        let push_results_copy = push_results.clone();
        let pop_results_copy = pop_results.clone();
        let push_pop_handle = thread::spawn(move || {
            for _ in 0..100 {
                for _ in 0..10 {
                    let result = unsafe { push_pop_buffer.push(Job::new(|| {})) };
                    push_results_copy.lock().unwrap().push(result.is_ok());
                }

                for _ in 0..10 {
                    let result = unsafe { push_pop_buffer.wait_and_pop() };
                    pop_results_copy.lock().unwrap().push(result.is_ok());
                }
            }
        });

        let steal_buffer = duplicate(&buffer);
        let steal_results_copy = steal_results.clone();
        let steal_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = steal_buffer.steal();
                steal_results_copy.lock().unwrap().push(result.is_ok());
            }
        });

        push_pop_handle.join().unwrap();
        steal_handle.join().unwrap();

        let push_results = push_results.lock().unwrap();
        let pop_results = pop_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();

        assert_eq!(push_results.len(), 1000);
        assert_eq!(pop_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);

        let mut successful_pushes = 0;
        let mut successful_pops = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                successful_pushes += 1;
            }
            if pop_results[i] {
                successful_pops += 1;
            }
            if steal_results[i] {
                successful_steals += 1;
            }
        }

        assert!(
            successful_pushes > 950,
            "successful_pushes: {}",
            successful_pushes
        );
        assert_eq!(
            successful_pops + successful_steals,
            successful_pushes,
            "successful_pops: {}, successful_steals: {}",
            successful_pops,
            successful_steals
        );
    });
}

#[test]
fn should_push_and_pop_from_one_thread_while_one_is_stealing_on_full_buffer() {
    retry(10, || {
        let buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let pop_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..1000 {
            unsafe { buffer.push(Job::new(|| {})) }.unwrap();
        }

        let push_pop_buffer = buffer.clone();
        let push_results_copy = push_results.clone();
        let pop_results_copy = pop_results.clone();
        let push_pop_handle = thread::spawn(move || {
            for _ in 0..100 {
                for _ in 0..10 {
                    let result = unsafe { push_pop_buffer.push(Job::new(|| {})) };
                    push_results_copy.lock().unwrap().push(result.is_ok());
                }

                for _ in 0..10 {
                    let result = unsafe { push_pop_buffer.wait_and_pop() };
                    pop_results_copy.lock().unwrap().push(result.is_ok());
                }
            }
        });

        let steal_buffer = duplicate(&buffer);
        let steal_results_copy = steal_results.clone();
        let steal_handle = thread::spawn(move || {
            for _ in 0..1000 {
                let result = steal_buffer.steal();
                steal_results_copy.lock().unwrap().push(result.is_ok());
            }
        });

        push_pop_handle.join().unwrap();
        steal_handle.join().unwrap();

        let push_results = push_results.lock().unwrap();
        let pop_results = pop_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();

        assert_eq!(push_results.len(), 1000);
        assert_eq!(pop_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);

        let mut successful_pushes = 0;
        let mut successful_pops = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                successful_pushes += 1;
            }
            if pop_results[i] {
                successful_pops += 1;
            }
            if steal_results[i] {
                successful_steals += 1;
            }
        }

        // pushing on here is more reliable than the other full buffer tests,
        // because the additional pop in the same threads guarantees that there
        // is some room in the buffer to push to
        assert!(
            successful_pushes > 950,
            "successful_pushes: {}",
            successful_pushes
        );
        assert_eq!(
            successful_pops + successful_steals,
            successful_pushes + 1000,
            "successful_pops: {}, successful_steals: {}",
            successful_pops,
            successful_steals
        );
    });
}

#[test]
fn should_push_and_pop_from_one_thread_while_mutliple_are_stealing_on_empty_buffer() {
    retry(10, || {
        let buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let pop_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));

        let push_pop_buffer = buffer.clone();
        let push_results_copy = push_results.clone();
        let pop_results_copy = pop_results.clone();
        let push_pop_handle = thread::spawn(move || {
            for _ in 0..100 {
                for _ in 0..10 {
                    let result = unsafe { push_pop_buffer.push(Job::new(|| {})) };
                    push_results_copy.lock().unwrap().push(result.is_ok());
                }

                for _ in 0..10 {
                    let result = unsafe { push_pop_buffer.wait_and_pop() };
                    pop_results_copy.lock().unwrap().push(result.is_ok());
                }
            }
        });

        let mut steal_handles = Vec::new();
        for _ in 0..10 {
            let steal_buffer = buffer.clone();
            let steal_results_copy = steal_results.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let result = steal_buffer.steal();
                    steal_results_copy.lock().unwrap().push(result.is_ok());
                }
            });
            steal_handles.push(handle);
        }

        push_pop_handle.join().unwrap();

        for handle in steal_handles {
            handle.join().unwrap();
        }

        let push_results = push_results.lock().unwrap();
        let pop_results = pop_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();

        assert_eq!(push_results.len(), 1000);
        assert_eq!(pop_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);

        let mut successful_pushes = 0;
        let mut successful_pops = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                successful_pushes += 1;
            }
            if pop_results[i] {
                successful_pops += 1;
            }
            if steal_results[i] {
                successful_steals += 1;
            }
        }

        // pushing on here is less reliable than the other empty buffer tests,
        // because the additional steals allow pushes, to access locked nodes.
        // in this implementation this is non-blocking and returns an error
        assert!(
            successful_pushes > 950,
            "successful_pushes: {}",
            successful_pushes
        );
        assert_eq!(
            successful_pops + successful_steals,
            successful_pushes,
            "successful_pops: {}, successful_steals: {}",
            successful_pops,
            successful_steals
        );
    });
}

#[test]
fn should_push_and_pop_from_one_thread_while_mutliple_are_stealing_on_full_buffer() {
    retry(10, || {
        let buffer = JobBuffer::new(1000);
        let push_results = Arc::new(Mutex::new(Vec::new()));
        let pop_results = Arc::new(Mutex::new(Vec::new()));
        let steal_results = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..1000 {
            unsafe { buffer.push(Job::new(|| {})).unwrap() };
        }

        let push_pop_buffer = buffer.clone();
        let push_results_copy = push_results.clone();
        let pop_results_copy = pop_results.clone();
        let push_pop_handle = thread::spawn(move || {
            for _ in 0..100 {
                for _ in 0..10 {
                    let result = unsafe { push_pop_buffer.push(Job::new(|| {})) };
                    push_results_copy.lock().unwrap().push(result.is_ok());
                }

                for _ in 0..10 {
                    let result = unsafe { push_pop_buffer.wait_and_pop() };
                    pop_results_copy.lock().unwrap().push(result.is_ok());
                }
            }
        });

        let mut steal_handles = Vec::new();
        for _ in 0..10 {
            let steal_buffer = buffer.clone();
            let steal_results_copy = steal_results.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let result = steal_buffer.steal();
                    steal_results_copy.lock().unwrap().push(result.is_ok());
                }
            });
            steal_handles.push(handle);
        }

        push_pop_handle.join().unwrap();

        for handle in steal_handles {
            handle.join().unwrap();
        }

        let push_results = push_results.lock().unwrap();
        let pop_results = pop_results.lock().unwrap();
        let steal_results = steal_results.lock().unwrap();

        assert_eq!(push_results.len(), 1000);
        assert_eq!(pop_results.len(), 1000);
        assert_eq!(steal_results.len(), 1000);

        let mut successful_pushes = 0;
        let mut successful_pops = 0;
        let mut successful_steals = 0;
        for i in 0..1000 {
            if push_results[i] {
                successful_pushes += 1;
            }
            if pop_results[i] {
                successful_pops += 1;
            }
            if steal_results[i] {
                successful_steals += 1;
            }
        }

        let mut unsuccessful_pops = 0;
        while unsafe { buffer.wait_and_pop() }.is_ok() {
            unsuccessful_pops += 1;
        }

        // pushing on here is more reliable than the other full buffer tests,
        // because the additional pop in the same threads guarantees that there
        // is some room in the buffer to push to. also, because of the high
        // contention, not all jobs may be popped off, thus the buffer is still
        // filled, and unpopped jobs are counted
        assert!(
            successful_pushes > 950,
            "successful_pushes: {}",
            successful_pushes
        );
        assert_eq!(
            successful_pops + successful_steals + unsuccessful_pops,
            successful_pushes + 1000,
            "successful_pops: {}, successful_steals: {}, unsuccessful_pops: {}",
            successful_pops,
            successful_steals,
            unsuccessful_pops
        );
    });
}
