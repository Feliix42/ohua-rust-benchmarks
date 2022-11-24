use std::thread;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

fn threads_fun(xs: Vec<u32>) -> u32 {
    let z = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for x in xs {
        let zc = z.clone();
        let handle =
            thread::spawn(move || {
                let y = x + 1;
                zc.store(y, Ordering::Relaxed)
            });
        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap() }

    let zp = Arc::try_unwrap(z).unwrap().into_inner();
    zp
}

struct State {
    z: u32,
}

impl State {
    fn new() -> Self { State { z : 0 } }
    fn update(&mut self, x:u32) { self.z = x; }
    fn get(self) -> u32 { self.z }
}


fn ohua_fun(xs: Vec<u32>) -> u32 {
    let mut z = State::new();
    for x in xs {
        let y = x + 1;
        z.update(y);
    }
    z.get()
}

fn seq_fun(xs: Vec<u32>) -> u32 {
    let mut z = 0;
    for x in xs {
        let y = x + 1;
        z = y;
    }
    z
}

fn unrolled_fun(x0:u32, x1:u32) -> u32 {
    let mut z = 0;
    let y = x0 + 1;
    z = y;

    let y = x1 + 1;
    z = y;

    z
}

#[cfg(test)]
mod test {

    #[test]
    fn check_seq() {
        let xs = vec![1, 2];
        let z = crate::verify::seq_fun(xs);
        assert!(z == 3)
    }

    #[test]
    fn check_threads() {
        let xs = vec![1, 2];
        let z = crate::verify::threads_fun(xs);
        assert!(z == 3 || z == 2)
    }

    #[test]
    fn check_ohua() {
        let xs = vec![1, 2];
        let z = crate::verify::ohua_fun(xs);
        assert!(z == 3)
    }

}

#[cfg(kani)]
mod verification {
    // This is commented out because the check never finishes. Kani does not support concurrent programs:
    // WARN kani_compiler::codegen_cprover_gotoc::codegen::intrinsic Kani does not support concurrency for now.
//    #[kani::unwind(5)]
//    #[kani::proof]
//    fn check_last_threads() {
//        let x0 = kani::any();
//        let x1 = kani::any();
//        let mut xs = Vec::new();
//        xs.push(x0);
//        xs.push(x1);
//        let z = crate::verify::threads_fun(xs);
//        assert!(z == (x0 + 1) || z == (x1 + 1));
//    }

    #[kani::unwind(5)]
    #[kani::proof]
    fn check_last_ohua() {
        let (x0, x1) = (kani::any(), kani::any());
        /*
        let x0 = kani::any();
        let x1 = kani::any();
        let mut xs = Vec::new();
        xs.push(x0);
        xs.push(x1);
         */
        let xs = vec![x0,x1];
        let z = crate::verify::ohua_fun(xs);
        assert!(z == (x1 + 1));
    }

    #[kani::unwind(5)]
    #[kani::proof]
    fn check_last_vec() {
        let x0 = kani::any();
        let x1 = kani::any();
        let mut xs = Vec::new();
        xs.push(x0);
        xs.push(x1);
        let z = crate::verify::seq_fun(xs);
        assert!(z == (x1 + 1));
    }

    #[kani::proof]
    fn check_last_unrolled() {
        let x0 = kani::any();
        let x1 = kani::any();
        let z = crate::verify::unrolled_fun(x0, x1);
        assert!(z == (x1 + 1));
    }

}
