use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
advent_of_code::solution!(15);

struct InitializationSequence {
    parts: Vec<String>,
}

impl From<&str> for InitializationSequence {
    fn from(value: &str) -> Self {
        let parts = value.split(',').map(|x| x.trim().to_string()).collect();
        Self { parts }
    }
}

impl InitializationSequence {
    fn hash(value: &str) -> u32 {
        let mut hash: u32 = 0;
        value.chars().for_each(|c| {
            let ascii = c as u32;
            hash += ascii;
            hash *= 17;
            hash %= 256;
        });
        hash
    }

    fn checksum(&self) -> u32 {
        self.parts.iter().map(|value| Self::hash(value)).sum()
    }
}

struct Lens {
    label: String,
    focal_length: u32,
}

struct LensSlots {
    slots: Vec<Vec<Lens>>,
    lens_slots: HashMap<String, usize>,
}

impl LensSlots {
    fn new() -> Self {
        let slots = (0..256).map(|_| vec![]).collect_vec();
        Self {
            slots,
            lens_slots: HashMap::new(),
        }
    }

    fn initialize(&mut self, initialization_sequence: &InitializationSequence) {
        initialization_sequence.parts.iter().for_each(|value| {
            if value.contains('-') {
                let label = value.split('-').next().unwrap();
                self.remove_lens(label);
            } else {
                let (label, focal_length_str) = value.split('=').collect_tuple().unwrap();
                let focal_length = focal_length_str.parse::<u32>().unwrap();
                self.add_lens(label, focal_length);
            }
        });
    }

    fn remove_lens(&mut self, label: &str) {
        self.lens_slots.get(label).cloned().iter().for_each(|slot| {
            self.slots[*slot].retain(|lens| lens.label != label);
            self.lens_slots.remove(label);
        });
    }

    fn add_lens(&mut self, label: &str, focal_length: u32) {
        let hash = InitializationSequence::hash(label) as usize;
        if self.lens_slots.contains_key(label) {
            let lens = self.slots[hash]
                .iter_mut()
                .find(|lens| lens.label == label)
                .unwrap();
            lens.focal_length = focal_length;
        } else {
            let lens = Lens {
                label: label.to_string(),
                focal_length,
            };
            self.slots[hash].push(lens);
            self.lens_slots.insert(label.to_string(), hash);
        }
    }

    fn power(&self) -> u32 {
        self.slots
            .iter()
            .enumerate()
            .map(|(idx, lens)| {
                lens.iter()
                    .enumerate()
                    .map(|(slot, lens)| {
                        let box_number = idx + 1;
                        let slot_number = slot + 1;
                        (box_number as u32) * (slot_number as u32) * lens.focal_length
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

impl Display for LensSlots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, slot) in self.slots.iter().enumerate() {
            if slot.is_empty() {
                continue;
            }
            write!(f, "Box {}: ", i)?;
            for lens in slot {
                write!(f, "[{} {}]", lens.label, lens.focal_length)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let initialization_sequence = InitializationSequence::from(input);
    Some(initialization_sequence.checksum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let initialization_sequence = InitializationSequence::from(input);
    let mut lens_slots = LensSlots::new();
    lens_slots.initialize(&initialization_sequence);
    Some(lens_slots.power())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
