#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub type Result<T> = core::result::Result<T, ()>;

    fn read_byte(&mut self, addr: u8) -> Result<u8>;
    fn write_byte(&mut self, addr: u8, data: u8) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
