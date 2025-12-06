use iced::{
    widget::{button, column, container, row, text, text_input},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use std::path::PathBuf;
use std::time::Duration;

pub fn main() -> iced::Result {
    MacroBot::run(Settings::default())
}

#[derive(Debug, Clone)]
enum AppState {
    Idle,
    Running,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Idle
    }
}

struct MacroBot {
    state: AppState,
    selected_script: Option<PathBuf>,
    status_message: String,
    run_count: u64,
    delay_seconds: String,
    delay_value: u64,
}

#[derive(Debug, Clone)]
enum Message {
    SelectScript,
    ScriptSelected(Option<PathBuf>),
    StartRunning,
    StopRunning,
    DelayChanged(String),
    ScriptCompleted,
}

impl Application for MacroBot {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                state: AppState::Idle,
                selected_script: None,
                status_message: String::from("Select a script to begin"),
                run_count: 0,
                delay_seconds: String::from("10"),
                delay_value: 10,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Macro Bot")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectScript => Command::perform(
                async {
                    rfd::AsyncFileDialog::new()
                        .set_title("Select a script to run")
                        .pick_file()
                        .await
                        .map(|f| f.path().to_path_buf())
                },
                Message::ScriptSelected,
            ),
            Message::ScriptSelected(path) => {
                if let Some(script_path) = path {
                    self.selected_script = Some(script_path.clone());
                    self.status_message = format!("Selected: {}", script_path.display());
                    self.run_count = 0;
                } else {
                    self.status_message = String::from("No script selected");
                }
                Command::none()
            }
            Message::StartRunning => {
                if let Some(script_path) = &self.selected_script {
                    self.state = AppState::Running;
                    self.run_count = 0;
                    self.status_message = String::from("Running...");

                    // Parse delay
                    self.delay_value = self.delay_seconds.parse().unwrap_or(10);

                    let script = script_path.clone();
                    let delay = self.delay_value;

                    // Start the script runner
                    Command::perform(async move { run_script_loop(script, delay).await }, |_| {
                        Message::ScriptCompleted
                    })
                } else {
                    Command::none()
                }
            }
            Message::StopRunning => {
                self.state = AppState::Idle;
                self.status_message = String::from("Stopped");
                Command::none()
            }
            Message::DelayChanged(value) => {
                self.delay_seconds = value;
                Command::none()
            }
            Message::ScriptCompleted => {
                self.run_count += 1;
                if matches!(self.state, AppState::Running) {
                    self.status_message = format!("Completed run #{}", self.run_count);

                    // Continue running
                    if let Some(script_path) = &self.selected_script {
                        let script = script_path.clone();
                        let delay = self.delay_value;

                        Command::perform(
                            async move {
                                tokio::time::sleep(Duration::from_secs(delay)).await;
                                run_script_once(script).await
                            },
                            |_| Message::ScriptCompleted,
                        )
                    } else {
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("🤖 MACRO BOT").size(32);

        let script_display = if let Some(script) = &self.selected_script {
            text(format!("Script: {}", script.display())).size(14)
        } else {
            text("Script: None").size(14)
        };

        let status_text = text(format!(
            "Status: {}",
            match self.state {
                AppState::Idle => "STOPPED",
                AppState::Running => "RUNNING",
            }
        ))
        .size(16);

        let run_count_text = if self.run_count > 0 {
            text(format!("Run Count: {}", self.run_count)).size(14)
        } else {
            text("").size(14)
        };

        let delay_input = row![
            text("Delay (seconds):").size(14),
            text_input("10", &self.delay_seconds)
                .on_input(Message::DelayChanged)
                .width(Length::Fixed(80.0))
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let message_text = text(&self.status_message).size(14);

        let select_button = button(text("Select Script").size(16))
            .on_press(Message::SelectScript)
            .padding(10);

        let action_button = match self.state {
            AppState::Idle => {
                if self.selected_script.is_some() {
                    button(text("▶ Start Running").size(16))
                        .on_press(Message::StartRunning)
                        .padding(10)
                } else {
                    button(text("▶ Start Running").size(16)).padding(10)
                }
            }
            AppState::Running => button(text("⏹ Stop").size(16))
                .on_press(Message::StopRunning)
                .padding(10),
        };

        let content = column![
            title,
            text("").size(10),
            script_display,
            text("").size(10),
            status_text,
            run_count_text,
            text("").size(10),
            delay_input,
            text("").size(10),
            message_text,
            text("").size(20),
            row![select_button, action_button].spacing(20),
        ]
        .spacing(10)
        .padding(30)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

async fn run_script_once(script_path: PathBuf) -> () {
    let _ = execute_script(&script_path).await;
}

async fn run_script_loop(script_path: PathBuf, delay: u64) -> () {
    loop {
        let _ = execute_script(&script_path).await;
        tokio::time::sleep(Duration::from_secs(delay)).await;
    }
}

async fn execute_script(script_path: &PathBuf) -> Result<(), String> {
    let extension = script_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let script_path_str = script_path.to_string_lossy().to_string();

    let (program, args) = match extension {
        "py" => {
            #[cfg(unix)]
            {
                ("python3".to_string(), vec![script_path_str])
            }
            #[cfg(windows)]
            {
                ("python".to_string(), vec![script_path_str])
            }
        }
        "sh" => {
            #[cfg(unix)]
            {
                ("sh".to_string(), vec![script_path_str])
            }
            #[cfg(windows)]
            {
                return Err("Shell scripts not supported on Windows".to_string());
            }
        }
        "bat" | "cmd" => {
            #[cfg(windows)]
            {
                ("cmd".to_string(), vec!["/C".to_string(), script_path_str])
            }
            #[cfg(unix)]
            {
                return Err("Batch files not supported on Unix".to_string());
            }
        }
        _ => (script_path_str, vec![]),
    };

    let result = tokio::process::Command::new(&program)
        .args(&args)
        .output()
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
