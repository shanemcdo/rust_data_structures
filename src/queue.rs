pub struct Queue<T> {
    vals: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { vals: vec![] }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { vals: vec }
    }

    pub fn push(&mut self, val: T) {
        self.vals.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        // very very slow but I don't care :)
        self.vals.reverse();
        let res = self.vals.pop();
        self.vals.reverse();
        res
    }

    pub fn is_empty(&self) -> bool {
        self.vals.is_empty()
    }
}
