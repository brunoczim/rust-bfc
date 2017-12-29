use std::io::{
    Write,
    Error,
    ErrorKind,
};
use front_end::{
    AstNode,
    Node,
    Location,
};
use utils::{
    HeadedList,
};
use super::{
    Arch,
    Format,
};
use std::vec::{
    IntoIter,
};
use std::process::{
    Command,
    Stdio,
};
use std::{
    fs,
};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X86Mode {
    Amd64,
    X86,
}

#[derive(Clone, Debug)]
struct Loop {
    ops: IntoIter<Node<AstNode>>,
    start: Vec<u8>,
    end: Vec<u8>,
}

impl X86Mode {

    pub fn label_for(loc: &Location) -> Vec<u8> {
        let mut label = <Vec<_> as From<&mut [_]>>::from(&mut b"_at_".clone());
        label.append(&mut loc.line.to_string().bytes().collect());
        label.append(&mut vec![b'_']);
        label.append(&mut loc.column.to_string().bytes().collect());
        return label;
    }

    pub fn gen_asm<T: Write>(
        &self,
        ast: Vec<Node<AstNode>>,
        out: &mut T
    ) -> Result<usize, Error> {
        let mut acc = 0;
        macro_rules! fetch_res {
            ($res:expr) => {match $res {
                Ok(n) => acc += n,
                Err(e) => return Err(e),
            }}
        }
        fetch_res!(out.write(b".text\n"));
        fetch_res!(out.write(b".globl _start\n"));
        fetch_res!(out.write(b"_start:\n"));
        match *self {
            X86Mode::Amd64 => {
                fetch_res!(out.write(b"  pushq $0\n"));
                fetch_res!(out.write(b"  mov %rsp, %rbx\n"));
            },
            X86Mode::X86 => {
                fetch_res!(out.write(b"  pushl $0\n"));
                fetch_res!(out.write(b"  mov %esp, %esi\n"));
            },
        }
        let mut loops = HeadedList::new(Loop {
            ops: ast.into_iter(),
            start: Vec::new(),
            end: Vec::new(),
        }, None);
        'outer: loop {
            let Node {val, loc} = loop {
                let done = match loops.val_mut().ops.next() {
                    Some(v) => break v,
                    _ => match loops.take() {
                        Some(lp) => lp,
                        _ => break 'outer,
                    }
                };
                fetch_res!(out.write(b"  "));
                fetch_res!(out.write(&done.end));
                fetch_res!(out.write(b":\n"));
                match *self {
                    X86Mode::Amd64 => fetch_res!(out.write(b"  cmpw $0, (%rbx)\n")),
                    X86Mode::X86 => fetch_res!(out.write(b"  cmpw $0, (%esi)\n")),
                }
                fetch_res!(out.write(b"  jne "));
                fetch_res!(out.write(&done.start));
                fetch_res!(out.write(b"\n"));
            };
            match val {
                AstNode::Increment(n) => {
                    fetch_res!(out.write(b"  addl $"));
                    fetch_res!(out.write(&n.to_string().bytes().collect::<Vec<_>>()));
                    match *self {
                        X86Mode::Amd64 => fetch_res!(out.write(b", (%rbx)\n")),
                        X86Mode::X86 => fetch_res!(out.write(b", (%esi)\n")),
                    }
                },
                AstNode::Decrement(n) => {
                    fetch_res!(out.write(b"  subl $"));
                    fetch_res!(out.write(&n.to_string().bytes().collect::<Vec<_>>()));
                    match *self {
                        X86Mode::Amd64 => fetch_res!(out.write(b", (%rbx)\n")),
                        X86Mode::X86 => fetch_res!(out.write(b", (%esi)\n")),
                    }
                },
                AstNode::Next(n) => {
                    let mut start_lbl = Self::label_for(&loc);
                    let mut end_lbl = start_lbl.clone();
                    start_lbl.append(&mut <Vec<_> as From<&mut [_]>>::from(&mut b"_check_esp_start".clone()));
                    end_lbl.append(&mut <Vec<_> as From<&mut [_]>>::from(&mut b"_check_esp_end".clone()));
                    fetch_res!(out.write(b"  sub $"));
                    fetch_res!(out.write(&(n * 2).to_string().bytes().collect::<Vec<_>>()));
                    match *self {
                        X86Mode::Amd64 => fetch_res!(out.write(b", %rbx\n")),
                        X86Mode::X86 => fetch_res!(out.write(b", %esi\n")),
                    }
                    fetch_res!(out.write(b"  jmp "));
                    fetch_res!(out.write(&end_lbl));
                    fetch_res!(out.write(b"\n  "));
                    fetch_res!(out.write(&start_lbl));
                    fetch_res!(out.write(b":\n"));
                    match *self {
                        X86Mode::Amd64 => fetch_res!(out.write(b"  pushq $0\n  ")),
                        X86Mode::X86 => fetch_res!(out.write(b"  pushl $0\n  ")),
                    }
                    fetch_res!(out.write(&end_lbl));
                    fetch_res!(out.write(b":\n"));
                    match *self {
                        X86Mode::Amd64 => fetch_res!(out.write(b"  cmp %rbx, %rsp\n")),
                        X86Mode::X86 => fetch_res!(out.write(b"  cmp %esi, %esp\n")),
                    }
                    fetch_res!(out.write(b"  jae "));
                    fetch_res!(out.write(&start_lbl));
                    fetch_res!(out.write(b"\n"));
                },
                AstNode::Previous(n) => {
                    fetch_res!(out.write(b"  add $"));
                    fetch_res!(out.write(&(n * 2).to_string().bytes().collect::<Vec<_>>()));
                    match *self {
                        X86Mode::Amd64 => fetch_res!(out.write(b", %rbx\n")),
                        X86Mode::X86 => fetch_res!(out.write(b", %esi\n")),
                    }
                },
                AstNode::PutChar() => {
                    match *self {
                        X86Mode::Amd64 => {
                            fetch_res!(out.write(b"  mov $1, %rax\n"));
                            fetch_res!(out.write(b"  mov $1, %rdi\n"));
                            fetch_res!(out.write(b"  mov %rbx, %rsi\n"));
                            fetch_res!(out.write(b"  mov $1, %rdx\n"));
                            fetch_res!(out.write(b"  syscall\n"));
                        },
                        X86Mode::X86 => {
                            fetch_res!(out.write(b"  mov $4, %eax\n"));
                            fetch_res!(out.write(b"  mov $1, %ebx\n"));
                            fetch_res!(out.write(b"  mov %esi, %ecx\n"));
                            fetch_res!(out.write(b"  mov $1, %edx\n"));
                            fetch_res!(out.write(b"  int $0x80\n"));
                        },
                    }
                },
                AstNode::GetChar() => {
                    match *self {
                        X86Mode::Amd64 => {
                            let mut end = Self::label_for(&loc);
                            end.append(&mut <Vec<_> as From<&mut [_]>>::from(&mut b"_getc_end".clone()));
                            fetch_res!(out.write(b"  movw $0, (%rbx)\n"));
                            fetch_res!(out.write(b"  mov $0, %rax\n"));
                            fetch_res!(out.write(b"  mov $0, %rdi\n"));
                            fetch_res!(out.write(b"  mov %rbx, %rsi\n"));
                            fetch_res!(out.write(b"  mov $1, %rdx\n"));
                            fetch_res!(out.write(b"  syscall\n"));
                            fetch_res!(out.write(b"  cmp $1, %rax\n"));
                            fetch_res!(out.write(b"  je "));
                            fetch_res!(out.write(&end));
                            fetch_res!(out.write(b"\n"));
                            fetch_res!(out.write(b"  movw $-1, (%rbx)\n  "));
                            fetch_res!(out.write(&end));
                            fetch_res!(out.write(b":\n"));
                        },
                        X86Mode::X86 => {
                            let mut end = Self::label_for(&loc);
                            end.append(&mut <Vec<_> as From<&mut [_]>>::from(&mut b"_getc_end".clone()));
                            fetch_res!(out.write(b"  movw $0, (%esi)\n"));
                            fetch_res!(out.write(b"  mov $3, %eax\n"));
                            fetch_res!(out.write(b"  mov $0, %ebx\n"));
                            fetch_res!(out.write(b"  mov %esi, %ecx\n"));
                            fetch_res!(out.write(b"  mov $1, %edx\n"));
                            fetch_res!(out.write(b"  int $0x80\n"));
                            fetch_res!(out.write(b"  cmp $1, %eax\n"));
                            fetch_res!(out.write(b"  je "));
                            fetch_res!(out.write(&end));
                            fetch_res!(out.write(b"\n"));
                            fetch_res!(out.write(b"  movw $-1, (%esi)\n  "));
                            fetch_res!(out.write(&end));
                            fetch_res!(out.write(b":\n"));
                        },
                    }
                },
                AstNode::Loop(lp) => {
                    let mut start = Self::label_for(&loc);
                    let mut end = start.clone();
                    start.append(&mut <Vec<_> as From<&mut [_]>>::from(&mut b"_loop_start".clone()));
                    end.append(&mut <Vec<_> as From<&mut [_]>>::from(&mut b"_loop_end".clone()));
                    fetch_res!(out.write(b"  jmp "));
                    fetch_res!(out.write(&end));
                    fetch_res!(out.write(b"\n  "));
                    fetch_res!(out.write(&start));
                    fetch_res!(out.write(b":\n"));
                    loops.receive(Loop {
                        ops: lp.into_iter(),
                        start,
                        end,
                    });
                },
            }
        }
        match *self {
            X86Mode::Amd64 => {
                fetch_res!(out.write(b"  mov $60, %rax\n"));
                fetch_res!(out.write(b"  mov $0, %rdi\n"));
                fetch_res!(out.write(b"  syscall\n"));
            },
            X86Mode::X86 => {
                fetch_res!(out.write(b"  mov $1, %eax\n"));
                fetch_res!(out.write(b"  mov $0, %ebx\n"));
                fetch_res!(out.write(b"  int $0x80\n"));
            },
        }
        Ok(acc)
    }

}

impl Arch for X86Mode {

    fn generate(
        &self,
        ast: Vec<Node<AstNode>>,
        format: Format,
        out: String
    ) -> Result<usize, Error> {
        match format {
            Format::Asm => {
                let mut file = match fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(out) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                self.gen_asm(ast, &mut file)
            },
            Format::Elf => {
                let obj_file = out.clone() + ".o";
                let mut asm = match *self {
                    X86Mode::Amd64 => match Command::new("as")
                        .arg("-o")
                        .arg(&obj_file)
                        .arg("--64")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn() {
                        Err(e) => return Err(e),
                        Ok(p) => p,
                    },
                    X86Mode::X86 => match Command::new("as")
                        .arg("-o")
                        .arg(&obj_file)
                        .arg("--32")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn() {
                        Err(e) => return Err(e),
                        Ok(p) => p,
                    },
                };
                let amount = {
                    let mut stdin = asm.stdin.take().unwrap();
                    match self.gen_asm(ast, &mut stdin) {
                        Ok(n) => n,
                        Err(e) => return Err(e),
                    }
                };
                match asm.wait() {
                    Ok(status) => if !status.success() {
                        return Err(Error::new(
                            ErrorKind::Other,
                            match status.code() {
                                Some(code) => format!("Assembler returned status {}.", code),
                                _ => String::from("Assembler exited abnormally."),
                            }
                        ))
                    },
                    Err(e) => return Err(e),
                }
                let mut ld = match *self {
                    X86Mode::Amd64 => match Command::new("ld")
                        .arg(obj_file)
                        .arg("-o")
                        .arg(out)
                        .arg("-m")
                        .arg("elf_x86_64")
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn() {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    },
                    X86Mode::X86 => match Command::new("ld")
                        .arg(obj_file)
                        .arg("-o")
                        .arg(out)
                        .arg("-m")
                        .arg("elf_i386")
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn() {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    },
                };
                match ld.wait() {
                    Ok(status) => if status.success() {
                        Ok(amount)
                    } else {
                        Err(Error::new(
                            ErrorKind::Other,
                            match status.code() {
                                Some(code) => format!("Linker returned status {}.", code),
                                _ => String::from("Linker exited abnormally."),
                            }
                        ))
                    },
                    Err(e) => Err(e),
                }
            },
        }
    }

}
