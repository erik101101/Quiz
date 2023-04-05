use std::collections::HashMap;
use std::io;

fn select_question(questions: &HashMap<i32, &str>) {
    for (idx, points) in questions.keys().enumerate() {
        let idx = idx + 1;
        println!("{idx}) {points}");
    }
    let keys: Vec<_> = questions.keys().collect();

    loop {
        let stdin = io::stdin();

        let mut s = String::new();
        stdin.read_line(&mut s).unwrap();

        let trim = s.trim();
        let number: Result<usize, _> = trim.parse();

        if let Ok(ok_number) = number {
            if ok_number == 0 {
                println!("0 ist keine valide Auswahl.");
                continue;
            }
            if let Some(key) = keys.get(ok_number - 1) {
                println!("{q}", q = questions.get(key).unwrap());
                break;
            } else {
                println!("FEHLER!!!! ALLES EXPLODIERT!!!!!!11!!!1!1!!!!11!")
            }
        } else {
            println!("Die Eingabe ist keine Zahl oder eine negative oder zu hohe Zahl.")
        }
    }
}

fn main() {
    let mut questions = HashMap::new();
    questions.insert(100, "Warum ist die Banane krumm?");
    questions.insert(200, "How much is the fish?");

    select_question(&questions);
}
