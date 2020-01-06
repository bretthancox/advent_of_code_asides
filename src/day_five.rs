#![allow(dead_code)]
use crate::file_open;
use crate::intcode::*;





pub fn day_five() {
    let file_input = file_open();
    let input_string = file_input.unwrap();
    let int_vec = input_string.split(",").collect::<Vec<&str>>().iter().map(|o| o.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let mut intcode_obj = Intcode::new(int_vec);
    //println!("Struct: {:?}\nInternal intcode: {:?}", intcode_obj, intcode_obj.intcode);

    let opcode_results = intcode_obj.work_on_intcode_manual();
    println!("{:?}", opcode_results);
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;//{assert_eq, assert_ne};

    #[test]
    fn test_extended_opcode_maker_1() {
        let input = vec![10001, 0, 2, 1];
        let intcode_obj_test = Intcode::new(input);
        let ext_opcode_test = intcode_obj_test.get_opcode_method();
        //println!("test_extended_opcode_maker_1: {:?}", intcode_obj_test);
        assert_eq!(ext_opcode_test, ExtendedOpcode { opcode: Opcode::Add, mode_1: Modes::Position, mode_2: Modes::Position, mode_3: Modes::Immediate});
        
    }

    #[test]
    fn test_extended_opcode_maker_2() {
        let input = vec![10001, 0, 2, 1, 11002, 2, 2, 5, 99];
        let intcode_obj_test = Intcode::new(input);
        let ext_opcode_test = intcode_obj_test.get_opcode_method();
        assert_eq!(ext_opcode_test, ExtendedOpcode { opcode: Opcode::Add, mode_1: Modes::Position, mode_2: Modes::Position, mode_3: Modes::Immediate});
    }

    #[test]
    fn test_intcode_worker_1() {
        let input = vec![10001, 0, 2, 1, 11002, 2, 2, 5, 99];
        let mut intcode_obj_test = Intcode::new(input);
        let ext_opcode_test = intcode_obj_test.work_on_intcode();
        assert_eq!(intcode_obj_test.intcode, vec![10001, 0, 2, 10003, 11002, 2, 2, 4, 99]);
    }
}