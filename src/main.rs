use std::io;
use std::process::{Command, Stdio};

fn interactive_command(command: &mut Command) {
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let status = command.status().expect("Failed to execute command");
    if status.success() {
        println!("Command executed successfully");
    } else {
        println!(
            "Command failed with exit code: {}",
            status.code().unwrap_or(-1)
        );
    }
}

fn print_menu() {
    println!("请选择:");
    println!("1. 升级内核");
    println!("2. 升级所有软件");
    println!("3. 启动shell");
    println!("4. 退出");
}

fn handle_option(option: u32) {
    match option {
        1 => update_kernel(),
        2 => update_all_pkgs(),
        3 => launch_shell(),
        4 => {
            println!("退出");
            std::process::exit(0);
        }
        _ => println!("无效输入，请重新输入"),
    }
}

fn launch_shell() {
    let mut command = Command::new("/bin/bash");
    command.arg("-c").arg(
        "echo \"提示：按Ctrl-D退出\" && bash --rcfile <(cat ~/.bashrc; echo 'PS1=\"(shell) # \"')",
    );
    interactive_command(&mut command);
}

fn update_all_pkgs() {
    let mut command = Command::new("sudo");
    command.arg("emerge").arg("-avuND").arg("@world");
    interactive_command(&mut command);
}

fn update_kernel() {
    let mut command = Command::new("sudo");
    let update_command = r#"
       eselect kernel set $(eselect kernel list | tail -n -1 | awk -F'[][]' '{print $2}')
       genkernel all --install --bootloader=grub2 --hyperv --mountboot
    "#;
    command.arg("/bin/bash").arg("-c").arg(update_command);
    interactive_command(&mut command);
}

fn main() {
    loop {
        print_menu();
        println!("");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let option: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        handle_option(option);
    }
}
