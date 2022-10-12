use colored::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Too few arguments!");

        std::process::exit(1);
    } else if args.len() > 3 {
        println!("Too many arguments!");

        std::process::exit(1);
    }

    let first_word = args[1].chars().collect::<Vec<_>>();
    let second_word = args[2].chars().collect::<Vec<_>>();

    if first_word.len() == second_word.len() {
        for chr_index in 0..first_word.len() {
            if first_word[chr_index] == second_word[chr_index] {
                print!("{}", first_word[chr_index].to_string().green());
            } else {
                print!("{}", first_word[chr_index].to_string().red());
            }
        }

        println!();

        for chr_index in 0..second_word.len() {
            if second_word[chr_index] == first_word[chr_index] {
                print!("{}", second_word[chr_index].to_string().green());
            } else {
                print!("{}", second_word[chr_index].to_string().red());
            }
        }
    } else if first_word.len() > second_word.len() {
        for chr_index in 0..second_word.len() {
            if first_word[chr_index] == second_word[chr_index] {
                print!("{}", first_word[chr_index].to_string().green());
            } else {
                print!("{}", first_word[chr_index].to_string().red());
            }
        }

        for chr_index in second_word.len()..first_word.len() {
            print!("{}", first_word[chr_index].to_string().red());
        }

        println!();

        for chr_index in 0..second_word.len() {
            if second_word[chr_index] == first_word[chr_index] {
                print!("{}", second_word[chr_index].to_string().green());
            } else {
                print!("{}", second_word[chr_index].to_string().red());
            }
        }
    } else {
        for chr_index in 0..first_word.len() {
            if first_word[chr_index] == second_word[chr_index] {
                print!("{}", first_word[chr_index].to_string().green());
            } else {
                print!("{}", first_word[chr_index].to_string().red());
            }
        }

        println!();

        for chr_index in 0..first_word.len() {
            if second_word[chr_index] == first_word[chr_index] {
                print!("{}", second_word[chr_index].to_string().green());
            } else {
                print!("{}", second_word[chr_index].to_string().red());
            }
        }

        for chr_index in first_word.len()..second_word.len() {
            print!("{}", second_word[chr_index].to_string().red());
        }
    }
}
