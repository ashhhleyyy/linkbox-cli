mod client;
mod store;
mod model;

use structopt::StructOpt;
use crate::client::LinkboxClient;
use std::error::Error;
use crate::store::{load_from_store, save_to_store, remove_store};
use rustyline::{Editor, ColorMode};
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use rustyline::highlight::Highlighter;
use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use rustyline::config::Configurer;

#[derive(StructOpt, Debug)]
enum Opt {
    Login {
        server: String
    },
    Logout,
    List,
    Get {
        id: i32,
    },
    Create {
        url: String,
        note: String,
    },
    Delete {
        id: i32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();

    match opt {
        Opt::Login { server } => {
            if load_from_store()?.is_some() {
                eprintln!("You are already logged in!");
            } else if !LinkboxClient::is_valid_instance(&server).await? {
                eprintln!("Not a linkbox instance at: {}!", server);
            } else {
                let (username, password) = ask_username_and_password()?;
                let mut client = LinkboxClient::new(server);
                client.login(username, password).await?;
                save_to_store(&client)?;
                println!("You are now logged in!");
            };
        }
        Opt::Logout => {
            if load_from_store()?.is_some() {
                if ask("Are you sure you want to log out? [Yn] ")? {
                    remove_store()?;
                    println!("You are now logged out!");
                }
            } else {
                eprintln!("You are not logged in!");
            }
        }
        Opt::List => {
            if let Some(mut client) = load_from_store()? {
                let links = client.list_links().await?;
                for link in links {
                    println!("{}: {}\n\t{}", link.id, link.url, link.note);
                }
            } else {
                eprintln!("You are not logged in!");
            }
        }
        Opt::Get { id } => {
            if let Some(mut client) = load_from_store()? {
                let link = client.fetch_link(id).await?;
                if let Some(link) = link {
                    println!("{}: {}\n\t{}", link.id, link.url, link.note);
                } else {
                    eprintln!("Link with id {} not found!", id);
                }
            } else {
                eprintln!("You are not logged in!");
            }
        }
        Opt::Create { url, note } => {
            if let Some(mut client) = load_from_store()? {
                let id = client.create_link(url, note).await?;
                println!("Created link with id: {}!", id);
            } else {
                eprintln!("You are not logged in!");
            }
        }
        Opt::Delete { id } => {
            if let Some(mut client) = load_from_store()? {
                let link = client.fetch_link(id).await?;
                if let Some(link) = link {
                    println!("{}: {}\n\t{}", link.id, link.url, link.note);
                    if ask("\nAre you sure you want to delete this note? [Yn] ")? {
                        client.delete_link(id).await?;
                        println!("Deleted link with id: {}!", id);
                    }
                } else {
                    println!("No link with that id exists!")
                }
            } else {
                eprintln!("You are not logged in!");
            }
        }
    }

    Ok(())
}

fn ask(question: &str) -> rustyline::Result<bool> {
    let mut editor = Editor::<()>::new();
    let mut response = editor.readline(question)?;
    while response != "Y" && response != "n" {
        response = editor.readline(question)?;
    }

    Ok(response == "Y")
}

fn ask_username_and_password() -> rustyline::Result<(String, String)> {
    let mut editor = Editor::new();
    let h = MaskingHighlighter { masking: false };
    editor.set_helper(Some(h));

    // TODO: This is a hack, see https://github.com/kkawakam/rustyline/blob/master/examples/read_password.rs
    let username = editor.readline("Enter username: ")?;
    editor.helper_mut().unwrap().masking = true;
    editor.set_color_mode(ColorMode::Forced);
    editor.set_auto_add_history(false);
    let password = editor.readline("Enter password: ")?;

    Ok((username, password))
}

#[derive(Completer, Helper, Hinter, Validator)]
struct MaskingHighlighter {
    masking: bool,
}

impl Highlighter for MaskingHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        use unicode_width::UnicodeWidthStr;
        if self.masking {
            Owned("*".repeat(line.width()))
        } else {
            Borrowed(line)
        }
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        self.masking
    }
}
