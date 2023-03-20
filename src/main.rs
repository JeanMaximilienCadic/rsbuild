use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::{thread, time};
use colored::Colorize;
use foreach::for_each;

fn exec(command: &str, print_command: bool) -> String {
    if print_command{
        println!("{} `{}`", "[rsbuild]".bold().yellow(), command);
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    let mut s = String::from_utf8(output.stdout).unwrap();
    // io::stdout().write_all(s.as_bytes()).unwrap();
    s.pop();
    return s;
}

fn help() {
    let s = "Usage:  zc [OPTIONS] COMMAND

A self-sufficient runtime for zakuro

Commands:
    restart         Restart the zakuro service
    *build           Build the image attached to the context (multi-platform)
    *build_vanilla   Build the vanilla image attached to the context (multi-platform)
    add_worker      Add a worker to the network
    down            Stop the container
    up              Start the container
    logs            Fetch the logs of master node
    pull            Pull the latest zakuro version
    test_network    Test the zakuro network
    =============
    *commit         Create a new image from a container's changes
    *cp             Copy files/folders between a container and the local filesystem
    *create         Create a new container
    *exec           Run a command in a running container
    *export         Export a container's filesystem as a tar archive
    *images         List images
    *info           Display system-wide information
    *kill           Kill one or more running containers
    *push           Push an image or a repository to a registry
    restart         Restart the zakuro service
    *rm             Remove one or more containers
    *start          Start one or more stopped containers
    *stop           Stop one or more running containers
    *version        Show the Docker version information

Run 'zc COMMAND --help' for more information on a command.

To get more help with docker, check out our guides at https://docs.zakuro.ai/go/guides/
";
    let _stemp ="
============Options:
        --config string      Location of client config files (default \"/Users/jcadic/.config/zakuro\")
    -c, --context string     Name of the context to use to connect to the daemon (overrides DOCKER_HOST env var and default context set with \"docker context use\")
    -D, --debug              Enable debug mode
    -H, --host list          Daemon socket(s) to connect to
    -v, --version            Print version information and quit

Management Commands:
=============
    *builder     Manage builds
    *config      Manage Docker configs
    *context     Manage contexts
    *network     Manage networks
    *node        Manage Swarm nodes
    *secret      Manage Docker secrets
    *service     Manage services
    *system      Manage Docker
    *trust       Manage trust on Docker images
    *volume      Manage volumes

";
    io::stdout().write_all(s.as_bytes());
}

fn cargo_build_release(){
    let s = exec("uname", false);
    let s1 = exec("uname -m ", false);
    let target_dir = format!("target/{}/{}", s, s1);
    let cmd = format!("cargo build --release --target-dir {}", target_dir);
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
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 =>{
            let arg0 = &args[1];
            match &arg0[..] {
                "cargo_release" => cargo_build_release(),
                "help" => help(),
                "build_wheel" => build_wheel(),
                "clean" => clean(),
                _ =>help()
            }
        }
        _ =>help()
    }
    println!("Exit...");
}