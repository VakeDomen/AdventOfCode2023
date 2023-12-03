use input::get_input;
use crate::input::get_test_input;



mod input;

fn main() {
    let nums = vec![
        vec!['z', 'e', 'r', 'o'],
        vec!['o', 'n', 'e'],
        vec!['t', 'w', 'o'],
        vec!['t', 'h', 'r', 'e', 'e'],
        vec!['f', 'o', 'u', 'r'],
        vec!['f', 'i', 'v', 'e'],
        vec!['s', 'i', 'x'],
        vec!['s', 'e', 'v', 'e', 'n'],
        vec!['e', 'i', 'g', 'h', 't'],
        vec!['n', 'i', 'n', 'e']
    ];

    let mut sum = 0;
    let test = true;

    let input = if test {
        get_test_input()
    } else {
        get_input()
    };

    for row in input.lines().into_iter() {
        let mut counts: Vec<i8> = vec![
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut first = None;
        let mut second = None;
        
        for ch in row.trim().chars() {
            
            if ch.is_numeric() {
                second = Some(ch);
                if first == None {
                    first = Some(ch);
                }
                counts = vec![0,0,0,0,0,0,0,0,0,0];
            } else if ch.is_alphabetic() {
                for i in 0..10 {
                    // increment or restart count
                    let delta: i8 = if nums[i][counts[i] as usize] == ch {
                        1
                    } else {
                        (-1 * counts[i]) + ((ch == nums[i][0]) as i8)
                    };
                    counts[i] += delta;
                    
                    // check if full-word match
                    if counts[i] as usize == nums[i].len() {
                        counts[i] = 0;
                        second = char::from_digit(i as u32, 10);
                        if first == None {
                            first = second.clone()
                        }
                    }
                }    
            }
        }

        
        let row_result = first
                .unwrap()
                .to_digit(10)
                .unwrap() * 10 + 
            second
                .unwrap()
                .to_digit(10)
                .unwrap();
        println!("F: {:?} S: {:?} RESULT: {:?} | LINE: {} ", first, second, row_result, row);
        sum += row_result;
    }
    println!("Result: {}", sum);
}
