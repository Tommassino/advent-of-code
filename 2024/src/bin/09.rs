use std::fmt::{Display, Formatter};
advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DiskBlock {
    File(usize),
    Free,
}

#[derive(Debug, Copy, Clone)]
struct FileBlock {
    id: usize,
    start: usize,
    size: usize,
}

#[derive(Debug)]
struct Disk {
    data: Vec<DiskBlock>,
    files: Vec<FileBlock>,
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for block in &self.data {
            match block {
                DiskBlock::File(id) => write!(f, "{}", id)?,
                DiskBlock::Free => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut data = Vec::new();
        let mut files = Vec::new();
        for (idx, c) in value.chars().enumerate() {
            let block_size = c.to_digit(10).unwrap() as usize;
            let file = if idx % 2 == 0 {
                files.push(FileBlock {
                    id: idx / 2,
                    start: data.len(),
                    size: block_size,
                });
                DiskBlock::File(idx / 2)
            } else {
                DiskBlock::Free
            };
            for _ in 0..block_size {
                data.push(file);
            }
        }
        Disk { data, files }
    }
}

impl Disk {
    fn compact(&mut self) {
        let mut left_idx = 0;
        let mut right_idx = self.data.len() - 1;
        while left_idx < right_idx {
            if let DiskBlock::Free = self.data[left_idx] {
                while let DiskBlock::Free = self.data[right_idx] {
                    right_idx -= 1;
                }
                if left_idx >= right_idx {
                    break;
                }
                self.data.swap(left_idx, right_idx);
            }
            left_idx += 1;
        }
    }

    fn compact_blocks(&mut self) {
        let stack: Vec<FileBlock> = self.files.iter().cloned().rev().collect();
        for file in stack {
            // look behind for free space
            for r_idx in 0..(self.files.len() - 1) {
                let left_file = self.files[r_idx];
                let right_file = self.files[r_idx + 1];
                // don't move right
                if left_file.start >= file.start {
                    continue;
                }
                let free_space = right_file.start - left_file.start - left_file.size;
                if free_space >= file.size {
                    let idx = self.files.iter().position(|f| f.id == file.id).unwrap();
                    self.files.remove(idx);
                    self.files.insert(
                        r_idx + 1,
                        FileBlock {
                            id: file.id,
                            start: left_file.start + left_file.size,
                            size: file.size,
                        },
                    );
                    break;
                }
            }
        }
    }

    fn checksum(&self) -> u64 {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, block)| match block {
                DiskBlock::File(id) => Some((idx * id) as u64),
                DiskBlock::Free => None,
            })
            .sum()
    }

    fn block_checksum(&self) -> u64 {
        self.files
            .iter()
            .map(|file_block: &FileBlock| {
                // (start * id, ..., (start + size) * id)
                (file_block.start..(file_block.start + file_block.size))
                    .map(|idx| (idx * file_block.id) as u64)
                    .sum::<u64>()
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Disk::from(input);
    disk.compact();
    Some(disk.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = Disk::from(input);
    disk.compact_blocks();
    Some(disk.block_checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
