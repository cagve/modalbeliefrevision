struct World{
    atoms: String
    bytes: Vec<u8>,
}



impl World {
    fn calculate_bytes(&mut self) {
        self.bytes = self.value.as_bytes().to_vec();
    }
}

