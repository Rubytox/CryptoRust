struct ZpZ {
    modulus: u32
}

impl ZpZ {
    fn get(&self, value: u32) -> u32 {
        value % self.modulus
    }

    fn field(&self) -> bool {
        // Find how to do that efficiently
    }
}

fn main() {
    let modulus = 2;
    let group = ZpZ {
        modulus
    };

    for number in 0..10 {
        println!("{} = {} [{}]", number, group.get(number), modulus);
    }

}