#![warn(rust_2018_idioms)]

use crate::common::{ChessError, ChessResult};
use crate::positions::Position;
use crate::proto::chess::MoveCommand;
use clap::{App, Arg};
use protobuf::Message;
use regex::Regex;
use std::io;
use std::io::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

mod board;
mod common;
mod pieces;
mod positions;
mod proto;

#[tokio::main]
async fn main() -> ChessResult<()> {
    let cli_args = get_cli_args();

    let address = cli_args.value_of("address").unwrap_or("127.0.0.1:10001");
    let is_server = cli_args.is_present("server");

    print!("{}", termion::clear::All);

    let (my_color, mut connection) = if is_server {
        (common::Color::White, act_as_server(address).await?)
    } else {
        (common::Color::Black, act_as_client(address).await?)
    };

    let mut current_color = common::Color::White;

    print!("{}", termion::clear::All);
    let board = board::Board::new();

    let mut command = "".to_string();
    let mut message = "".to_string();

    let mut buf: Vec<u8> = Vec::new();
    buf.resize(1024, 0);

    loop {
        board.borrow().draw();
        print!("{}", termion::clear::AfterCursor);
        println!();

        match board.borrow().is_in_check_state() {
            None => println!(),
            Some(color) if color == my_color => println!("You are in check!"),
            Some(_) => println!("Your opponent is in check!"),
        }

        if current_color != my_color {
            println!("Waiting for another player");
            let count = connection.read(&mut buf).await?;
            println!("{}", count);
            let move_cmd = protobuf::parse_from_bytes::<MoveCommand>(&buf)?;
            let result =
                parse_other_player_command(&move_cmd).and_then(|(from, to)| -> ChessResult<()> {
                    board.borrow_mut().move_piece(&from, &to, current_color)?;
                    Ok(())
                });
            if let Err(err) = result {
                message = err.to_string();
            } else {
                current_color = !current_color;
            };
        } else {
            println!("{}{}", termion::color::Fg(termion::color::Red), message);
            print!(
                "{}Command({}): ",
                termion::color::Fg(termion::color::Reset),
                command
            );
            command.clear();
            message.clear();

            io::stdout().flush()?;
            io::stdin().read_line(&mut command)?;
            command.retain(|c| !c.is_whitespace());

            if command == "quit" {
                break;
            }

            let result =
                parse_command(&command).and_then(|(from, to)| -> ChessResult<MoveCommand> {
                    board.borrow_mut().move_piece(&from, &to, current_color)?;
                    let mut move_cmd = MoveCommand::default();
                    move_cmd.from.set_default().name = from.to_string();
                    move_cmd.to.set_default().name = to.to_string();
                    Ok(move_cmd)
                });
            match result {
                Ok(cmd) => {
                    cmd.write_to_vec(&mut buf)?;
                    connection
                        .write_all(&buf[0..(cmd.get_cached_size() as usize)])
                        .await?;
                    current_color = !current_color;
                }
                Err(err) => message = err.to_string(),
            };
        }
    }
    Ok(())
}

fn parse_other_player_command(command: &MoveCommand) -> ChessResult<(Position, Position)> {
    let from = Position::from_proto(command.get_from())
        .ok_or_else(|| ChessError::InvalidInput("Unreadable".to_string()))?;
    let to = Position::from_proto(command.get_to())
        .ok_or_else(|| ChessError::InvalidInput("Unreadable".to_string()))?;
    Ok((from, to))
}

fn parse_command(command: &str) -> ChessResult<(Position, Position)> {
    if command.len() != 5 {
        return Err(ChessError::InvalidInput(command.to_string()));
    }
    let middle = command.bytes().nth(2);
    if middle.is_none() || (middle.unwrap() != b' ' && middle.unwrap() != b'-') {
        return Err(ChessError::InvalidInput(command.to_string()));
    }

    let from: Position = command.chars().take(2).collect::<String>().parse()?;
    let to: Position = command
        .chars()
        .skip(3)
        .take(2)
        .collect::<String>()
        .parse()?;

    Ok((from, to))
}

fn is_valid_address(v: String) -> Result<(), String> {
    let port_re: Regex = Regex::new(r#":\d{1,5}$"#).unwrap();
    if port_re.is_match(&v) {
        Ok(())
    } else {
        Err(String::from(
            "Value should contain hostname (or IP address), ':' and port number after it",
        ))
    }
}

fn get_cli_args() -> clap::ArgMatches<'static> {
    App::new("rust-chess")
        .version("1.0")
        .author("Dennis Kormalev <kormalev.denis@gmail.com>")
        .about(r#""Give me some rust" personal project "#)
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .help("Run chess as server (playing white)"),
        )
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .takes_value(true)
                .validator(is_valid_address)
                .help("If acts as a server - address:port to bind to. If acts as a client - address:port of the server."),
        )
        .get_matches()
}

async fn act_as_server(address: &str) -> ChessResult<TcpStream> {
    println!(
        "{}{}Waiting for connection at {}",
        termion::cursor::Goto(1, 1),
        termion::color::Fg(termion::color::Yellow),
        address
    );
    let (stream, _) = TcpListener::bind(address).await?.accept().await?;
    print!(
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::AfterCursor
    );
    println!(
        "{}{}Connected to {} as server (you are playing whites)",
        termion::cursor::Goto(1, 1),
        termion::color::Fg(termion::color::Green),
        stream.peer_addr()?
    );
    Ok(stream)
}

async fn act_as_client(address: &str) -> ChessResult<TcpStream> {
    println!(
        "{}{}Connecting to {}",
        termion::cursor::Goto(1, 1),
        termion::color::Fg(termion::color::Yellow),
        address
    );
    let stream = TcpStream::connect(address).await?;
    print!(
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::AfterCursor
    );
    println!(
        "{}{}Connected to {} as client (you are playing blacks)",
        termion::cursor::Goto(1, 1),
        termion::color::Fg(termion::color::Green),
        stream.peer_addr()?
    );
    Ok(stream)
}
