#![allow(dead_code)]
use crate::file_open;
use crate::intcode::*;

#[derive(Debug)]
pub struct AmpResult {
    pub output: isize,
    pub input_a: isize,
    pub input_b: isize,
    pub input_c: isize,
    pub input_d: isize,
    pub input_e: isize,
}

impl AmpResult {
    fn new() -> Self {
        Self {
            output:  0,
            input_a: 0,
            input_b: 0,
            input_c: 0,
            input_d: 0,
            input_e: 0,
        }
    }
}

pub fn iterating_over_the_inputs(int_vec: Vec<isize>, range_lower: isize, range_upper: isize) -> AmpResult {
    let mut result = AmpResult::new();

    for m in range_lower..=range_upper {
        let mut amp_a = Intcode::new(int_vec.clone());
        amp_a.work_on_intcode_auto(m, 0).unwrap();

        for n in range_lower..=range_upper {
            if n == m { continue; }
            let mut amp_b = Intcode::new(amp_a.intcode.clone());
            amp_b.work_on_intcode_auto(n, amp_a.debug_output).unwrap();

            for p in range_lower..=range_upper {
                if p == n || p == m { continue; }
                let mut amp_c = Intcode::new(amp_b.intcode.clone());
                amp_c.work_on_intcode_auto(p, amp_b.debug_output).unwrap();

                for q in range_lower..=range_upper {
                    if q == n || q == m || q == p { continue; }
                    let mut amp_d = Intcode::new(amp_c.intcode.clone());
                    amp_d.work_on_intcode_auto(q, amp_c.debug_output).unwrap();

                    for r in range_lower..=range_upper {
                        if r == n || r == m || r == p || r == q { continue; }
                        let mut amp_e = Intcode::new(amp_d.intcode.clone());
                        amp_e.work_on_intcode_auto(r, amp_d.debug_output);
                        println!("A: {}, B: {}, C: {}, D: {}, E: {}", m,n,p,q,r);
                        println!("A vec: {:?}\nB vec: {:?}\nC vec: {:?}\nD vec: {:?}\nE vec: {:?}", amp_a.intcode, amp_b.intcode, amp_c.intcode, amp_d.intcode, amp_e.intcode);

                        if amp_e.debug_output > result.output {
                            result.output = amp_e.debug_output;
                            result.input_a = m;
                            result.input_b = n;
                            result.input_c = p;
                            result.input_d = q;
                            result.input_e = r;
                        }
                    }
                }
            }
        }
    }

    result
}


pub fn day_seven() {
    let file_input = file_open();
    let input_string = file_input.unwrap();
    let int_vec = input_string.split(",").collect::<Vec<&str>>().iter().map(|o| o.parse::<isize>().unwrap()).collect::<Vec<isize>>();

    let best_result = iterating_over_the_inputs(int_vec, 0, 4);

    println!("{:?}", best_result);

    //println!("Struct: {:?}\nInternal intcode: {:?}", intcode_obj, intcode_obj.intcode);

    //let opcode_results = intcode_obj.work_on_intcode_manual();
    //println!("{:?}", opcode_results);
}