use std::env;
use std::process::{Command, exit};
use std::thread::sleep;

fn check_packages(packages: &[&str]) {
    for package in packages {
        let output = Command::new("which")
            .arg(package)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            println!("{} is not installed. Would you like to install it? (y/n)", package);
            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer).expect("Failed to read line");
            if answer.trim() == "y" {
                run_command("sudo", &["apt", "install", package]);
                if *package == "privoxy" {
                    let command = "echo 'forward-socks5 / 127.0.0.1:9050 .' | sudo tee -a /etc/privoxy/config";
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .output()
                        .expect("Failed to execute command");

                    if !output.status.success() {
                        eprintln!("Error: Command '{}' failed with status code {}", command, output.status);
                        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                        exit(1);
                    }
                }
            }
        }
    }
}

fn run_command(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error: Command '{}' failed with status code {}", cmd, output.status);
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn start_stop_restart(action: &str) {
    run_command("sudo", &["service", "tor", action]);
    run_command("sudo", &["service", "privoxy", action]);

    match action {
        "start" => println!("\nPROXY STARTED"),
        "stop" => println!("\nPROXY STOPPED"),
        _ => (),
    }
}

fn shuffle() {
    run_command("sudo", &["service", "tor", "restart"]);
    println!("proxyshuffle successfully");
    println!("\n");
    sleep(std::time::Duration::from_secs(5));
    let ip = run_command("torify", &["curl", "icanhazip.com"]);
    println!("Your new IP: {}", ip);
    println!("\n");
}

fn check() {
    let ip = run_command("torify", &["curl", "icanhazip.com"]);
    println!("Your IP: {}", ip);
}

fn nav_dw_snav(action: &str, url: &str) {
    let cmd = match action {
        "nav" => "firefox",
        "dw" => "torify wget",
        _ => "torify w3m",
    };

    run_command(cmd, &[url]);
    println!("\n");
}

fn main() {
    println!(r#"
███████╗ █████╗ ███████╗██╗   ██╗██████╗ ██╗  ██╗
██╔════╝██╔══██╗██╔════╝╚██╗ ██╔╝██╔══██╗╚██╗██╔╝
█████╗  ███████║███████╗ ╚████╔╝ ██████╔╝ ╚███╔╝
██╔══╝  ██╔══██║╚════██║  ╚██╔╝  ██╔═══╝  ██╔██╗
███████╗██║  ██║███████║   ██║   ██║     ██╔╝ ██╗
╚══════╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝     ╚═╝  ╚═╝
                                                 "#);

    check_packages(&["tor", "privoxy"]);

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "start" | "stop" | "restart" => start_stop_restart(&args[1]),
            "shuffle" => shuffle(),
            "check" => check(),
            "nav" | "dw" | "snav" => {
                if args.len() > 2 {
                    nav_dw_snav(&args[1], &args[2]);
                }
            }
            _ => {
                println!("Commands:\nCOMMAND  -  DO\neasypx start - start tor & privoxy\neasypx stop - stop tor & privoxy\neasypx restart - restart tor & privoxy\neasypx shuffle - shuffle ip\neasypx check - check ip\neasypx nav LINKTOWEBSITE - firefox connect LINKTOWEBSITE\neasypx snav LINKTOWEBSITE - W3M (terminal browser) proxy connect to LINKTOWEBSITE\neasypx dw LINKTOARCHIVE - proxy fast & secure download");
            }
        }
    }
}