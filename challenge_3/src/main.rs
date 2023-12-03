use std::collections::HashMap;

use crate::input::{get_test_input, get_input};
mod input;


type Schematic = Vec<Vec<char>>;
type SchematicNumber = (i32, SchematicNumberStart);
type SchematicNumberStart = (usize, usize);
type SchematicNumberBuilder = (String, usize, usize);


trait Builder {
    fn build(self) -> SchematicNumber; 
}

impl Builder for SchematicNumberBuilder {
    fn build(self) -> SchematicNumber {
        (self.0.parse().unwrap(), (self.1, self.2))
    }
}

trait Extractor {
    fn adj(&self, x: usize, y: usize)-> Option<Vec<SchematicNumber>>;
    fn get_el(&self, x: usize, y: usize)-> Option<char>;
    fn get_digit(&self, x: usize, y: usize)-> Option<u32>;
    fn get_number(&self, x: usize, y: usize)-> Option<SchematicNumber>;
}

impl Extractor for Schematic {
    fn adj(&self, x: usize, y: usize)-> Option<Vec<SchematicNumber>> {
        if self.len() < 1 || x >= self.len() {
            return None;
        }

        if y >= self[0].len() {
            return None;
        }

        let seleted = self[x][y];

        if !vec!['/', '@', '=', '#', '*', '%', '$', '&', '+', '-'].contains(&seleted) {
            return None;
        }
        
        Some(vec![
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ].into_iter()
                .filter_map(|(x, y)| self.get_number(x, y))
                .collect()
        )
    }


    fn get_el(&self, x: usize, y: usize) -> Option<char> {
        if self.len() - 1 < x {
            return None;
        }
        if self[x].len() - 1 < y {
            return None;
        }
        return Some(self[x][y]);
    }

    fn get_digit(&self, x: usize, y: usize) -> Option<u32> {
        if self.len() - 1 < x {
            return None;
        }
        if self[x].len() - 1 < y {
            return None;
        }
        self[x][y].to_digit(10)
    }

    fn get_number(&self, x: usize, y: usize)-> Option<SchematicNumber> {
        let base = self.get_digit(x, y);
        let mut number: SchematicNumberBuilder = match base {
            Some(c) => (String::from(char::from_digit(c, 10).unwrap()), x, y),
            None => return None,
        };
        
        let mut posty = y;
        let mut has_post_digit = true;
        while has_post_digit {
            
            if posty != y {
                match self.get_digit(x, posty) {
                    Some(d) => number.0.push(char::from_digit(d, 10).unwrap()),
                    None => has_post_digit = false,
                }
            }

            posty += 1;
        }

        let mut prey = y;
        let mut has_pre_digit = true;
        while has_pre_digit {

            if prey != y {
                match self.get_digit(x, prey) {
                    Some(d) => {
                        number.2 = prey;
                        number.0.insert(0, char::from_digit(d, 10).unwrap())
                    },
                    None => has_pre_digit = false,
                }
            }
            if prey > 0 {
                prey -= 1;
            } else {
                break;
            }
        }

        Some(number.build())

    }

    
}


fn main() {
    let test = false;

    let input = if test {
        get_test_input()
    } else {
        get_input()
    };

    let schematic: Schematic = input
        .lines()
        .into_iter()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect();

    
    // pt 1
    // let mut hm: HashMap<SchematicNumberStart, i32> = HashMap::new();
    let mut sum = 0;
    
    for i in 0..schematic.len() {
        for j in 0..schematic[i].len() {
            if let Some(element) = schematic.get_el(i, j) {
                if element != '*' {
                    continue;
                }
            }
            if let Some(mut dgs) = schematic.adj(i, j) {
                dgs.dedup();
                if dgs.len() != 2 {
                    continue;
                }
                //  pt 2
                sum += dgs[0].0 * dgs[1].0;


                // pt 1
                // for dg in dgs {
                //     if hm.contains_key(&dg.1) {
                //         continue;
                //     }
                //     sum += dg.0;
                //     hm.insert(dg.1, dg.0);
                // }
            }
        } 
    }

    // println!("HM: {:#?}", hm);
    
    println!("Result: {}", sum);
    
}
