use aoc::*;
use bitvec::prelude::*;

const INPUT: &str = include_str!("../../input/16.txt");

type BitSlice = bitvec::slice::BitSlice<Msb0, u8>;
type BitVec = bitvec::vec::BitVec<Msb0, u8>;

fn decode(binary: &BitSlice) -> i64 {
    binary
        .iter()
        .by_val()
        .fold(0, |acc, bit| if bit { 2 * acc + 1 } else { 2 * acc })
}

#[derive(Debug)]
struct Packet {
    version: i64,
    type_id: i64,
    data: Data,
}

#[derive(Debug)]
enum Data {
    Literal(i64),
    Operator(Vec<Packet>),
}

impl Packet {
    fn parse(mut input: &BitSlice) -> (usize, Self) {
        let version = decode(&input[0..3]);
        let type_id = decode(&input[3..6]);
        input = &input[6..];
        let mut consumed = 0;
        let data = if type_id == 4 {
            let mut value = BitVec::new();
            for chunk in input.chunks(5) {
                consumed += 5;
                value.extend_from_bitslice(&chunk[1..]);
                if !chunk[0] {
                    break;
                }
            }
            Data::Literal(decode(&value))
        } else {
            let length_type = input[consumed];
            consumed += 1;
            if length_type {
                let sub_packets = decode(&input[consumed..(consumed + 11)]);
                consumed += 11;
                let mut packets = vec![];
                for _ in 0..sub_packets {
                    let (consumed_bytes, packet) = Packet::parse(&input[consumed..]);
                    consumed += consumed_bytes;
                    packets.push(packet);
                }
                Data::Operator(packets)
            } else {
                let total_len = decode(&input[consumed..(consumed + 15)]);
                consumed += 15;
                let mut packets = vec![];
                let mut partial_input = &input[consumed..(consumed + total_len as usize)];
                consumed += total_len as usize;
                while !partial_input.is_empty() {
                    let (c, packet) = Packet::parse(&partial_input);
                    partial_input = &partial_input[c..];
                    packets.push(packet);
                }
                Data::Operator(packets)
            }
        };
        (
            consumed + 6,
            Self {
                version,
                type_id,
                data,
            },
        )
    }

    fn version_sum(&self) -> i64 {
        let sum = match &self.data {
            Data::Literal(_) => 0,
            Data::Operator(packets) => packets.iter().map(Packet::version_sum).sum(),
        };
        sum + self.version
    }

    fn eval(&self) -> i64 {
        match &self.data {
            Data::Literal(n) => *n,
            Data::Operator(packets) => {
                let nums: Vec<i64> = packets.iter().map(Packet::eval).collect();
                match self.type_id {
                    0 => nums.into_iter().sum(),
                    1 => nums.into_iter().product(),
                    2 => nums.into_iter().min().unwrap(),
                    3 => nums.into_iter().max().unwrap(),
                    #[rustfmt::skip]
                    5 => if nums[0] > nums[1] { 1 } else { 0 },
                    #[rustfmt::skip]
                    6 => if nums[0] < nums[1] { 1 } else { 0 },
                    #[rustfmt::skip]
                    7 => if nums[0] == nums[1] { 1 } else { 0 },
                    _ => panic!(),
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let bits = hex::decode(INPUT.trim())?;
    let bits = bits.view_bits::<Msb0>();
    let (_, packet) = Packet::parse(bits);
    println!("1: {}", packet.version_sum());
    println!("2: {}", packet.eval());
    Ok(())
}
