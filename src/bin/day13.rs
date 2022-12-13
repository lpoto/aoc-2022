use std::{cmp::Ordering, collections::LinkedList, fmt::Error, str::FromStr};

#[derive(Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            | (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            | (Packet::List(_), Packet::Int(r)) => {
                self.cmp(&Packet::List(vec![Packet::Int(*r)]))
            }
            | (Packet::Int(l), Packet::List(_)) => {
                Packet::List(vec![Packet::Int(*l)]).cmp(other)
            }
            | (Packet::List(l), Packet::List(r)) => {
                for (l, r) in l.iter().zip(r.iter()) {
                    match l.cmp(r) {
                        | Ordering::Equal => continue,
                        | o => return o,
                    }
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Packet, Error> {
        fn parse(s: &mut LinkedList<char>) -> Result<Packet, Error> {
            let mut packet = Packet::List(Vec::new());
            while s.len() > 0 {
                match s.pop_front().unwrap() {
                    | ',' => (),
                    | ']' => break,
                    | '[' => {
                        if let Packet::List(ref mut v) = packet {
                            match parse(s) {
                                | Ok(p) => v.push(p),
                                | Err(e) => return Err(e),
                            }
                        }
                    }
                    | a => {
                        let a = a.to_digit(10);
                        if let None = a {
                            return Err(Error::default());
                        }
                        let mut a = a.unwrap();
                        while s.front() != None
                            && s.front() != Some(&',')
                            && s.front() != Some(&']')
                        {
                            match s.pop_front().unwrap().to_digit(10) {
                                | Some(b) => a = a * 10 + b,
                                | None => return Err(Error::default()),
                            }
                        }
                        if let Packet::List(ref mut v) = packet {
                            v.push(Packet::Int(a));
                        }
                    }
                };
            }
            Ok(packet)
        }
        parse(&mut s.chars().collect())
    }
}

fn main() {
    let input = include_str!("../../input/day13.txt").trim();

    let mut packets: Vec<Packet> =
        vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];

    let valid_pairs_indices_sum =
        input.split("\n\n").enumerate().fold(0, |acc, (i, pair)| {
            let split: Vec<&str> = pair.lines().collect();
            for j in 0..2 {
                let err = format!("Failed to parse: {}", split[j]);
                packets.push(split[j].parse::<Packet>().expect(&err));
            }
            match packets[packets.len() - 2].cmp(&packets[packets.len() - 1]) {
                | Ordering::Less => acc + i + 1,
                | _ => acc,
            }
        });

    packets.sort();

    let decoder_key = packets.iter().enumerate().fold(1, |acc, (i, packet)| {
        if packet.eq(&"[[2]]".parse::<Packet>().unwrap())
            || packet.eq(&"[[6]]".parse::<Packet>().unwrap())
        {
            acc * (i + 1)
        } else {
            acc
        }
    });

    println!("P1: {}", valid_pairs_indices_sum);
    println!("P2: {}", decoder_key);
}
