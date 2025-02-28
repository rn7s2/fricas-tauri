use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    thread,
};

struct FriCAS {
    _pid: u32,
    tx: Sender<String>,
    rx: Receiver<String>,
}

#[derive(Debug, serde::Serialize)]
struct Result {
    result: String,
    result_type: String,
}

#[tauri::command]
fn execute(command: &str, state: tauri::State<Mutex<FriCAS>>) -> Result {
    fn katex_render(input: &str) -> String {
        let opts = katex::Opts::builder().display_mode(true).build().unwrap();
        match katex::render_with_opts(input, &opts) {
            Ok(v) => v,
            Err(e) => match e {
                katex::Error::JsExecError(s) => s,
                _ => "".into(),
            },
        }
    }

    let state = state.lock().unwrap();
    state.tx.send(command.to_string()).unwrap();

    let raw = state.rx.recv().unwrap();
    if raw.contains("$$") {
        let parts: Vec<&str> = raw.split("$$").map(str::trim).collect();
        let result = "\\let\\sp=^\\let\\sb=_\\let\\leqno=\\;\n".to_owned()
            + &parts[1][0..parts[1].find("\\leqno").unwrap()];
        let result_type = parts[2].replace(" ->", "").trim().to_string();

        let html1 = katex_render(&result);
        Result {
            result: html1,
            result_type,
        }
    } else {
        Result {
            result: "nil".into(),
            result_type: "nil".into(),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let fricas = launch_fricas();
    tauri::Builder::default()
        .manage(Mutex::new(fricas))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn launch_fricas() -> FriCAS {
    // Start the external program
    let child = Command::new("fricas")
        .args(["-nosman", "-eval", ")set output tex on"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start the process");
    let pid = child.id(); // TODO: kill process when app crashes

    // Get handles to the child's stdin and stdout
    let mut child_stdin = child.stdin.expect("Failed to open stdin");
    let mut child_stdout = child.stdout.expect("Failed to open stdout");

    // Channels for communication
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();

    // Spawn a thread to handle reading from the child's stdout
    thread::spawn(move || {
        {
            let mut init = String::new();
            loop {
                let mut buf = [0u8; 1];
                child_stdout
                    .read_exact(&mut buf)
                    .expect("Failed to read from stdout");
                init.push(buf[0] as char);
                if init.ends_with("(1) -> ") {
                    break;
                }
            }
        }

        loop {
            let command: String = rx_in.recv().unwrap() + "\n";
            child_stdin.write_all(command.as_bytes()).unwrap();
            child_stdin.flush().unwrap();

            let mut output = String::new();
            loop {
                let mut buf = [0u8; 1];
                child_stdout
                    .read_exact(&mut buf)
                    .expect("Failed to read from stdout");
                output.push(buf[0] as char);
                if output.ends_with(") -> ") {
                    break;
                }
            }
            tx_out.send(output).unwrap();
        }
    });

    FriCAS {
        _pid: pid,
        tx: tx_in,
        rx: rx_out,
    }
}
