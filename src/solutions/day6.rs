use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrbitTree(pub BTreeMap<String /* orbiter */, String /* orbitee */>);

use std::str::FromStr;

impl OrbitTree {
    pub fn new() -> Self {
        OrbitTree(BTreeMap::new())
    }

    pub fn add_connection(&mut self, connection: &Connection) {
        let Connection(center, with) = connection;
        self.0.insert(center.to_string(), with.to_string());
    }

    pub fn count_orbits(&self, object: &String) -> usize {
        if *object == "COM".to_owned() {
            return 0;
        }
        if let Some(v) = self.0.get(object) {
            1 + self.count_orbits(v)
        } else {
            0
        }
    }

    pub fn all_orbits(&self) -> usize {
        let mut set = BTreeSet::new();
        let mut acc = 0;

        for (k, v) in self.0.iter() {
            if !set.contains(k) {
                acc += self.count_orbits(&k);
                set.insert(k);
            }
            if !set.contains(v) {
                acc += self.count_orbits(&v);
                set.insert(v);
            }
        }
        acc
    }

    pub fn get_orbiters<'a>(&'a self, from: &'a String) -> Vec<&String> {
        self.0
            .iter()
            .filter(|(_, v)| *v == from)
            .map(|(k, _)| k)
            .collect::<Vec<&String>>()
    }

    pub fn path(&self, from: &String, to: &String) -> Option<usize> {
        let mut frontier: VecDeque<(&String, usize)> = VecDeque::new();
        let mut visited: BTreeSet<&String> = BTreeSet::new();

        frontier.push_back((from, 0));
        while let Some((object, dist)) = frontier.pop_front() {
            if
            /* could be quite inefficient */
            self.get_orbiters(object).contains(&to) {
                return Some(dist - 1);
            }

            if !visited.contains(object) {
                visited.insert(object);
                // insert orbiters
                self.get_orbiters(object)
                    .iter()
                    .for_each(|orb| frontier.push_back((orb, dist + 1)));

                if let Some(orbitee) = self.0.get(object) {
                    frontier.push_back((orbitee, dist + 1));
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Connection(String, String);

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.rsplit(")");

        let x: Option<Connection> = it.next().and_then(|fst| {
            it.next()
                .map(|snd| Connection(fst.to_owned(), snd.to_owned()))
        });
        match x {
            Some(r) => Ok(r),
            None => Err(format!("Can't parse Connection from {}", s)),
        }
    }
}

impl FromStr for OrbitTree {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let conns: Result<Vec<Connection>, String> = s.lines().map(|l| l.parse()).collect();
        let mut tree: OrbitTree = OrbitTree::new();

        for conn in conns? {
            tree.add_connection(&conn)
        }
        Ok(tree)
    }
}

use std::convert::TryInto;

pub fn part1(input: &OrbitTree) -> i32 {
    input.all_orbits().try_into().unwrap()
}

pub fn part2(input: &OrbitTree) -> i32 {
    if let Some(res) = input.path(&"YOU".to_owned(), &"SAN".to_owned()) {
        res.try_into().unwrap()
    } else {
        unreachable!()
    }
}

#[test]
pub fn part1_test() {
    let input: OrbitTree = super::get_input(6).unwrap();
    assert_eq!(part1(&input), 314702);
}

#[test]
pub fn part2_test() {
    let input: OrbitTree = super::get_input(6).unwrap();

    assert_eq!(part2(&input), 439);
}
