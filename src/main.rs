use std::env;
use std::io::{self, Write};
use std::process::{Command};
use colored::Colorize;


fn exec(command: &str, print_command: bool) -> String{
    if print_command{
        println!("{} `{}`", "[rsbuild]".bold().yellow(), command);
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    // let mut s = String::from_utf8(output.stdout).unwrap();
    // io::stdout().write_all(s.as_bytes()).unwrap();
    // s.pop();
    // return s;
    let mut output_str = String::from_utf8(output.stdout).unwrap();
    let mut error_str = String::from_utf8(output.stderr).unwrap();
    output_str = output_str.replace("[output] ", "").replace("[rsbuild] ", "");
    error_str = error_str.replace("[error] ", "").replace("[rsbuild] ", "");
    io::stdout().write_all(format!("{} {}", "[output]".bold().blue(), output_str).as_bytes());
    io::stderr().write_all(format!("{} {}", "[error]".bold().red(), error_str).as_bytes());
    return output_str;
}

fn help() {
    let s = "Usage:  rsbuild COMMAND [OPTIONS]

A self-sufficient runtime to build projects.

COMMAND:
    clean         N/A
    build         N/A
    pull          N/A

OPTIONS:
    wheel         N/A
    vanilla       N/A
    sandbox       N/A
    release       N/A
    debug         N/A


Run 'rsbuild' for more information on a command.

To get more help with docker, check out our guides at https://docs.rsbuild.com/guides/
";
    io::stdout().write_all(s.as_bytes()).unwrap();
}

fn cargo_build_release(){
    let s = exec("uname", false);
    let s1 = exec("uname -m ", false);
    let target_dir = format!("target/{}/{}", s, s1);
    let cmd = format!("cargo build --release --target-dir {}", target_dir);
    println!("{}", exec(&cmd, true));
}

fn cargo_build_debug(){
    let s = exec("uname", false);
    let s1 = exec("uname -m ", false);
    let target_dir = format!("target/{}/{}", s, s1);
    let cmd = format!("cargo build --debug --target-dir {}", target_dir);
    println!("{}", exec(&cmd, true));
}

fn _exec_commands(commands: Vec<&str>){
    for command in &commands {
        exec(command, true);
    };
}
fn clean(){
    _exec_commands(vec![
        "rm -r build",
        "rm -r $(find . -type d -iname *egg-info*)",
        "rm -r $(find . -type d -iname *pycache*)",
        "rm -r $(find . -type d -iname *.ipynb_checkpoints*)",
    ]);
}

fn build_wheel(){
    _exec_commands(vec![
        "mv dist/*.whl dist/legacy",
        "pip wheel . -w dist --no-deps",
        "rsbuild clean"
    ]);
}

fn build_vanilla(){
    _exec_commands(vec![
        "docker compose build vanilla",
    ]);
}

fn build_sandbox(){
    _exec_commands(vec![
        "docker compose build sandbox",
    ]);
}
fn build(){
    _exec_commands(vec![
        "rsbuild build wheel",
        "rsbuild build vanilla",
        "rsbuild build sandbox"
    ]);
}

fn pull(){
    _exec_commands(vec![
        "rsbuild pull vanilla",
        "rsbuild pull sandbox",
    ]);
}
fn pull_vanilla(){
    _exec_commands(vec!["docker compose pull vanilla"]);
}
fn pull_sandbox(){
    _exec_commands(vec!["docker compose pull sandbox"]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 =>{
            let arg0 = &args[1];
            match &arg0[..] {
                "help" => help(),
                "glances" => println!("{}", exec("glances", true)),
                "build" => build(),
                "pull" => pull(),
                "clean" => clean(),
                _ =>help()
            }
        }
        3=>{
            let arg0 = &args[1];
            let arg1 = &args[2];
            match &arg0[..] {
                "build" => {
                    match &arg1[..]{
                        "wheel" => build_wheel(),
                        "vanilla" => build_vanilla(),
                        "sandbox" => build_sandbox(),
                        "debug" => cargo_build_debug(),
                        "release" => cargo_build_release(),
                        _=>help()
                    }
                }               
                "pull" => {
                    match &arg1[..]{
                        "vanilla" => pull_vanilla(),
                        "sandbox" => pull_sandbox(),
                        _=>help()
                    }
                }
                _ =>help()
            }
        }
        4=>{
            let arg0 = &args[1];
            let arg1 = &args[2];
            let arg2 = &args[3];
            match &arg0[..] {
                "build" => {
                    match &arg1[..]{
                        "cargo" => {
                            match &arg2[..]{
                                "debug" => cargo_build_debug(),
                                "release" => cargo_build_release(),
                                _=>help()
                            }
                        }
                        _=>help()
                    }
                }     
                _=>help()
            }
        }
        _=>help()
    }
}