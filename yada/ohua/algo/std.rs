struct Vec<T> {}
impl<T> Vec<T> {
    //pub fn default() -> Self {
        //unimplemented!()
    //}
    pub fn push(&mut self, value: Duration) {
        unimplemented!()
    }
    pub fn evict_mapped(&mut self) {
        unimplemented!()
    }
    pub fn calculate_done1(&mut self, its_left: u32) -> bool {
        unimplemented!()
    }
    pub fn len(&self) -> usize {
        unimplemented!()
    }
    pub fn exp(&mut self, other: Self) {
        unimplemented!()
    }
    pub fn new2() -> Self {
        unimplemented!()
    }
}

enum Option<T> {}

struct Arc<T> {}
impl<T> Arc<T> {
    fn new1(i: T) -> Self {
        unimplemented!()
    }
}
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

pub fn id<T>(item: T) -> T {
    unimplemented!()
}

pub fn unpack(store: Arc<Store>, res: Vec<Option<(Message, TcpStream)>>) -> (Store, Vec<Option<(Message, TcpStream)>>) {
    unimplemented!()
}

pub fn cont() -> bool {
    true
}
