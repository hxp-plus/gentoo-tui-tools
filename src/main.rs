use std::io;
use std::process::{Command, Stdio};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::ListState;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;

fn main() {
    // Initialize the terminal backend
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create a list of menu items
    let menu_items = vec![
        ListItem::new("启动Shell"),
        ListItem::new("更新Portage仓库"),
        ListItem::new("更新所有软件包"),
        ListItem::new("更新内核"),
    ];

    // State for the selected menu item
    let mut selected_index = 0;

    // Main event loop
    loop {
        // Render the initial state of the TUI
        render_tui(&mut terminal, &menu_items, selected_index);

        let stdin = io::stdin();
        for evt in stdin.events() {
            // Handle user input
            match evt.unwrap() {
                Event::Key(key) => match key {
                    Key::Ctrl('c') => {
                        terminal.clear().unwrap();
                        return;
                    }
                    Key::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    Key::Down => {
                        if selected_index < menu_items.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    Key::Char('\n') => {
                        // Exit the TUI and run the command based on the selected menu item
                        terminal.clear().unwrap();
                        drop(terminal); // Drop the terminal before handling selection
                        handle_selection(selected_index);
                        // Reinitialize the terminal after handling selection
                        let stdout = io::stdout().into_raw_mode().unwrap();
                        let backend = TermionBackend::new(stdout);
                        terminal = Terminal::new(backend).unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }

            // Render the TUI after handling the event
            render_tui(&mut terminal, &menu_items, selected_index);
        }
    }
}

fn render_tui(
    terminal: &mut Terminal<TermionBackend<termion::raw::RawTerminal<io::Stdout>>>,
    menu_items: &Vec<ListItem>,
    selected_index: usize,
) {
    // Clear the terminal
    terminal.clear().unwrap();

    // Get the size of the terminal
    let size = terminal.size().unwrap();

    // Create a layout with a title and a menu
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    // Render the title and the menu
    terminal
        .draw(|f| {
            let menu = List::new(menu_items.clone())
                .block(
                    Block::default()
                        .title("请选择（按 Ctrl-C 退出）")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().bg(Color::LightBlue).fg(Color::Black))
                .highlight_symbol("> ");
            let mut list_state = ListState::default();
            list_state.select(Some(selected_index));
            f.render_stateful_widget(menu, chunks[0], &mut list_state);
        })
        .unwrap();
}

fn interactive_command(command: &mut Command) {
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    // command.exec();
    let status = command.status().expect("Failed to execute command");
    if status.success() {
        println!("Command executed successfully");
    } else {
        println!(
            "Command failed with exit code: {}",
            status.code().unwrap_or(-1)
        );
    }
    println!("Press any key to continue...");
    io::stdin().keys().next();
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

fn update_emerge_repo() {
    let mut command = Command::new("sudo");
    let update_command = r#"
       emerge --sync
    "#;
    command.arg("/bin/bash").arg("-c").arg(update_command);
    interactive_command(&mut command);
}

fn handle_selection(selected_index: usize) {
    match selected_index {
        0 => launch_shell(),
        1 => update_emerge_repo(),
        2 => update_all_pkgs(),
        3 => update_kernel(),
        _ => println!("Invalid selection"),
    }
}
