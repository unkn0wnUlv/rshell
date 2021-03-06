use std::io::{self,Write};
use std::str::FromStr;
mod utils;
mod commands; 


#[cfg(test)]
mod unittest_tokenize_command {
    use super::*;
    #[test]
    #[ignore]
    fn empty_command() {
        assert_eq!("", tokenize_command("".to_string()).keyword);
    }

    #[test]
    fn test_keyword() {
      assert_eq!("test", tokenize_command("test".to_string()).keyword)
    }

    #[test]
    fn no_arg() {
      assert_eq!(0, tokenize_command("test".to_string()).args.len())
    }

    #[test]
    fn one_arg() {
      assert_eq!(1, tokenize_command("test one".to_string()).args.len())
    }

    #[test]
    fn multi_args() {
      assert_eq!(3, tokenize_command("test one two three".to_string()).args.len())
    }

    #[test]
    #[ignore]
    fn quotes() {
      assert_eq!(2, tokenize_command("test \"one two\" three".to_string()).args.len())
    }
}

// This is the structure of a command
struct Command {
    keyword: String,
    args: Vec<String>,
}

enum Builtin {
    Echo,
    Exit,
    Cat,
    Pwd,
    Rm,
    Ls,
    Mkdir,
    Clear,
    Help,
}

impl FromStr for Builtin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Builtin::Echo),
            "exit" => Ok(Builtin::Exit),
            "cat" => Ok(Builtin::Cat),
            "pwd" => Ok(Builtin::Pwd),
            "rm" => Ok(Builtin::Rm),
            "ls" => Ok(Builtin::Ls),
            "mkdir" => Ok(Builtin::Mkdir),
            "clear" => Ok(Builtin::Clear),
            "help" => Ok(Builtin::Help),
            _ => Err(()),
        }
    }
}

// Process the command, checking if it is an available built in
fn process_command(c : Command) -> i32 {
    match Builtin::from_str(&c.keyword) {
        Ok(Builtin::Echo) => commands::builtin_echo(&c.args),
        Ok(Builtin::Exit) => commands::builtin_exit(),
        Ok(Builtin::Cat) => commands::builtin_cat(&c.args),
        Ok(Builtin::Pwd) => commands::builtin_pwd(),
        Ok(Builtin::Rm) => commands::builtin_rm(&c.args),
        Ok(Builtin::Ls) => commands::builtin_ls(&c.args),
        Ok(Builtin::Mkdir) => commands::builtin_mkdir(&c.args),
        Ok(Builtin::Help) => commands::builtin_help(),
        Ok(Builtin::Clear) => commands::builtin_clear(),
        _ => {
            println!("{}: command not yet implemented", &c.keyword);
            1
        },
    }
}

fn print_prompt() {
    let prompt_char = "-~>";
    print!("\n{0} ", prompt_char);
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut command = String::new();
    
    io::stdin().read_line(&mut command)
        .expect("Failed to read command");

    utils::debug(format!("{:?}", command));

    command
}

fn tokenize_command(c: String) -> Command {
    let mut command_split : Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();

    utils::debug(format!("Split input {:?}", command_split));

    let command = Command {
        keyword : command_split.remove(0),
        args : command_split,
    };
    command
}

fn main() {
    println!("~~~~~ WELCOME TO THE TINY RUST SHELL ~~~~~");
    loop {
        print_prompt();

        let command = read_command();

        let split_command: Command = tokenize_command(command);

        process_command(split_command);
    }
}
