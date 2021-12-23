#![feature(control_flow_enum)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod bus;
pub mod opcode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}