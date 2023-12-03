use crate::input::{get_test_input, get_input};
mod input;

fn main() {
    let test = false;

    let input = if test {
        get_test_input()
    } else {
        get_input()
    };

    let mut sum = 0;

    let red_cap = 12;
    let green_cap = 13;
    let blue_cap = 14;

    for mut line in input.lines().into_iter() {
        line = line.trim();

        let game_tokens = line.split(":").collect::<Vec<&str>>();
        assert_eq!(game_tokens.len(), 2);

        let round_tokens = game_tokens[1]
            .split(";")
            .collect::<Vec<&str>>(); 

        let mut rgb = vec![0, 0, 0];

        for round in round_tokens.into_iter() {
            for cube_data in round.split(",").into_iter() {
                let value_key_tokens = cube_data
                    .trim()
                    .split(" ")
                    .collect::<Vec<&str>>();
                assert_eq!(value_key_tokens.len(), 2);
                let key_index = match value_key_tokens[1] {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!(""),
                };
                let value: i32 = value_key_tokens[0].parse().unwrap();
                if rgb[key_index] < value {
                    rgb[key_index] = value;
                }
            }
        }

        // // pt one
        // if rgb[0] > red_cap { continue; }
        // if rgb[1] > green_cap { continue; }
        // if rgb[2] > blue_cap { continue; }

        // let game_id: i32 = game_tokens[0]
        //     .split(" ")
        //     .nth(1)
        //     .unwrap()
        //     .parse()
        //     .unwrap();


        // pt 2

        sum += rgb[0] * rgb[1] * rgb[2];

    }

    println!("Result: {}", sum);
}
