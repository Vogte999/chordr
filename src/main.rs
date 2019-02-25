extern crate rand;
use rand::Rng;
use std::time::Duration;
use std::thread::sleep;
use std::io;

fn main() {

    println!("How many seconds do you want between the chords?");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let num = match trimmed.parse::<u32>() {
        Ok(i) => i,
        Err(..) => 1,
    };

    println!("Which level are you at?");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let lev = match trimmed.parse::<u32>() {
        Ok(i) => i,
        Err(..) => 1,
    };

    match lev {
        1 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_1());
            }
        },
        2 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_2());
            }
        },
        3 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_3());
            }
        },
        4 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_4());
            }
        },
        5 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_5());
            }
        },
        6 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_6());
            }
        },
        7 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_7());
            }
        },
        8 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_8());
            }
        },
        9 => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord());
            }
        },
        _ => {
            loop {
                sleep(Duration::new(num.clone() as u64, 0));
                println!("{}", generate_chord_1());
            }
        },
    }

}

fn generate_chord() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 28);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        3   => "Am".to_string(),
        4   => "Dm".to_string(),
        5   => "Em".to_string(),
        6   => "G".to_string(),
        7   => "C".to_string(),
        8   => "G7".to_string(),
        9   => "C7".to_string(),
        10  => "B7".to_string(),
        11  => "Fmaj7".to_string(),
        12  => "A minibarré".to_string(),
        13  => "A7".to_string(),
        14  => "D7".to_string(),
        15  => "E7".to_string(),
        16  => "F".to_string(),
        17  => "Asus4".to_string(),
        18  => "Asus2".to_string(),
        19  => "Dsus4".to_string(),
        20  => "Dsus2".to_string(),
        21  => "Esus4".to_string(),
        22  => "G (full)".to_string(),
        23  => "G (full5)".to_string(),
        24  => "D/F#".to_string(),
        25  => "G/B 1".to_string(),
        26  => "G/B 2".to_string(),
        27  => "C/G".to_string(),
        _   => "What the Fuck".to_string(),
    }
}
fn generate_chord_8() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 24);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        3   => "Am".to_string(),
        4   => "Dm".to_string(),
        5   => "Em".to_string(),
        6   => "G".to_string(),
        7   => "C".to_string(),
        8   => "G7".to_string(),
        9   => "C7".to_string(),
        10  => "B7".to_string(),
        11  => "Fmaj7".to_string(),
        12  => "A minibarré".to_string(),
        13  => "A7".to_string(),
        14  => "D7".to_string(),
        15  => "E7".to_string(),
        16  => "F".to_string(),
        17  => "Asus4".to_string(),
        18  => "Asus2".to_string(),
        19  => "Dsus4".to_string(),
        20  => "Dsus2".to_string(),
        21  => "Esus4".to_string(),
        22  => "G (full)".to_string(),
        23  => "G (full5)".to_string(),
        _   => "What the Fuck".to_string(),
    }
}
fn generate_chord_7() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 22);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        3   => "Am".to_string(),
        4   => "Dm".to_string(),
        5   => "Em".to_string(),
        6   => "G".to_string(),
        7   => "C".to_string(),
        8   => "G7".to_string(),
        9   => "C7".to_string(),
        10  => "B7".to_string(),
        11  => "Fmaj7".to_string(),
        12  => "A minibarré".to_string(),
        13  => "A7".to_string(),
        14  => "D7".to_string(),
        15  => "E7".to_string(),
        16  => "F".to_string(),
        17  => "Asus4".to_string(),
        18  => "Asus2".to_string(),
        19  => "Dsus4".to_string(),
        20  => "Dsus2".to_string(),
        21  => "Esus4".to_string(),
        _   => "What the Fuck".to_string(),
    }
}
fn generate_chord_6() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 17);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        3   => "Am".to_string(),
        4   => "Dm".to_string(),
        5   => "Em".to_string(),
        6   => "G".to_string(),
        7   => "C".to_string(),
        8   => "G7".to_string(),
        9   => "C7".to_string(),
        10  => "B7".to_string(),
        11  => "Fmaj7".to_string(),
        12  => "A minibarré".to_string(),
        13  => "A7".to_string(),
        14  => "D7".to_string(),
        15  => "E7".to_string(),
        16  => "F".to_string(),
        _   => "What the Fuck".to_string(),
    }
}
fn generate_chord_5() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 16);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        3   => "Am".to_string(),
        4   => "Dm".to_string(),
        5   => "Em".to_string(),
        6   => "G".to_string(),
        7   => "C".to_string(),
        8   => "G7".to_string(),
        9   => "C7".to_string(),
        10  => "B7".to_string(),
        11  => "Fmaj7".to_string(),
        12  => "A minibarré".to_string(),
        13  => "A7".to_string(),
        14  => "D7".to_string(),
        15  => "E7".to_string(),
        _   => "What the Fuck".to_string(),
    }
}
fn generate_chord_4() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 13);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        3   => "Am".to_string(),
        4   => "Dm".to_string(),
        5   => "Em".to_string(),
        6   => "G".to_string(),
        7   => "C".to_string(),
        8   => "G7".to_string(),
        9   => "C7".to_string(),
        10  => "B7".to_string(),
        11  => "Fmaj7".to_string(),
        12  => "A minibarré".to_string(),
        _   => "What the Fuck".to_string(),
    }
}
fn generate_chord_3() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 8);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        3   => "Am".to_string(),
        4   => "Dm".to_string(),
        5   => "Em".to_string(),
        6   => "G".to_string(),
        7   => "C".to_string(),
        _   => "What the Fuck".to_string(),
    }
}
fn generate_chord_2() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 6);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        3   => "Am".to_string(),
        4   => "Dm".to_string(),
        5   => "Em".to_string(),
        _   => "What the Fuck".to_string(),
    }
}
fn generate_chord_1() -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 3);

    match num {
        0   => "D".to_string(),
        1   => "A".to_string(),
        2   => "E".to_string(),
        _   => "What the Fuck".to_string(),
    }
}