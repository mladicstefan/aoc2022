use std::collections::HashSet;

#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Item(u8);

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl TryFrom<u8> for Item {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> color_eyre::Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(color_eyre::eyre::eyre!(
                "{} is not a valid item",
                value as char
            )),
        }
    }
}

impl Item {
    fn score(self) -> usize {
        match self {
            // b'a' is 97 so smallest, next we do -b'a' and +1 for the lowest value to have 1
            // instead of 97
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 1 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let mut total_score = 0;

    for line in include_str!("day3.txt").lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let first_items: HashSet<Item> = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<_, _>>()?;

        let duplicate = second
            .bytes()
            .map(Item::try_from)
            .find_map(|item| {
                let item = item.ok()?;
                first_items.contains(&item).then_some(item)
            })
            .expect("eval failed");
        total_score += duplicate.score();
    }

    println!("{}", total_score);
    Ok(())
}
