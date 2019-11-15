use std::io;
use std::process::Command;

fn main() {

    let tag_icons: [&str; 9] = [
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_1.xbm)",
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_2.xbm)",
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_3.xbm)",
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_4.xbm)",
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_5.xbm)",
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_6.xbm)",
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_7.xbm)",
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_8.xbm)",
        "^i(/home/dexter/.scripts/bash_panel/icons/tag_9.xbm)"
    ];

    let base_bg = "^bg(#2c2020)";

    let active_fg = "^fg(#8cac63)";
    let active_bg = "^bg(#422f2f)";

    let inactive_fg = "^fg(#64848b)";
    let inactive_bg = &base_bg;

    let empty_fg = "^fg(#8b7d66)";
    let empty_bg = &base_bg;

    let active_1_fg = "^fg(#BB0000)";
    let active_1_bg = &base_bg;

    let active_2_fg = "^fg(#0000BB)";
    let active_2_bg = &base_bg;

    let fg_colours: [&str; 4] = [
        empty_fg, 
        inactive_fg, 
        active_2_fg, 
        active_1_fg
    ];
    let bg_colours: [&str; 4] = [
        empty_bg,
        inactive_bg,
        active_2_bg,
        active_1_bg
    ];

    let mut active_tag: u8 = 1;
    let mut prev_tag:u8 = 1;

    let mut tag_flags: [usize; 9] = [0; 9];
    let mut prev_tag_flags: [usize; 9] = [99;9];

    loop {
        // START LOOP
        let mut line = String::new();
        
        io::stdin().read_line(&mut line)
            .expect("failed to read line");
        let line: &str = line.trim();
        if line.starts_with("tag_"){//TAG CHANGED
            if line.starts_with("tag_changed") {
                active_tag = line[12..]
                    .chars()
                    .next()
                    .unwrap() 
                    as u8 -48;
                let active_tag_index: usize = active_tag as usize -1;
                let echo_arg = 
                    format!("{}{}{}  ", 
                        active_bg, 
                        active_fg, 
                        tag_icons[active_tag_index]
                    );
                let echo_path =
                    format!("> /tmp/bar_pipe_{}", active_tag);
                Command::new("/bin/bash")
                    .arg("-c")
                    .arg(format!("echo \"{}\" {}", echo_arg, echo_path))
                    .spawn()
                    .expect("oopsie woopsie uwu");    
                let prev_tag_index: usize = prev_tag as usize -1;
                let prev_tag_flag: usize = tag_flags[prev_tag_index];
                let echo_arg = 
                    format!("{}{}{}   ", fg_colours[prev_tag_flag], 
                            bg_colours[prev_tag_flag], 
                            tag_icons[prev_tag_index]);
                let echo_path =
                    format!("> /tmp/bar_pipe_{}", prev_tag);
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("echo \"{}\" {}", echo_arg, echo_path))
                    .spawn()
                    .expect("Failed to execute echo");
                prev_tag = active_tag;
            }
            //println!("[TAG_CHANGED]: tag changed to: {}", active_tag);
        //}




        //else if line.starts_with("tag_flags") { // TAG FLAGS
            let tag_status = Command::new("sh")
                .arg("-c")
                .arg("herbstclient tag_status 0")
                .output()
                .expect("Failed to execute `herbstclient tag_status 0`");
            let tag_status = String::from_utf8_lossy(&tag_status.stdout);
            let tag_status = tag_status.trim();

            for i in 0..9 {
                let status = tag_status[i*3..]
                    .chars()
                    .next()
                    .unwrap();
                match status {
                    '.' => tag_flags[i] = 0,
                    '-' => tag_flags[i] = 2,
                    '+' => tag_flags[i] = 3,
                    _ if status != '#' && status != '%' => tag_flags[i] = 1,
                    _ => { ; }
                }
                println!("{}", tag_flags[i]);
                if i+1 != active_tag as usize 
                    && tag_flags[i] != prev_tag_flags[i] {
                    let echo_arg = 
                        format!("{}{}{}   ", fg_colours[tag_flags[i]], 
                                bg_colours[tag_flags[i]], 
                                tag_icons[i]);
                    let echo_path =
                        format!("> /tmp/bar_pipe_{}", i+1);
                    Command::new("sh")
                        .arg("-c")
                        .arg(format!("echo \"{}\" {}", echo_arg, echo_path))
                        .spawn()
                        .expect("oopsie");
                    prev_tag_flags[i] = tag_flags[i];
                }
            }
        }
        // END LOOP
    }

}
