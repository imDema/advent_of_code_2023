use std::collections::{HashSet, VecDeque};

use eyre::Result;

struct Card {
    nums: Vec<u32>,
    wins: HashSet<u32>,
}

impl Card {
    fn win_count(&self) -> usize {
        self.nums.iter().filter(|n| self.wins.contains(n)).count()
    }

    fn compute_score(&self) -> u32 {
        let cnt = self.win_count() as u32;
        if cnt > 0 {
            1 << (cnt - 1)
        } else {
            0
        }
    }
}

fn parse_card(s: &str) -> Result<Card> {
    let (_, s) = s.split_once(": ").expect("wrong format");
    let (win, num) = s.split_once(" | ").expect("wrong format");

    Ok(Card {
        wins: win.split_whitespace().map(|n| n.parse().unwrap()).collect(),
        nums: num.split_whitespace().map(|n| n.parse().unwrap()).collect(),
    })
}


fn main() -> Result<()> {
    color_eyre::install()?;
    let input = std::io::read_to_string(std::io::stdin())?;

    let r = input
        .lines()
        .map(|l| parse_card(l).unwrap())
        .map(|c| c.compute_score())
        .sum::<u32>();

    println!("{r}");

    let r = input
        .lines()
        .map(|l| parse_card(l).unwrap())
        .fold((0, VecDeque::new()), |(tot, mut queue), card| {
            let win_count = card.win_count();
            let curr = queue.pop_front().unwrap_or_default() + 1;
            if queue.len() < win_count {
                queue.resize(win_count, 0);
            }
            queue.iter_mut().take(win_count).for_each(|c| *c += curr);

            (tot + curr, queue)
        }).0;

    println!("{r}");

    Ok(())
}
