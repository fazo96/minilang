extern crate regex;
use regex::Regex;
use std::io;
use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::num::ParseIntError;

fn readInt() -> i32 {
    let stdin = io::stdin();
    let mut string = String::new();
    stdin.read_line(&mut string);
    string.trim().parse::<i32>().unwrap()
}

#[derive(Debug)]
enum Refs {
    Memory { index: i32 },
    Literal { value: i32 }
}

// Takes out i32 from a wrapper, like "Mem[10]"
fn conv(string: &str,regex : &Regex, mem: &mut Vec<i32>) -> Refs {
    match regex.is_match(&string) {
        true => {
            let i = regex.captures(string).unwrap().at(1).unwrap().parse::<i32>().unwrap();
            while mem.len() <= (i as usize) {
                mem.push(0);
                println!("mem.len() is now {}",mem.len());
            }
            Refs::Memory { 
                index: i
            }
        },
        false => match string.parse::<i32>() {
            Ok(v) => Refs::Literal { value: v },
            Err(e) => Refs::Literal { value: 0 }
        }
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

fn get_operator(string: &str) -> Conditions {
    match string {
        ">" => Conditions::Greater,
        "<" => Conditions::Lesser,
        ">=" => Conditions::GreaterEqual,
        "<=" => Conditions::LesserEqual,
        "=" => Conditions::Equal,
        "==" => Conditions::Equal,
        _ => panic!("What")
    }
}

#[derive(Debug)]
enum Operators {
    Plus { arg: Refs },
    Minus { arg: Refs }
}

#[derive(Debug)]
enum Istructions {
    Assignment { index: i32, value: Refs, operator: Operators },
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

fn deref(arg : &Refs, mem : &Vec<i32>) -> i32 {
    match arg {
        &Refs::Memory { index } => {
            let a : usize = index as usize;
            match mem.get(a) {
                Some(x) => {
                    let y : i32 = * x;
                    y
                },
                None => 0
            }
        },
        &Refs::Literal { value } => value
    }
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
            &Istructions::Assignment { ref index, ref value, ref operator } => {
                let v = deref(value,&mem);
                let v2 = match operator {
                    &Operators::Plus { ref arg } => deref(&arg,&mem),
                    &Operators::Minus { ref arg } => deref(&arg,&mem) * -1
                };
                let a : i32 = * index;
                mem[a as usize] = v + v2;
                println!("Stored {} in {}",v+v2,a as usize);
                return pc + 1;
            },
            &Istructions::Read { index } => {
                let i = readInt();
                mem[index as usize] = i;
                println!("Stored {} in {}",i,index as usize);
                return pc + 1;
            },
            &Istructions::If { ref a, ref condition, ref b, ref jump } => {
                let aa = deref(a,&mem);
                let bb = deref(b,&mem);
                let is_true = match condition {
                    &Conditions::Greater => aa > bb,
                    &Conditions::GreaterEqual => aa >= bb,
                    &Conditions::Lesser => aa < bb,
                    &Conditions::LesserEqual => aa <= bb,
                    &Conditions::Equal => aa == bb
                };
                match is_true {
                    true => {
                        let r : i32 = *jump;
                        println!("Condition true, jumping to {}",jump);
                        r
                    },
                    false => pc + 1
                }
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
        self.pc = 1;
        while self.pc >= 1 {
           if self.code.len() <= (self.pc as usize) { break; }
           println!("Running instruction: {}",self.pc);
           self.pc = self.code[(self.pc - 1) as usize].execute(&self.pc,&mut self.mem); 
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
    let assign_regex = Regex::new(r"^Mem\[(?P<t>\d+)\][ ]*:=[ ]*(?P<a>\d+|Mem\[\d+\])([ ]*(?P<o>\+|-)([ ]*(?P<b>\d+|Mem\[\d+\]))){0,1}$").unwrap();
    let if_regex = Regex::new(r"^if (?P<a>\d+|Mem\[\d+\])[ ]*(?P<o>>|<|=){1}(?P<p>={0,1})[ ]*(?P<b>\d+|Mem\[\d+\]) then goto (?P<t>\d+)$").unwrap();

    let mem_regex = Regex::new(r"^Mem\[(\d+)\]$").unwrap();
    let num_regex = Regex::new(r"^\d+$").unwrap();

    f.read_to_string(&mut program);
    for line in program.lines() {
        pc = pc + 1;
        let mut code = &mut vm.code;
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
            code.push(Istruction { istruction: Istructions::Halt });   
        } else if pass_regex.is_match(&istr) { // PASS
            code.push(Istruction { istruction: Istructions::Pass });
        } else if goto_regex.is_match(&istr) { // GOTO
            let to = goto_regex.captures(&istr).unwrap().at(1).unwrap().parse::<i32>().unwrap();
            code.push(Istruction { istruction: Istructions::Jump { to: to } });
        } else if write_regex.is_match(&istr) { // WRITE
            let to = write_regex.captures(&istr).unwrap().at(1).unwrap().parse::<i32>().unwrap();
            code.push(Istruction { istruction: Istructions::Write { index: to } });
        } else if read_regex.is_match(&istr) { // READ
            let to = read_regex.captures(&istr).unwrap().at(1).unwrap().parse::<i32>().unwrap();
            code.push(Istruction { istruction: Istructions::Read { index: to } });
        } else if if_regex.is_match(&istr) { // IF
            let a_str = if_regex.replace_all(istr,"$a");
            let b_str = if_regex.replace_all(istr,"$b");
            let a = conv(&a_str,&mem_regex,&mut vm.mem);
            let b = conv(&b_str,&mem_regex,&mut vm.mem);
            let op_str : &str = & if_regex.replace_all(istr,"$o$p");
            let oper = get_operator(&op_str);
            let jmp = if_regex.replace_all(istr,"$t").parse::<i32>().unwrap();
            code.push(Istruction { istruction: Istructions::If { a: a, b: b, condition: oper, jump: jmp } });
        } else if assign_regex.is_match(&istr) { // ASSIGN
            let target = assign_regex.replace_all(istr,"$t").parse::<i32>().unwrap();
            while vm.mem.len() <= (target as usize) {
                vm.mem.push(0);
                println!("mem.len() is now {}",vm.mem.len());
            }
            let a_str = assign_regex.replace_all(istr,"$a");
            let b_str = assign_regex.replace_all(istr,"$b");
            let a = conv(&a_str,&mem_regex,&mut vm.mem);
            let op : &str = & assign_regex.replace_all(istr,"$o");
            let b = conv(&b_str,&mem_regex,&mut vm.mem);
            let oper = match op {
                "+" => Operators::Plus { arg: b },
                "-" => Operators::Minus { arg: b },
                ""  => Operators::Plus { arg: Refs::Literal { value: 0 } },
                 _  => panic!("Invalid operator")
            };
            code.push(Istruction { istruction: Istructions::Assignment { index: target, value: a, operator: oper } });
        } else { // UNKNOWN
            println!("({}) UNKNOWN ISTRUCTION: {}",pc,istr);
            break;
        }
    }
    println!("--- ISTRUCTIONS");
    for i in &vm.code {
        println!("{:?}",i.istruction);
    }
    println!("--- EXECUTING...");
    vm.run()
}
