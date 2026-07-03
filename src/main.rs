use serde::{Serialize, Deserialize};
use std::io::{self, Write};
use reqwest::Client;
use colored::Colorize;
use terminal_size::{Height, Width, terminal_size};
use crossterm::{
    cursor,
    execute,
    queue,
    terminal::{Clear, ClearType},
};


#[derive(Debug, Serialize, Deserialize)]
struct Definition {
    definition: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meaning {
    part_of_speech: String,
    definitions: Vec<Definition>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WordDict {
    word: String,
    meanings: Vec<Meaning>, 
}

fn clear_screen() { 
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
    let _ = queue!(stdout, cursor::MoveTo(0, 0));
    let _ = stdout.flush();
}

fn get_window_width() -> u16 {
    let window_size = terminal_size();
    if let Some((Width(w), Height(_))) = window_size {
        return w;
    } else {
        return 0;
    }

}

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    // Check status before processing
    let response = response.error_for_status()?;
    let text = response.text().await?;
    Ok(text)
}

#[tokio::main]
async fn main() {

    let letters = vec!["A", "B", "C", "D", "E", "F", "J", "H", "I", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "X", "Y", "Z"];

    let mut score: i32 = 0;
    let mut guesses: i32 = 0;

    let mut line_separator = String::new();
    for _i in 0..get_window_width(){
        line_separator.push('─'); 
    }

    let title = r#"
▄▖▌     ▖  ▖     ▌  ▖  ▖▘▗ ▌ ▘  
▐ ▛▌█▌  ▌▞▖▌▛▌▛▘▛▌  ▌▞▖▌▌▜▘▛▌▌▛▌
▐ ▌▌▙▖  ▛ ▝▌▙▌▌ ▙▌  ▛ ▝▌▌▐▖▌▌▌▌▌
    "#;

    clear_screen();
    println!("{}", title);

    loop {
        let random_index = rand::random_range(..letters.len());
        let random_letter = letters[random_index];

        print!("\n{} \"{}\" ", "Type a word that begins with:".bold(), random_letter.bold());
        println!("{}", " 「type \"quit\" to quit」".italic());

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        if guess.trim() == "quit" {
            break;
        }

        clear_screen();

        if !guess.trim().starts_with(&random_letter.to_lowercase()) {
            println!("The initial letter was incorrect!");
            println!("{}", "You Lost!!".red().bold());
            
            println!("\n──────────────"); 
            println!(" Score: {}", score);
            println!(" Guesses: {}", guesses);
            println!("──────────────"); 
            break;
        }

        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", guess);

        match fetch_data(&url).await {
            Ok(data) => {
                println!("{}", "Right Guess!!".green().bold());
                println!("Your guess was: {}", guess);

                println!("{}", line_separator);
                println!("Word Definition:");

                let json: Vec<WordDict> = serde_json::from_str(&data).unwrap();
                println!("\n{}", json[0].word.bold());
                println!("{}", json[0].meanings[0].definitions[0].definition);

                println!("{}", line_separator);

                score += 10;

                println!("\n──────────────"); 
                println!(" Score: {}", score);
            }
            Err(_e) => { 
                println!("\nThe choosen word doesn't exist!");
                println!("{}", "You Lost!!".red().bold());
                println!("\n─────────────"); 
                println!(" Score: {}", score);
                println!(" Guesses: {}", guesses);
                println!("─────────────"); 
                break;
            }
        }

        guesses += 1;
        println!(" Guesses: {}", guesses);
        println!("──────────────"); 
    }
}
