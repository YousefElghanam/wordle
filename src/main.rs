use	std::io::Read;
use	std::fs::File;
use	std::process::exit;
use	rand_set::RandSet;
use	colored::*;
use	rustyline::{DefaultEditor, Result, error::ReadlineError};

const WHITE: i8 = 0;
const GREEN: i8 = 1;
const YELLOW: i8 = 2;

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
					println!("{}{}{}{}", "WARNING".yellow(), ": ", word.red(), "in dictionary is not alphabetic. Skipping it");
				}
			}
		}
		Err(err) => {
			eprintln!("{}{}{}", "ERROR".red(), ": Couldn't open dictionary 'words.txt': ", err);
			exit(1);
		}
	}
	dict
}

fn	fill_ans_freq (answer: String, mut ans_freq: [i32; 26]) -> [i32; 26] {
	for c in answer.bytes() {
		if c.is_ascii_alphabetic() {
			ans_freq[(c.to_ascii_lowercase() - b'a') as usize] += 1;
		}
	}
	ans_freq
}

fn	print_colorized_word(answer: &String, mut ans_freq: [i32; 26], word: &(String, [i8; 5])) {
	let mut	cpy = word.clone();
	
	for (i, c) in cpy.0.char_indices() {
		if answer.as_bytes()[i] == c as u8 {
			cpy.1[i] = GREEN;
			ans_freq[((c as u8).to_ascii_lowercase() - b'a') as usize] -= 1;
		}
	}
	for (i, c) in cpy.0.char_indices() {
		if answer.contains(c) && ans_freq[((c as u8).to_ascii_lowercase() - b'a') as usize] > 0 {
			if cpy.1[i] == WHITE {
				cpy.1[i] = YELLOW;
				ans_freq[((c as u8).to_ascii_lowercase() - b'a') as usize] -= 1;
			}
		}
	}
	print!(" ");
	for (i, c) in cpy.0.char_indices() {
		print!(" ");
		if cpy.1[i] == GREEN {
			print!("{}", c.to_string().to_ascii_uppercase().green());
		}
		else if cpy.1[i] == YELLOW {
			print!("{}", c.to_string().to_ascii_uppercase().yellow());
		}
		else {
			print!("{}", c.to_string().to_ascii_uppercase());
		}
	}
	print!("\n");
}

fn	display_map(answer: String, ans_freq: [i32; 26], map: &Vec<String>) {
	let mut	word: (String, [i8; 5]) = (String::new(), [0; 5]);
	let mut	x = 1;

	for i in 0..6 {
		if map[i].is_empty() {
			if x == 1 {
				print!(">");
				x = 0
			}
			else {
				print!(" ");
			}
			println!(" _ _ _ _ _");
		}
		else {
			word.0 = map[i].to_string();
			print_colorized_word(&answer, ans_freq, &word);
		}
	}
}

fn  main() -> Result<()> {
	let mut	rl = DefaultEditor::new()?;
	let		dict: RandSet<String>;
	let		answer: Option<&String>;
	let mut	tries = 0;
	let mut	ans_freq = [0; 26];
	let mut	map = vec![String::new(); 6];

	dict = load_dict();
	answer = dict.get_rand();
	ans_freq = fill_ans_freq(answer.unwrap().to_string(), ans_freq);
	
	/* DEBUG */
	// print_ans_freq(ans_freq);

	println!("== Wordle ==");
	println!("Guess the word");

	/* DEBUG */
	// println!("looking for {}. shshshs :D", answer.unwrap().green());

	print!("\x1B[2J\x1B[1;1H");
	display_map(answer.unwrap().to_string(), ans_freq, &map);
	loop {
		let	readline = rl.readline(&format!("{}", "> ".green().bold()));

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
                let _ = rl.add_history_entry(word.trim());
				if dict.contains(&word.to_ascii_lowercase()) {
					map[tries] = word.to_ascii_lowercase().clone();
					print!("\x1B[2J\x1B[1;1H");
					display_map(answer.unwrap().to_string(), ans_freq, &map);
					if answer.unwrap() == &word.to_ascii_lowercase() {
						println!("YOU WIN");
						break Ok(());
					}
				}
				else {
					println!("{} '{}' {}", "Word".yellow(), word.red(), "is not in dictionary".yellow());
					continue ;
				}
				tries += 1;
				if tries == 6 {
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
				eprintln!("{} {}", "ERROR ".red(), err);
				break Ok(());
			}
		}
	}
}
