use std::io:: Read;
use std::fs::File;
use rand_set::RandSet;
use colored::*;
use rustyline::{DefaultEditor, Result, error::ReadlineError};

fn  load_dict() -> RandSet<String> {
	let mut	dict: RandSet<String> = RandSet::new();
	let mut	content = String::new();
	let     file;

	file = File::open("words.txt");
	match file {
		Ok(mut opened_file) => {
			opened_file
				.read_to_string(&mut content)
				.expect("Data from dictionary should be in UTF-8");
			for word in content.split("\n") {
				if word.chars().all(|c| c.is_alphabetic())
					&& word.to_string().len() == 5 {
						dict.insert(word.to_ascii_lowercase().to_string());
				}
				else {
					println!("WARNING: '{}' in dictionary is not alphabetic. Skipping it", word);
				}
			}
		}
		Err(err) => {
			println!("ERROR: {err} in words.txt");
		}
	}
	dict
}

fn	display_guesses(answer: String, guess: String) {
	// ADD LOGIC HERE
}

fn  main() -> Result<()> {
	let mut	rl = DefaultEditor::new()?;
	let		dict: RandSet<String>;
	let		answer;
	let mut	tries = 6;

	dict = load_dict();
	answer = dict.get_rand();
	println!("== Wordle ==");
	println!("Guess the word");
	println!("looking for {}. shshshs :D", answer.unwrap().green());

	// display_guesses(&answer, NULL);
	loop {
		let	readline = rl.readline(&format!("{}", ">> ".green().bold()));

		match readline {
			Ok(word) => {
				if word.is_empty() {
					continue;
				}
				if !word.to_string().chars().all(|c| c.is_alphabetic()) {
					println!("{}", "Word needs to be only alphabetic".yellow());
					continue;
				}
				if word.to_string().len() != 5 {
					println!("{}", "Word needs to be 5 characters long".yellow());
					continue;
				}
				if dict.contains(&word) {
					if answer.unwrap() == &word {
						println!("YOU WIN");
						break Ok(());
					}
				else {
					display_guesses(answer.unwrap().to_string(), word);
				}
				}
				else {
					println!("{} '{}' {}", "Word ".yellow(), word.red(), "is not in dictionary".yellow());
					continue ;
				}
				tries -= 1;
				if tries == 0 {
					println!("THE OPPOSITE OF WIN HAPPENED");
					break Ok(());
				}
			}
			Err(ReadlineError::Interrupted) => {
				eprintln!("SUDDEN DEATH!");
				break Ok(());
			}
			Err(ReadlineError::Eof) => {
				eprintln!("LOSERS QUIT -_-");
				break Ok(());
			}
			Err(err) => {
				eprintln!("{} {}", "Error ".red(), err);
				break Ok(());
			}
		}
	}
}
