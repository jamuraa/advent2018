use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

struct MetadataTree {
    children: Vec<MetadataTree>,
    metadata: Vec<u32>,
}

impl MetadataTree {
    fn parse(nums: &mut Iterator<Item = u32>) -> MetadataTree {
        let mut children: Vec<MetadataTree> = Vec::new();
        let num_children = nums.next().unwrap();
        let num_metadata = nums.next().unwrap();
        for _i in 0..num_children {
            children.push(MetadataTree::parse(nums));
        }
        MetadataTree {
            children,
            metadata: nums.take(num_metadata as usize).collect(),
        }
    }

    fn all_metadata(&self) -> Vec<u32> {
        let mut all = Vec::new();
        for c in self.children.iter() {
            all.extend_from_slice(&c.all_metadata());
        }
        all.extend_from_slice(self.metadata.as_slice());
        all
    }

    fn value(&self) -> u32 {
        if self.children.len() == 0 {
            return self.metadata.iter().fold(0, |a, &v| a + v);
        }
        let mut val = 0;
        for idx in self.metadata.iter() {
            if idx != &0 {
                if let Some(child) = self.children.get((idx - 1) as usize) {
                    val += child.value();
                }
            }
        }
        val
    }
}

fn numbers_in_string(s: &str) -> Vec<u32> {
    s.split(|x| !char::is_numeric(x))
        .filter(|x| !x.is_empty())
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let nums = numbers_in_string(&lines.next().unwrap().unwrap());

    let tree = MetadataTree::parse(&mut nums.into_iter());

    let metadata: Vec<u32> = tree.all_metadata();

    println!(
        "Sum of all the metadata info is: {}",
        metadata.iter().fold(0, |a, &v| a + v)
    );
    println!("Value of the root node is: {}", tree.value());
    Ok(())
}
