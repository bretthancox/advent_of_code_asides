#![allow(dead_code)]
use crate::input_file_handler::file_open;


#[derive(Debug, Clone, PartialEq)]
enum Opcode {
    Stop,
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    OtherValue,
}


#[derive(Debug, Clone, PartialEq)]
enum Modes {
    Position,
    Immediate,
}

#[derive(Debug, Clone)]
pub struct Intcode {
    intcode: Vec<isize>,
    opcode_pointer: usize,
    continue_loop: bool,
}

#[derive(Debug, Clone)]
pub struct ExtendedOpcode {
    opcode: Opcode,
    mode_1: Modes,
    mode_2: Modes,
    mode_3: Modes,
}


impl PartialEq for ExtendedOpcode {
    /*Without implementing PartialEq, could not compare
    names between different nodes*/
    fn eq(&self, comparator: &Self) -> bool {
        self.opcode == comparator.opcode
        &&
        self.mode_1 == comparator.mode_1
        &&
        self.mode_2 == comparator.mode_2
        &&
        self.mode_3 == comparator.mode_3
    }
}

impl ExtendedOpcode {
    fn new(opcode: Opcode) -> Self {
        Self {
            opcode: opcode,
            mode_1: Modes::Position,
            mode_2: Modes::Position,
            mode_3: Modes::Position,
        }
    }
}


impl Intcode {
    fn new(intcode_vec: Vec<isize>) -> Self {
        Self {
            intcode: intcode_vec,
            opcode_pointer: 0,
            continue_loop: true,
        }
    }





    fn split_opcodes(&self, opcode_full: isize) -> Vec<isize> {
        let unsplit_opcode = opcode_full.to_string();
        let split_opcode = unsplit_opcode.trim().split_terminator("").skip(1).collect::<Vec<&str>>(); //split_terminator("").skip(1) splits the string, but prevents the resulting vector having "" at start and end
        //println!("{:?}", split_opcode);
        let rev_split_opcode = split_opcode.iter().rev().map(|o| o.parse::<isize>().unwrap()).collect::<Vec<isize>>();

        rev_split_opcode
    }


    fn get_opcode_method(&self) -> ExtendedOpcode {
        let opcode_full = self.intcode[self.opcode_pointer];
        let opcode_split = self.split_opcodes(opcode_full);
        //println!("{:?}", opcode_split);
        let mut extended_opcode: ExtendedOpcode = ExtendedOpcode::new(Opcode::OtherValue);

        if opcode_split.len() > 2 && opcode_split[2] == 1 {
            extended_opcode.mode_1 = Modes::Immediate;   
        }

        if opcode_split.len() > 3 && opcode_split[3] == 1 {
            extended_opcode.mode_2 = Modes::Immediate;
        }

        if opcode_split.len() > 4 && opcode_split[4] == 1 {
            extended_opcode.mode_3 = Modes::Immediate;
        }

        match opcode_split[0] {
            9 => match opcode_split[1] {
                9 => extended_opcode.opcode = Opcode::Stop,
                _ => extended_opcode.opcode = Opcode::OtherValue,
            }
            1 => extended_opcode.opcode = Opcode::Add,
            2 => extended_opcode.opcode = Opcode::Multiply,
            3 => extended_opcode.opcode = Opcode::Input,
            4 => extended_opcode.opcode = Opcode::Output,
            5 => extended_opcode.opcode = Opcode::JumpIfTrue,
            6 => extended_opcode.opcode = Opcode::JumpIfFalse,
            7 => extended_opcode.opcode = Opcode::LessThan,
            8 => extended_opcode.opcode = Opcode::Equals,
            _ => extended_opcode.opcode = Opcode::OtherValue,
        }
        //println!("{:?}", self.intcode);
        extended_opcode
    }


    fn add_and_place(&mut self, extended_opcode: ExtendedOpcode) {
        let value_1 = match extended_opcode.mode_1 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 1 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 1 ],
        };

        let value_2 = match extended_opcode.mode_2 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 2 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 2 ],
        };

        let write_to = match extended_opcode.mode_3 {
            Modes::Position => self.intcode[ self.opcode_pointer + 3 ] as usize,
            Modes::Immediate => self.opcode_pointer + 3 as usize,
        };

        self.intcode[write_to] = value_1 + value_2;
        self.opcode_pointer += 4;
    }


    fn multiply_and_place(&mut self, extended_opcode: ExtendedOpcode) {
        let value_1 = match extended_opcode.mode_1 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 1 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 1 ],
        };

        let value_2 = match extended_opcode.mode_2 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 2 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 2 ],
        };

        let write_to = match extended_opcode.mode_3 {
            Modes::Position => self.intcode[ self.opcode_pointer + 3 ] as usize,
            Modes::Immediate => self.opcode_pointer + 3 as usize,
        };

        self.intcode[write_to] = value_1 * value_2;
        self.opcode_pointer += 4;
    }


    fn take_input(&mut self, extended_opcode: ExtendedOpcode) {
        let user_input: isize = read!();

        let write_to = match extended_opcode.mode_1 {
            Modes::Position => self.intcode[ self.opcode_pointer + 1 ] as usize,
            Modes::Immediate => self.opcode_pointer + 1 as usize,
        };
        self.intcode[write_to] = user_input;
        self.opcode_pointer += 2;
    }

    fn print_output(&mut self, extended_opcode: ExtendedOpcode) {
        let read_from = match extended_opcode.mode_1 {
            Modes::Position => self.intcode[ self.opcode_pointer + 1 ] as usize,
            Modes::Immediate => self.opcode_pointer + 1 as usize,
        };
        let val_to_output = self.intcode[read_from];
        println!("Diagnostic code: {:?}", val_to_output);
        self.opcode_pointer += 2;
    }

    fn jump_if_true(&mut self, extended_opcode: ExtendedOpcode) {
        let value_1 = match extended_opcode.mode_1 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 1 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 1 ],
        };

        let value_2 = match extended_opcode.mode_2 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 2 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 2 ],
        };

        if value_1 != 0 {
            self.opcode_pointer = value_2 as usize;
        } else {
            self.opcode_pointer += 3;
        }
    }

    fn jump_if_false(&mut self, extended_opcode: ExtendedOpcode) {
        let value_1 = match extended_opcode.mode_1 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 1 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 1 ],
        };

        let value_2 = match extended_opcode.mode_2 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 2 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 2 ],
        };

        if value_1 == 0 {
            self.opcode_pointer = value_2 as usize;
        } else {
            self.opcode_pointer += 3;
        }
    }

    fn less_than(&mut self, extended_opcode: ExtendedOpcode) {
        let value_1 = match extended_opcode.mode_1 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 1 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 1 ],
        };

        let value_2 = match extended_opcode.mode_2 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 2 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 2 ],
        };

        let write_to = match extended_opcode.mode_3 {
            Modes::Position => self.intcode[ self.opcode_pointer + 3 ] as usize,
            Modes::Immediate => self.opcode_pointer + 3 as usize,
        };

        if value_1 < value_2 {
            self.intcode[write_to] = 1;
        } else {
            self.intcode[write_to] = 0;
        }

        self.opcode_pointer += 4;
    }

    fn equals(&mut self, extended_opcode: ExtendedOpcode) {
        let value_1 = match extended_opcode.mode_1 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 1 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 1 ],
        };

        let value_2 = match extended_opcode.mode_2 {
            Modes::Position => self.intcode[ self.intcode[ self.opcode_pointer + 2 ] as usize ],
            Modes::Immediate => self.intcode[ self.opcode_pointer + 2 ],
        };

        let write_to = match extended_opcode.mode_3 {
            Modes::Position => self.intcode[ self.opcode_pointer + 3 ] as usize,
            Modes::Immediate => self.opcode_pointer + 3 as usize,
        };

        if value_1 == value_2 {
            self.intcode[write_to] = 1;
        } else {
            self.intcode[write_to] = 0;
        }

        self.opcode_pointer += 4;
    }

    fn work_on_intcode(&mut self) -> Result<(), &str> {
        while self.continue_loop {

            let extended_opcode = self.get_opcode_method();
            println!("Opcode pointer: {:?}\nOpcode: {:?}\nOpcode value: {:?}", self.opcode_pointer, extended_opcode.opcode, self.intcode[self.opcode_pointer]);

            match extended_opcode.opcode {
                Opcode::Stop => { 
                    println!("Finished.");
                    self.continue_loop = false;
                },
                Opcode::Add => self.add_and_place(extended_opcode),
                Opcode::Multiply => self.multiply_and_place(extended_opcode),
                Opcode::Input => self.take_input(extended_opcode),
                Opcode::Output => self.print_output(extended_opcode),
                Opcode::JumpIfTrue => self.jump_if_true(extended_opcode),
                Opcode::JumpIfFalse => self.jump_if_false(extended_opcode),
                Opcode::LessThan => self.less_than(extended_opcode),
                Opcode::Equals => self.equals(extended_opcode),
                Opcode::OtherValue => return Err("A value outside Opcode range was found."),
                _ => return Err("No idea where this came from..."),
            }
        }

        Ok(())
    }

}


pub fn day_five() {
    let file_input = file_open();
    let input_string = file_input.unwrap();
    let int_vec = input_string.split(",").collect::<Vec<&str>>().iter().map(|o| o.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let mut intcode_obj = Intcode::new(int_vec);
    //println!("Struct: {:?}\nInternal intcode: {:?}", intcode_obj, intcode_obj.intcode);

    let opcode_results = intcode_obj.work_on_intcode();
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