// Remember: either 0-index and do < 2020 or 1-index and do <= 2020
//
// Hash: number => last_spoken
use std::collections::HashMap;

pub fn day15a(inputs: &[String]) -> anyhow::Result<String> {
    let initial_numbers: Vec<usize> = inputs[0].split(",").map(|s| s.parse().unwrap()).collect();
    let answer = play_game(&initial_numbers, 2020);

    Ok(answer.to_string())
}

pub fn day15b(inputs: &[String]) -> anyhow::Result<String> {
    let initial_numbers: Vec<usize> = inputs[0].split(",").map(|s| s.parse().unwrap()).collect();
    let answer = play_game(&initial_numbers, 30000000);

    Ok(answer.to_string())
}

fn play_game(input: &Vec<usize>, number_of_turns: usize) -> usize {
    let mut number_last_played = HashMap::new();

    let mut turn = 1;
    // this will be overwritten with a real value while we're seeding the game state
    let mut last_number_spoken = 0;

    for i in input.iter() {
        last_number_spoken = *i;
        number_last_played.insert(*i, turn);
        turn += 1;
    }

    while turn <= number_of_turns {
        let last_played = number_last_played.get(&last_number_spoken);

        let next_number = match last_played {
            Some(played) => {
                turn - 1 - played
            },
            None =>  0
        };

        // if it's not there, next number is 0
        // if it is there, next number is (current turn - 1 - turn from map)

        // we update the hash this late as we need the old turn to still be
        // there for the calculation above to work
        number_last_played.insert(last_number_spoken, turn - 1);
        last_number_spoken = next_number;
        turn += 1;
    }

    last_number_spoken
}
