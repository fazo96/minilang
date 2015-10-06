extern crate regex;
use regex::Regex;
use std::io::Read;
use std::fs::File;
use std::str::FromStr;

fn readInt() -> i32 {
    let mut input = String::new();
    let string = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    string as i32
}

#[derive(Debug)]
enum Refs {
    Memory { index: i32 },
    Literal { value: i32 }
}

// Takes out i32 from a wrapper, like "Mem[10]"
fn conv(string: &str,regex : &Regex) -> Refs {
    match regex.is_match(&string) {
        true => Refs::Memory { 
            index: regex.captures(string).unwrap().at(1).unwrap().parse::<i32>().unwrap(),
        },
        false => Refs::Literal { value: string.parse::<i32>().unwrap() }
    }
}

#[derive(Debug)]
enum Conditions {
    Greater,
    GreaterEqual,
    Equal,
    LesserEqual,
    Lesser
}

#[derive(Debug)]
enum Istructions {
    Assignment { index: i32, value: i32 },
    Jump { to: i32 },
    Halt,
    Pass,
    Write { index: i32 },
    Read { index: i32 },
    If { a: Refs, condition: Conditions, b: Refs, jump: i32 }
}

struct Istruction {
    istruction: Istructions
}

impl Istruction {
    fn execute(&self,pc: &i32,mem: &mut Vec<i32>) -> i32 {
        match &self.istruction {
            &Istructions::Pass => pc + 1,
            &Istructions::Jump { to } => to,
            &Istructions::Write { index } => {
                println!("Output: {}",mem[index as usize]);
                return pc + 1;
            },
            &Istructions::Halt => -1,
            &Istructions::Assignment { index, value } => {
                mem[index as usize] = value;
                return pc + 1;
            },
            &Istructions::Read { index } => {
                let i = readInt();
                mem[index as usize] = i;
                return pc + 1;
            },
            &Istructions::If { ref a, ref condition, ref b, ref jump } => {
                return -1
            }
        }
    }
}

struct VM {
    mem: Vec<i32>,
    code: Vec<Istruction>,
    pc: i32
}

impl VM {
    fn new() -> VM {
        VM {
            mem: Vec::new() as Vec<i32>,
            code: Vec::new() as Vec<Istruction>,
            pc: 0
        }
    }
    fn run(&mut self) {
        self.pc = 0;
        while self.pc >= 0 {
           self.pc = self.code[self.pc as usize].execute(&self.pc,&mut self.mem); 
        }
    }
}

fn main() {
    let mut f = File::open("program.txt").ok().expect("Error opening file");
    let mut program = String::new();
    let mut pc = 0;
    let mut vm = VM::new();

    let istrn_regex = Regex::new(r"^(\d+): ").unwrap();
    let halt_regex = Regex::new(r"^halt$").unwrap();
    let pass_regex = Regex::new(r"^pass$").unwrap();
    let goto_regex = Regex::new(r"^goto (\d+)$").unwrap();
    let read_regex = Regex::new(r"^read\((\d+)\)$").unwrap();
    let write_regex = Regex::new(r"^write\((\d+)\)$").unwrap();
    let assignment_regex = Regex::new(r"^Mem\[(\d+)\]:= (Mem\[(\d+)\])((\+|-)(Mem\[(\d+)\]))*$").unwrap();
    let if_regex = Regex::new(r"^if (?P<a>\d+|Mem\[\d+\]) (?P<o>>|<|=){1}(?P<p>={0,1}) (?P<b>\d+|Mem\[\d+\]) then goto (?P<t>\d+)$").unwrap();

    let mem_regex = Regex::new(r"^Mem\[(\d+)\]$").unwrap();

    f.read_to_string(&mut program);
    for line in program.lines() {
        pc = pc + 1;
        let l = line.trim().to_lowercase();
        println!("({}) Parsing Istruction: {}",pc,l);
        let istrs = istrn_regex.captures(&l).unwrap().at(1).unwrap_or("-1");
        let istrn = istrs.parse::<i32>().unwrap();
        if istrn != pc {
            println!("ISTRUCTION: {} HAS INVALID NUMBER",l);
            break;
        }

        let istr : &str = & istrn_regex.replace_all(line,"");
        println!("({}) Processing: {}",pc,istr);
        if halt_regex.is_match(&istr) { // HALT
            vm.code.push(Istruction { istruction: Istructions::Halt });   
        } else if pass_regex.is_match(&istr) { // PASS
            vm.code.push(Istruction { istruction: Istructions::Pass });
        } else if goto_regex.is_match(&istr) { // GOTO
            let to = goto_regex.captures(&istr).unwrap().at(1).unwrap().parse::<i32>().unwrap();
            vm.code.push(Istruction { istruction: Istructions::Jump { to: to } });
        } else if write_regex.is_match(&istr) { // WRITE
            let to = write_regex.captures(&istr).unwrap().at(1).unwrap().parse::<i32>().unwrap();
            vm.code.push(Istruction { istruction: Istructions::Write { index: to } });
        } else if read_regex.is_match(&istr) { // READ
            let to = read_regex.captures(&istr).unwrap().at(1).unwrap().parse::<i32>().unwrap();
            vm.code.push(Istruction { istruction: Istructions::Read { index: to } });
        } else if if_regex.is_match(&istr) { // IF
            let a_str = if_regex.replace_all(istr,"$a");
            let b_str = if_regex.replace_all(istr,"$b");
            let a = conv(&a_str,&mem_regex);
            let b = conv(&b_str,&mem_regex);
            let op_str : &str = & if_regex.replace_all(istr,"$o$p");
            let oper = match op_str {
                ">" => Conditions::Greater,
                "<" => Conditions::Lesser,
                ">=" => Conditions::GreaterEqual,
                "<=" => Conditions::LesserEqual,
                "=" => Conditions::Equal,
                "==" => Conditions::Equal,
                _ => panic!("What")
            };
            let jmp = if_regex.replace_all(istr,"$t").parse::<i32>().unwrap();
            vm.code.push(Istruction { istruction: Istructions::If { a: a, b: b, condition: oper, jump: jmp } });
        } else if assignment_regex.is_match(&istr) { // ASSIGN
              
        } else { // UNKNOWN
            println!("({}) UNKNOWN ISTRUCTION: {}",pc,istr);
            break;
        }
    }
    println!("--- ISTRUCTIONS");
    for istr in vm.code {
        println!("{:?}",istr.istruction);
    }
}
