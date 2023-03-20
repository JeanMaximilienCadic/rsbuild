use std::env;
use std::io::{self, Write};
use std::process::{Command,Output};
use colored::Colorize;

fn read_output(command: &str)-> Output{
    return Command::new("sh")
    .arg("-c")
    .arg(command)
    .output()
    .expect("failed to execute process");
    
}
fn read_output_str(command: &str)-> String{
    let output = read_output(command);
    let mut output_str = String::from_utf8(output.stdout).unwrap();
    output_str.pop();
    return output_str;
}

fn exec(command: &str, print_command: bool) -> String{
    if print_command{
        println!("{} `{}`", "[rsbuild]".bold().yellow(), command);
    }
    let output = read_output(command);   
    // let mut s = String::from_utf8(output.stdout).unwrap();
    // io::stdout().write_all(s.as_bytes()).unwrap();
    // s.pop();
    // return s;
    let mut output_str = String::from_utf8(output.stdout).unwrap();
    let mut error_str = String::from_utf8(output.stderr).unwrap();
    output_str = output_str.replace("[output] ", "").replace("[rsbuild] ", "");
    error_str = error_str.replace("[error] ", "").replace("[rsbuild] ", "");
    let output_str2 = format!("{} {}", "[output]".bold().blue(), output_str);
    let error_str2 = format!("{} {}", "[error]".bold().red(), error_str);
    if output_str.len()>0 && print_command{
        io::stdout().write_all(output_str2.as_bytes()).unwrap();
    }
    if error_str.len()>0 && print_command{
        io::stderr().write_all(error_str2.as_bytes()).unwrap();
    }        
    return output_str2;
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

fn cargo_build_arg(arg: &str){
    let target_dir = format!("target/{}/{}", read_output_str("uname"), read_output_str("uname -m"));
    let cmd = format!("cargo build {} --target-dir {}", arg, target_dir);
    exec(&cmd, true);
}
fn cargo_build_release(){
    cargo_build_arg("--release");
}

fn cargo_build_debug(){
    cargo_build_arg("");

}

fn _exec_commands(commands: Vec<&str>){
    for command in &commands {
        exec(command, true);
    };
}
fn clean(){
    for command in &vec![
        "rm -r build",
        "rm -r $(find . -type d -iname *egg-info*)",
        "rm -r $(find . -type d -iname *pycache*)",
        "rm -r $(find . -type d -iname *.ipynb_checkpoints*)",
    ] {
        exec(command, false);
    };
}

fn build_wheel(){
    exec("mv dist/*.whl dist/legacy", false);
    _exec_commands(vec![
        "pip wheel . -w dist --no-deps",
        "rsbuild clean"
    ]);
}

fn build_docker_compose(v:&str){
    _exec_commands(vec![
        &format!("docker compose build {}",v)
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
fn cython(pkg: &str){
    let build_dir = format!("/tmp/bin/._rsbuild-{}", pkg);
    exec(&format!("rm -r {}", build_dir), false);
    exec(&format!("mkdir -p {}/dist/legacy", build_dir), false);
    exec(&format!("cp requirements.txt {}", build_dir), false);
    exec(&format!("cp setup.cfg {}", build_dir), false);
    exec(&format!("cp setup.py {}",build_dir), false);
    _exec_commands(vec![
        &format!("cythonize -a -i {}", pkg),
        "rsbuild clean",
        &format!("rm $(find ./{} -type f -iname *.c)", pkg),
        &format!("find {} -type f -iname *.so >so_files", pkg),
        &format!("rsync -av --files-from=so_files ./ {}", build_dir),
        &format!("rm $(find ./{} -type f -iname *.so)", pkg),
        &format!("cd {} && rsbuild build wheel", build_dir),
        "rsbuild clean",
        "rm so_files",
        ]);
    exec("rm $(find . -type f -iname *.html)", false);
    exec(&format!("mv {}/dist/*.whl dist/", build_dir), false);
    exec(&format!("rm -r {}", build_dir), false);
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
                _ =>{
                    exec(&arg0[..], true);
                }
            }
        }
        3=>{
            let arg0 = &args[1];
            let arg1 = &args[2];
            match &arg0[..] {
                "build" => {
                    match &arg1[..]{
                        "wheel" => build_wheel(),
                        "debug" => cargo_build_debug(),
                        "release" => cargo_build_release(),
                        _ => build_docker_compose(&arg1[..]),
                    }
                }               
                "pull" => {
                    match &arg1[..]{
                        "vanilla" => pull_vanilla(),
                        "sandbox" => pull_sandbox(),
                        _=>help()
                    }
                }
                "cython" => cython(&arg1[..]),
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