slint::include_modules!();

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::UdpSocket;

const GUI_LISTEN_ADDR: &str = "0.0.0.0:5679";
const CORE_TARGET_ADDR: &str = "192.168.8.235:5678";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum IncomingMessage {
    /// TTS text: {"type": "tts", "session_id": "xxx", "state": "sentence_start"|"sentence_end", "text": "..."}
    Tts {
        #[serde(rename = "type")]
        msg_type: String,
        #[allow(dead_code)]
        session_id: String,
        state: String,
        text: String,
    },
    /// Activation code: {"type": "activation", "code": "123456"}
    Activation {
        #[serde(rename = "type")]
        msg_type: String,
        code: String,
    },
    /// Toast notification: {"type": "toast", "text": "设备已激活"}
    Toast {
        #[serde(rename = "type")]
        msg_type: String,
        text: String,
    },
    /// Device state change: {"state": 3|4|5|6}
    State {
        state: i32,
    },
    /// Catch-all for any unrecognized messages
    Unknown(serde_json::Value),
}

#[derive(Serialize)]
struct OutgoingMessage {
    #[serde(rename = "type")]
    msg_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = MainWindow::new()?;

    // Create a weak handle for background threads to safely update the UI
    let ui_handle = ui.as_weak();

    // Prepare for UDP send (to core)
    let send_socket = UdpSocket::bind("0.0.0.0:0").await?;
    let send_socket = Arc::new(send_socket);

    // Set up UI callbacks
    let send_socket_clone = send_socket.clone();
    ui.on_interrupt_clicked(move || {
        let socket = send_socket_clone.clone();
        tokio::spawn(async move {
            let msg = OutgoingMessage {
                msg_type: "abort".to_string(),
            };
            if let Ok(json) = serde_json::to_string(&msg) {
                let _ = socket.send_to(json.as_bytes(), CORE_TARGET_ADDR).await;
            }
        });
    });

    // Background task to listen to UDP packets from core
    let recv_socket = UdpSocket::bind(GUI_LISTEN_ADDR).await?;
    let mut buf = [0u8; 4096];

    tokio::spawn(async move {
        loop {
            if let Ok((len, _addr)) = recv_socket.recv_from(&mut buf).await {
                if let Ok(json_str) = std::str::from_utf8(&buf[..len]) {
                    if let Ok(msg) = serde_json::from_str::<IncomingMessage>(json_str) {
                        let ui_handle_clone = ui_handle.clone();
                        slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_handle_clone.upgrade() {
                                match msg {
                                    IncomingMessage::State { state } => {
                                        println!("[State] {}", state);
                                        ui.set_device_state(state);
                                    }
                                    IncomingMessage::Activation { code, .. } => {
                                        println!("[Activation] code={}", code);
                                        ui.set_activation_code(code.into());
                                    }
                                    IncomingMessage::Toast { text, .. } => {
                                        println!("[Toast] {}", text);
                                        ui.set_toast_text(text.into());
                                    }
                                    IncomingMessage::Tts { state, text, .. } => {
                                        println!("[TTS] state={}, text={}", state, text);
                                        if state == "sentence_start" {
                                            // New sentence begins — replace subtitle
                                            ui.set_subtitle_text(text.into());
                                        } else if state == "sentence_end" {
                                            // Sentence finished — could keep or clear
                                            // Keep showing the last full sentence
                                        }
                                    }
                                    IncomingMessage::Unknown(val) => {
                                        println!("[Unknown] {:?}", val);
                                    }
                                }
                            }
                        })
                        .unwrap();
                    }
                }
            }
        }
    });

    ui.run()?;
    Ok(())
}

