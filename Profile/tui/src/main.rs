use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
};
use std::io::{stdout, Result, Write};
use std::fs::{self, File};
use std::path::Path;
use regex::Regex;
use serde::{Serialize, Deserialize};

// User data structure for JSON serialization
#[derive(Serialize, Deserialize, Clone)]
struct User {
    username: String,
    password: String,
    email: String,
}

// Application state
#[derive(PartialEq, Clone, Copy)]
enum AppMode {
    Login,
    Register,
}

#[derive(PartialEq, Clone, Copy)]
enum InputField {
    Username,
    Password,
    Email,
}

struct App {
    mode: AppMode,
    username: String,
    password: String,
    email: String,
    current_input: InputField,
    error_message: Option<String>,
}

impl App {
    // Create a new app instance
    fn new() -> Self {
        // Ensure users.json exists
        if !Path::new("users.json").exists() {
            fs::write("users.json", "[]").expect("Unable to create users.json");
        }

        Self {
            mode: AppMode::Login,
            username: String::new(),
            password: String::new(),
            email: String::new(),
            current_input: InputField::Username,
            error_message: None,
        }
    }

    // Validate email using regex
    fn is_valid_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }

    // Check if username exists in JSON file
    fn username_exists(&self, username: &str) -> Result<bool> {
        let contents = fs::read_to_string("users.json")?;
        let users: Vec<User> = serde_json::from_str(&contents)?;
        Ok(users.iter().any(|u| u.username == username))
    }

    // Attempt to log in
    fn login(&mut self) -> Result<bool> {
        if self.username.is_empty() || self.password.is_empty() {
            self.error_message = Some("Username and password cannot be empty".to_string());
            return Ok(false);
        }

        // Read users from JSON file
        let contents = fs::read_to_string("users.json")?;
        let users: Vec<User> = serde_json::from_str(&contents)?;

        // Find user and validate password
        match users.iter().find(|u| u.username == self.username) {
            Some(user) if user.password == self.password => {
                self.error_message = Some("Login Successful!".to_string());
                Ok(true)
            },
            Some(_) => {
                self.error_message = Some("Invalid password".to_string());
                Ok(false)
            },
            None => {
                self.error_message = Some("Username not found. Please register.".to_string());
                Ok(false)
            }
        }
    }

    // Register a new user
    fn register(&mut self) -> Result<bool> {
        // Validate inputs
        if self.username.is_empty() || self.password.is_empty() || self.email.is_empty() {
            self.error_message = Some("All fields must be filled".to_string());
            return Ok(false);
        }

        // Validate email
        if !Self::is_valid_email(&self.email) {
            self.error_message = Some("Invalid email format".to_string());
            return Ok(false);
        }

        // Check if username already exists
        if self.username_exists(&self.username)? {
            self.error_message = Some("Username already exists".to_string());
            return Ok(false);
        }

        // Read existing users
        let mut contents = fs::read_to_string("users.json")?;
        let mut users: Vec<User> = serde_json::from_str(&contents)?;

        // Add new user
        users.push(User {
            username: self.username.clone(),
            password: self.password.clone(),
            email: self.email.clone(),
        });

        // Write updated users back to file
        let updated_json = serde_json::to_string_pretty(&users)?;
        fs::write("users.json", updated_json)?;

        self.error_message = Some("Registration Successful! Please log in.".to_string());
        self.mode = AppMode::Login;
        Ok(true)
    }
}

// Render the application UI
fn ui(app: &App, frame: &mut Frame) {
    // Create a layout with centered area
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Min(15),
            Constraint::Percentage(20),
        ])
        .split(frame.size());

    let login_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(layout[1]);

    // Title
    let title = match app.mode {
        AppMode::Login => "Login",
        AppMode::Register => "Register",
    };
    let title_block = Block::default()
        .title(title)
        .borders(Borders::ALL);
    let title_para = Paragraph::new(title)
        .block(title_block)
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(title_para, layout[0]);

    // Username input
    let username_style = if app.current_input == InputField::Username {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let username_block = Block::default()
        .title("Username")
        .borders(Borders::ALL)
        .border_style(username_style);
    let username_para = Paragraph::new(app.username.clone())
        .block(username_block)
        .style(Style::default());
    frame.render_widget(username_para, login_layout[0]);

    // Password input (masked)
    let password_style = if app.current_input == InputField::Password {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let password_block = Block::default()
        .title("Password")
        .borders(Borders::ALL)
        .border_style(password_style);
    let masked_password = "*".repeat(app.password.len());
    let password_para = Paragraph::new(masked_password)
        .block(password_block)
        .style(Style::default());
    frame.render_widget(password_para, login_layout[1]);

    // Email input (only for registration)
    if app.mode == AppMode::Register {
        let email_style = if app.current_input == InputField::Email {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let email_block = Block::default()
            .title("Email")
            .borders(Borders::ALL)
            .border_style(email_style);
        let email_para = Paragraph::new(app.email.clone())
            .block(email_block)
            .style(Style::default());
        frame.render_widget(email_para, login_layout[2]);
    }

    // Action buttons
    let action_text = match app.mode {
        AppMode::Login => "Press Enter to Login | Press 'Ctrl+r' to Register",
        AppMode::Register => "Press Enter to Register | Press 'Ctrl+l' to Login",
    };
    let action_block = Block::default()
        .title("Actions")
        .borders(Borders::ALL);
    let action_para = Paragraph::new(action_text)
        .block(action_block);
    frame.render_widget(action_para,
        match app.mode {
            AppMode::Login => login_layout[2],
            AppMode::Register => login_layout[3],
        }
    );

    // Error message
    if let Some(error) = &app.error_message {
        let error_block = Block::default()
            .title("Error/Info")
            .borders(Borders::ALL);
        let error_para = Paragraph::new(error.clone())
            .block(error_block)
            .style(Style::default().fg(Color::Red));
        frame.render_widget(error_para, layout[2]);
    }
}

// Handle user input
fn handle_input(app: &mut App, key: KeyCode, key_modifiers: KeyModifiers) -> Result<bool> {
    // Handle mode switching
    match (key, key_modifiers, app.mode) {
        (KeyCode::Char('r'), KeyModifiers::CONTROL, AppMode::Login) => {
            app.mode = AppMode::Register;
            app.current_input = InputField::Username;
            app.error_message = None;
            return Ok(false);
        },
        (KeyCode::Char('l'), KeyModifiers::CONTROL, AppMode::Register) => {
            app.mode = AppMode::Login;
            app.current_input = InputField::Username;
            app.error_message = None;
            return Ok(false);
        }
        _ => {}
    }

    // Handle tab to switch inputs
    match key {
        KeyCode::Tab => {
            app.current_input = match (app.mode, app.current_input) {
                (AppMode::Login, InputField::Username) => InputField::Password,
                (AppMode::Login, InputField::Password) => InputField::Username,
                (AppMode::Register, InputField::Username) => InputField::Password,
                (AppMode::Register, InputField::Password) => InputField::Email,
                (AppMode::Register, InputField::Email) => InputField::Username,
                (AppMode::Login, InputField::Email) => todo!(),

            };
            return Ok(false);
        },
        _ => {}
    }

    // Input handling based on current mode and input field
    match (key, app.mode, app.current_input) {
        // Character input
        (KeyCode::Char(c), AppMode::Login, InputField::Username) => {
            app.username.push(c);
        }
        (KeyCode::Char(c), AppMode::Login, InputField::Password) => {
            app.password.push(c);
        }
        (KeyCode::Char(c), AppMode::Register, InputField::Username) => {
            app.username.push(c);
        }
        (KeyCode::Char(c), AppMode::Register, InputField::Password) => {
            app.password.push(c);
        }
        (KeyCode::Char(c), AppMode::Register, InputField::Email) => {
            app.email.push(c);
        }

        // Backspace
        (KeyCode::Backspace, AppMode::Login, InputField::Username) => {
            app.username.pop();
        }
        (KeyCode::Backspace, AppMode::Login, InputField::Password) => {
            app.password.pop();
        }
        (KeyCode::Backspace, AppMode::Register, InputField::Username) => {
            app.username.pop();
        }
        (KeyCode::Backspace, AppMode::Register, InputField::Password) => {
            app.password.pop();
        }
        (KeyCode::Backspace, AppMode::Register, InputField::Email) => {
            app.email.pop();
        }

        // Login or Register
        (KeyCode::Enter, AppMode::Login, _) => {
            return Ok(app.login()?);
        }
        (KeyCode::Enter, AppMode::Register, _) => {
            return Ok(app.register()?);
        }

        _ => {}
    }

    Ok(false)
}

fn main() -> Result<()> {
    // Setup terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Application state
    let mut app = App::new();

    // Main event loop
    loop {
        // Render UI
        terminal.draw(|frame| ui(&app, frame))?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => break,
                    code => {
                        let should_exit = handle_input(&mut app, code, key.modifiers)?;
                        if should_exit {
                            break;
                        }
                    }
                }
            }
        }
    }

    // Cleanup terminal
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}