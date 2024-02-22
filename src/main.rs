use rand::Rng;
use std::io::{self, Write};
use colored::Colorize;

fn main() {
  let mut is_first: bool = true;
  let mut is_numbers: bool = true;
  let mut is_letters: bool = true;
  let mut is_chars: bool = true;
  let mut length: i16 = 8;

  loop {
    if is_first == true {
      is_first = false;
      println!("Hi, this is Password Generator made by {}", "MrBallonvas".bold().underline().blue());
    }
    println!("\n{} Type {} to configure generator", "1.".bold(), "\"1\"".green());
    println!("{} Type {} to generate a password", "2.".bold(), "\"2\"".green());
    println!("{} Type {} to exit\n", "3.".bold(), "\"exit\"".red());

    print!("Your selection: ");
    let _ = io::stdout().flush();

    let mut answer: String = String::new();
    io::stdin()
      .read_line(&mut answer)
      .expect("Failed to read line");
    let answer = answer.trim();

    match answer {
      "1" => {
          setup(&mut length, &mut  is_numbers, &mut  is_letters, &mut  is_chars);
      },
      "2" => {
          println!("{}: {}", "Password".bold(), gen(length, is_numbers, is_letters, is_chars).underline().bold().bright_blue().italic());
      },
      "exit" => {
        break;
      },
      _ => {
          println!("{}", "Invalid input, try again".red().bold().underline());
      }
  }
  }
}

fn gen(length: i16, is_numbers: bool, is_letters: bool, is_chars: bool) -> String {
  let mut password: String = String::new();
  let numbers: String = String::from("1234567890");
  let letters: String = String::from("abcdefghijklmnopqrstuvwxyzABCDIFGHIJKLMNOPQRSTUVWXYZ");
  let chars: String = String::from("!@#$%^&*()_+-=?");

  let mut i: i16 = 0;
  while i < length {
    let random = rand::thread_rng().gen_range(0..=2);
    
    match random {
      0 => {
        if is_numbers == false {
          continue;
        }
        i+=1;
        let random_index: usize = rand::thread_rng().gen_range(0..numbers.len());
        let random_element = numbers.chars().nth(random_index).unwrap();
        password.push(random_element)
      },
      1 => {
        if is_letters == false {
          continue;
        }
        i+=1;
        let random_index: usize = rand::thread_rng().gen_range(0..letters.len());
        let random_element = letters.chars().nth(random_index).unwrap();
        password.push(random_element)
      },
      2 => {
        if is_chars == false {
          continue;
        }
        i+=1;
        let random_index: usize = rand::thread_rng().gen_range(0..chars.len());
        let random_element = chars.chars().nth(random_index).unwrap();
        password.push(random_element)
      },
      i32::MIN..=-1 | 2..=i32::MAX => todo!()
    }
  }

  password
}

fn setup(length: &mut i16, is_numbers: &mut bool, is_letters: &mut bool, is_chars: &mut bool) {
  let mut is_first = true;

  loop {
    if is_first == true {
      is_first = false;
      println!("\nNow you can setup generator:");
    } else {
      println!("\n");
    }
  
    println!("1. to configure the length of password, type {}", "\"1\"".green());
    println!("2. to configure the password content, type {}", "\"2\"".green());
    println!("3. if you want to leave the settings, type {}\n", "\"exit\"".red());
  
    print!("Your selection: ");
    let _ = io::stdout().flush();
    let mut answer: String = String::new();
    io::stdin()
      .read_line(&mut answer)
      .expect("Failed to read line");
    let answer = answer.trim();
  
    match answer {
        "1" => {
          print!("input password length: ");
          let _ = io::stdout().flush();

          let mut _length = String::new();
          io::stdin()
            .read_line(&mut _length)
            .expect("Failed to read line");
          let _length: i16 = _length.trim().parse().expect("Invalid input");
          *length = _length;
        }, "2" => {
          print!("allow numbers [Y/n]: ");
          let _ = io::stdout().flush();
          let mut _is_numbers = String::new();
          io::stdin()
            .read_line(&mut _is_numbers)
            .expect("Failed to read line");

          if _is_numbers.trim().to_lowercase() == "y" {
            *is_numbers = true;
          } else if _is_numbers.trim().to_lowercase() == "n" {
            *is_numbers = false;
          } else {
            *is_numbers = true;
          }

          print!("allow letters [Y/n]: ");
          let _ = io::stdout().flush();
          let mut _is_letters = String::new();
          io::stdin()
            .read_line(&mut _is_letters)
            .expect("Failed to read line");

          if _is_letters.trim().to_lowercase() == "y" {
            *is_letters = true;
          } else if _is_letters.trim().to_lowercase() == "n" {
            *is_letters = false;
          } else {
            *is_letters = true;
          }

          print!("allow chars [Y/n]: ");
          let _ = io::stdout().flush();
          let mut _is_chars = String::new();
          io::stdin()
            .read_line(&mut _is_chars)
            .expect("Failed to read line");

          if _is_chars.trim().to_lowercase() == "y" {
            *is_chars = true;
          } else if _is_letters.trim().to_lowercase() == "n" {
            *is_chars = false;
          } else {
            *is_chars = true;
          }

          if _is_chars.trim().to_lowercase() == "n" 
          && _is_letters.trim().to_lowercase() == "n"
          && _is_numbers.trim().to_lowercase() == "n" {
            println!("\nAll parameters can`t be {}", "No".bold().red());
            println!("Using the default generation setting");
            *is_numbers = true;
            *is_letters = true;
            *is_chars = true;
          }
        },
        "exit" => {
          break
        },
        _ => println!("{}", "Invalid input, try again".red().bold().underline())
    }
  }
}