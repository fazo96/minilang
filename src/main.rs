extern crate regex;
use regex::Regex;
use std::io;
use std::env;
use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::option::Option;
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
    Modulo { arg: Refs },
    ElevatedTo { arg: Refs },
    DividedBy { arg: Refs },
    Times { arg: Refs },
    Minus { arg: Refs }
}

#[derive(Debug)]
enum Istructions {
    Assignment { index: i32, value: Refs, operator: Operators },
    Jump { to: String },
    Halt,
    Pass,
    Write { index: i32 },
    Read { index: i32 },
    If { a: Refs, condition: Conditions, b: Refs, jump: String, else_jump: Option<String> }
}

struct Istruction {
    istruction: Istructions,
    label: Option<String>
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

fn find_label(arg: &String, code: &Vec<Istruction>) -> i32 {
    let mut c = 0;
    let mut r = -1;
    for i in code {
        match i.label {
            Some(ref l) => if *l == *arg {
                            r = c + 1;
                            c = -1
                        } else {
                            c = c + 1;
                        },
            None => { c = c + 1 }
        };
        if c < 0 { break; };
    }
    r as i32
}

impl Istruction {
    fn execute(&self,pc: &i32,mem: &mut Vec<i32>,code: &Vec<Istruction>) -> i32 {
        println!("Executing: {:?}",self.istruction);
        match &self.istruction {
            &Istructions::Pass => pc + 1,
            &Istructions::Jump { ref to } => find_label(&to,&code),
            &Istructions::Write { index } => {
                println!("Output: {}",mem[index as usize]);
                return pc + 1;
            },
            &Istructions::Halt => -1,
            &Istructions::Assignment { ref index, ref value, ref operator } => {
                let a = deref(value,&mem);
                let r = match operator {
                    &Operators::Plus { ref arg } =>  a + deref(&arg,&mem),
                    &Operators::Minus { ref arg } => a - deref(&arg,&mem),
                    &Operators::Times { ref arg } => a * deref(&arg,&mem),
                    &Operators::DividedBy { ref arg } => a / deref(&arg,&mem),
                    &Operators::Modulo { ref arg } => a % deref(&arg,&mem),
                    &Operators::ElevatedTo { ref arg } => a.pow(deref(&arg,&mem) as u32),
                };
                let i = (*index) as usize;
                mem[i] = r;
                println!("Stored {} in {}",r,i);
                return pc + 1;
            },
            &Istructions::Read { index } => {
                let i = readInt();
                mem[index as usize] = i;
                println!("Stored {} in {}",i,index as usize);
                return pc + 1;
            },
            &Istructions::If { ref a, ref condition, ref b, ref jump, ref else_jump } => {
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
                        let r = find_label(&jump,&code);
                        println!("Condition true, jumping to {} ({})",jump,r);
                        r
                    },
                    false => match else_jump {
                        & Some(ref x) =>  {
                            let r = find_label(&x,&code);
                            println!("Condition false, jumping to {} ({})",x,r);
                            r
                        },
                        & None => pc + 1
                    }
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
            mem: Vec::<i32>::new() as Vec<i32>,
            code: Vec::<Istruction>::new() as Vec<Istruction>,
            pc: 0
        }
    }
    fn run(&mut self) {
        self.pc = 1;
        while self.pc >= 1 {
           if (self.pc as usize) > self.code.len() { break; }
           println!("Running instruction: {}",self.pc);
           self.pc = self.code[(self.pc - 1) as usize].execute(&self.pc,&mut self.mem, &self.code); 
        }
    }
    fn load_program(&mut self, program : &str) {
        let istrn_regex = Regex::new(r"^(\w+): ").unwrap();
        let halt_regex = Regex::new(r"^halt$").unwrap();
        let pass_regex = Regex::new(r"^pass$").unwrap();
        let goto_regex = Regex::new(r"^goto (\w+)$").unwrap();
        let read_regex = Regex::new(r"^read\((\d+)\)$").unwrap();
        let write_regex = Regex::new(r"^write\((\d+)\)$").unwrap();
        let assign_regex = Regex::new(r"^Mem\[(?P<t>\d+)\][ ]*:=[ ]*(?P<a>\d+|Mem\[\d+\])([ ]*(?P<o>\+|-|%|/|\^|\*)([ ]*(?P<b>\d+|Mem\[\d+\]))){0,1}$").unwrap();
        let if_regex = Regex::new(r"^if[ ]*(?P<a>\d+|Mem\[\d+\])[ ]*(?P<o>>|<|=){1}(?P<p>={0,1})[ ]*(?P<b>\d+|Mem\[\d+\])[ ]*then[ ]*goto[ ]*(?P<t>\w+)[ ]*(else[ ]*goto[ ]*(?P<e>\w+)){0,1}$").unwrap();

        let mem_regex = Regex::new(r"^Mem\[(\d+)\]$").unwrap();

        self.pc = 0;
        for line in program.lines() {
            self.pc = self.pc + 1;
            let l = line.trim().to_lowercase();
            println!("({}) Parsing Istruction: {}",self.pc,l);
            let label : Option<String> = match istrn_regex.is_match(&l) {
                true => match istrn_regex.captures(&l).unwrap().at(1) {
                    Some(x) => {
                        println!("({}) label \"{}\" has been set",self.pc,x);
                        Some(String::from(x))
                    },
                    None => None
                },
                false => None
            };
            let istr : &str = & istrn_regex.replace_all(line,"");
            println!("({}) Processing: {}",self.pc,istr);
            if halt_regex.is_match(&istr) { // HALT
                self.code.push(Istruction { label: label, istruction: Istructions::Halt });   
            } else if pass_regex.is_match(&istr) { // PASS
                self.code.push(Istruction { label: label, istruction: Istructions::Pass });
            } else if goto_regex.is_match(&istr) { // GOTO
                let to = String::from(goto_regex.captures(&istr).unwrap().at(1).unwrap());
                self.code.push(Istruction { label: label, istruction: Istructions::Jump { to: to } });
            } else if write_regex.is_match(&istr) { // WRITE
                let to = write_regex.captures(&istr).unwrap().at(1).unwrap().parse::<i32>().unwrap();
                self.code.push(Istruction { label: label, istruction: Istructions::Write { index: to } });
            } else if read_regex.is_match(&istr) { // READ
                let to = read_regex.captures(&istr).unwrap().at(1).unwrap().parse::<i32>().unwrap();
                self.code.push(Istruction { label: label, istruction: Istructions::Read { index: to } });
            } else if if_regex.is_match(&istr) { // IF
                let a_str = if_regex.replace_all(istr,"$a");
                let b_str = if_regex.replace_all(istr,"$b");
                let a = conv(&a_str,&mem_regex,&mut self.mem);
                let b = conv(&b_str,&mem_regex,&mut self.mem);
                let op_str : &str = & if_regex.replace_all(istr,"$o$p");
                let oper = get_operator(&op_str);
                let jmp = String::from(if_regex.replace_all(istr,"$t"));
                let else_txt : &str = & if_regex.replace_all(istr,"$e");
                let else_jmp = match else_txt {
                    "" => None,
                    x => Some(String::from(x))
                };
                self.code.push(Istruction { label: label, istruction: Istructions::If { a: a, b: b, condition: oper, jump: jmp, else_jump: else_jmp } });
            } else if assign_regex.is_match(&istr) { // ASSIGN
                let target = assign_regex.replace_all(istr,"$t").parse::<i32>().unwrap();
                while self.mem.len() <= (target as usize) {
                    self.mem.push(0);
                    println!("mem.len() is now {}",self.mem.len());
                }
                let a_str = assign_regex.replace_all(istr,"$a");
                let b_str = assign_regex.replace_all(istr,"$b");
                let a = conv(&a_str,&mem_regex,&mut self.mem);
                let op : &str = & assign_regex.replace_all(istr,"$o");
                let b = conv(&b_str,&mem_regex,&mut self.mem);
                let oper = match op {
                    "+" => Operators::Plus { arg: b },
                    "-" => Operators::Minus { arg: b },
                    "*" => Operators::Times { arg: b },
                    "/" => Operators::DividedBy { arg: b },
                    "^" => Operators::ElevatedTo { arg: b },
                    "%" => Operators::Modulo { arg: b },
                    ""  => Operators::Plus { arg: Refs::Literal { value: 0 } },
                     _  => panic!("Invalid operator")
                };
                self.code.push(Istruction { label: label, istruction: Istructions::Assignment { index: target, value: a, operator: oper } });
            } else { // UNKNOWN
                println!("({}) UNKNOWN ISTRUCTION: {}",self.pc,istr);
                break;
            }
        }
    }
}

fn main() {
    match env::args().nth(1) {
        Some(file_name) => {
            let mut f = File::open(file_name).ok().expect("Error opening file");
            let mut program = String::new();
            let mut vm = VM::new();
            f.read_to_string(&mut program);
            vm.load_program(&program);
            vm.run()
        },
        None => {
            println!("Usage: {} <filepath>",env::args().nth(0).unwrap())
        } 
    };
}
