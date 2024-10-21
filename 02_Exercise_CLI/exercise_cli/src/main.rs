//          _          _ ______ _____ _____
//         (_)        | |  ____|  __ \_   _|
//    _ __  ___  _____| | |__  | |  | || |
//   | '_ \| \ \/ / _ \ |  __| | |  | || |
//   | |_) | |>  <  __/ | |____| |__| || |_
//   | .__/|_/_/\_\___|_|______|_____/_____|
//   | |
//   |_|

//  https://links.pixeledi.eu
//  Rust exercises CLI | 11.2024

use csv;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
struct KnowHow {
    cnt: i32,
    fail: i32,
    player: String,
}

impl KnowHow {
    fn cnt_up(&mut self) {
        self.cnt += 1;
    }
    fn fail_up(&mut self) {
        self.fail += 1;
    }
    fn display_results(&self) {
        println!(
            "Player: {} hat {} Fragen beantwortet und {} falsch beantwortet!",
            self.player, self.cnt, self.fail
        );
    }
}

struct Question {
    frage: String,
    antwort: String,
}

impl Question {
    fn new(record: &csv::StringRecord) -> Self {
        Self {
            frage: record.get(1).unwrap_or_default().to_string(),
            antwort: record.get(2).unwrap_or_default().to_string(),
        }
    }

    fn ask(&self, gamer: &mut KnowHow) {
        loop {
            println!("Frage: {}", self.frage);
            let answer = read_input("Antwort (j/n): ");

            if answer == self.antwort {
                gamer.cnt_up();
                break;
            } else {
                println!("Falsch! Bitte nochmal");
                gamer.fail_up();
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Sofortige Ausgabe
    io::stdin()
        .read_line(&mut input)
        .expect("Fehler beim Lesen der Eingabe");
    input.trim().to_lowercase() // Normalisiere die Eingabe
}

fn read_csv<P: AsRef<Path>>(filename: P, gamer: &mut KnowHow) {
    let file = File::open(filename).expect("Datei konnte nicht geöffnet werden");
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        match result {
            Ok(record) => {
                // Question
                // println!("{}",record.get(1).unwrap_or_default().to_string());
                let question = Question::new(&record);
                question.ask(gamer);
            }
            Err(e) => {
                println!("Fehler beim Lesen des Records: {}", e);
            }
        }
    }
}

fn main() {
    let mut gamer = KnowHow {
        cnt: 0,
        fail: 0,
        player: String::from("bot"),
    };
    // let filename = "rust_nur3fragen.csv";
    let filename = "rust_allefragen.csv";


    let name = read_input("Dein Name: ");

    gamer.player = name;
    gamer.display_results();

    read_csv(filename, &mut gamer);
    gamer.display_results();
}
