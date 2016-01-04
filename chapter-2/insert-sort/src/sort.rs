#[derive(Debug)]
pub struct Algo {
    vec: Vec<String>,
}
impl Algo {
    pub fn new(vec: Vec<String>) -> Self {
        Algo { vec: vec }
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn is_sorted(&self) -> bool {
        let mut sorted = true;
        for i in 0..self.len() - 1 {
            if self.vec[i] > self.vec[i + 1] {
                sorted = false;
            }
        }
        sorted
    }
    pub fn sort(&mut self) {
        for i in 1..self.vec.len() {
            println!("e{}", i);
            for j in (0..(i + 1)).rev() {
                let index = j as usize;
                println!("{}", index);
                if (index == 0) || self.vec[index] > self.vec[index - 1] {
                    break;
                }
                self.vec.swap(index, index - 1)
            }
        }
    }
}
