pub struct SingletonInt {
}

static mut GLOBAL: u32 = 0;
impl SingletonInt {
    pub fn get_value(&self) -> u32 {
        unsafe {
            let value = GLOBAL;
            return value;
        }
    }

    pub fn increment_value(&self) {
        unsafe {
            GLOBAL += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let singleton_int = SingletonInt{};
        assert_eq!(singleton_int.get_value(), 0);
        singleton_int.increment_value();
        assert_eq!(singleton_int.get_value(), 1);
        let singleton_int2  = SingletonInt{};
        singleton_int2.increment_value();
        assert_eq!(singleton_int2.get_value(), 2);

    }
}
