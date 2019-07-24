use ncurses::*; // watch for globs
use std::convert::TryInto;
use std::ops::Index;

pub struct State {
    data: Vec<u8>,
    ptr: usize,
    pc: usize,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    IncrPtr,
    DecrPtr,
    IncrVal,
    DecrVal,
    GetChr,
    PutChr,
    StartLoop(usize),
    EndLoop(usize),
}

pub fn compile(s: &[u8]) -> Vec<Op> {
    let (_, program) = compile_intern(s);
    program
}

fn compile_intern(s: &[u8]) -> (usize, Vec<Op>) {
    let mut program = Vec::with_capacity(s.len());
    let mut i: usize = 0;

    // static DP: u8 = '<' as u8;
    // static IP: u8 = '>' as u8;
    // static IV: u8 = '+' as u8;
    // static DV: u8 = '-' as u8;
    // static PC: u8 = '.' as u8;
    // static GC: u8 = ',' as u8;
    // static SL: u8 = '[' as u8;
    // static EL: u8 = '[' as u8;
    // 60
    // 62
    // 43
    // 45
    // 46
    // 44
    // 91
    // 93

    while i < s.len() {
        let c = s[i];
        println!("{}", c);
        match c {
            60 => {
                program.push(Op::DecrPtr);
            }
            62 => {
                program.push(Op::IncrPtr);
            }
            43 => {
                program.push(Op::IncrVal);
            }
            45 => {
                program.push(Op::DecrVal);
            }
            46 => {
                program.push(Op::PutChr);
            }
            44 => {
                program.push(Op::GetChr);
            }
            91 => {
                let (_, sub_data) = s.split_at(i + 1);
                let (mut new_index, mut subprogram) = compile_intern(sub_data);
                program.push(Op::StartLoop(i + new_index + 1));
                program.append(&mut subprogram);
                program.push(Op::EndLoop(i));
                i += new_index + 1;
            }
            93 => return (i, program),
            _ => {}
        }
        println!("{} {}", i, c);
        i += 1;
    }
    (i, program)
}

impl State {
    pub fn new() -> State {
        let mut vec = Vec::with_capacity(1000);
        vec.resize(1000, 0);
        State {
            data: Vec::with_capacity(1000),
            ptr: 0,
            pc: 0,
        }
    }
    /*
           pub fn increment_ptr(&mut self) {
           self.ptr += 1;
           }

           pub fn decrement_ptr(&mut self) {
           self.ptr -= 1;
           }

           pub fn increment_value(&mut self) {
           self.data[self.ptr] += 1;
           }

           pub fn decrement_value(&mut self) {
           self.data[self.ptr] -= 1;
           }

           pub fn getchar(&mut self) {
    //Can panic
    self.data[self.ptr] = getch().try_into().unwrap();
    }

    pub fn putchar(&mut self) {
    printw(format!("{}", self.data[self.ptr] as char).as_str());
    }

    pub fn start_loop(&mut self) {
    if self.data[self.ptr] != 0 {

    }
    }
    */
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_compile() {
        let input = "<>+-.,[]";
        let result = compile(input.as_bytes());
        assert_eq!(
            result,
            vec!(
                Op::DecrPtr,
                Op::IncrPtr,
                Op::IncrVal,
                Op::DecrVal,
                Op::PutChr,
                Op::GetChr,
                Op::StartLoop(7),
                Op::EndLoop(6)
            )
        );
    }

}
