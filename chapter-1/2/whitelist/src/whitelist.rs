pub struct StaticSetOfInts {
    array_of_ints: Vec<u64>,
}

impl StaticSetOfInts {
    pub fn new(mut array: Vec<u64>) -> StaticSetOfInts {
        array.sort();
        return StaticSetOfInts { array_of_ints: array };
    }
    pub fn contains(&self, number: u64) -> bool {
        match self.array_of_ints.binary_search(&number) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
