use crate::common::Point;
use std::{cell::Cell, ptr, str::Lines};

#[derive(Clone, Copy)]
enum CircuitType<'a> {
    MasterCircuit(usize),
    SubCircuit(&'a Circuit<'a>),
}

struct Circuit<'a> {
    variant: Cell<CircuitType<'a>>,
}

impl<'a> Circuit<'a> {
    fn get_n_juncs(&self) -> usize {
        let mut master = self;
        loop {
            match master.variant.get() {
                CircuitType::SubCircuit(parent_circuit) => master = parent_circuit,
                CircuitType::MasterCircuit(n_juncs) => {
                    return n_juncs;
                }
            }
        }
    }

    fn extend_n_juncs(&self, extra_juncs: usize) {
        let mut master = self;
        loop {
            match master.variant.get() {
                CircuitType::SubCircuit(parent_circuit) => master = parent_circuit,
                CircuitType::MasterCircuit(n_juncs) => {
                    master
                        .variant
                        .set(CircuitType::MasterCircuit(n_juncs + extra_juncs));
                    break;
                }
            }
        }
    }
}

struct Junction<'a> {
    location: Point<3>,
    circuit: Cell<Option<&'a Circuit<'a>>>,
}

impl<'a> Junction<'a> {
    fn get_circuit(&self) -> Option<&'a Circuit<'a>> {
        if let Some(mut curr_circuit) = self.circuit.get() {
            // Keep traversing circuit pointers until we find the master circuit
            while let CircuitType::SubCircuit(parent_circuit) = curr_circuit.variant.get() {
                curr_circuit = parent_circuit;
            }
            // Lazily update this junction's circuit ref to make lookups cheaper in the future
            let circuit = Some(curr_circuit);
            self.circuit.set(circuit);
            return circuit;
        } else {
            return None;
        }
    }
}

struct Connection {
    left: usize,
    right: usize,
    len: f64,
}

struct Model<'a> {
    junctions: Vec<Junction<'a>>,
    max_cables: usize,
}

fn parse_input(mut input: Lines) -> Model {
    let max_cables = input.next().unwrap().parse().unwrap();
    let junctions = input
        .map(|str| Junction {
            location: Point::<3>::from_str(str),
            circuit: Cell::new(None),
        })
        .collect();
    Model {
        junctions,
        max_cables,
    }
}

fn calculate_result(model: Model) -> usize {
    let juncs = model.junctions;
    let mut conns = Vec::new();

    for (l, left) in juncs[1..juncs.len()].iter().enumerate() {
        for (r, right) in juncs[0..l].iter().enumerate() {
            let len = left.location.dist(&right.location);
            conns.push(Connection {
                left: l,
                right: r,
                len,
            });
        }
    }

    conns.sort_by(|a, b| a.len.partial_cmp(&b.len).unwrap());

    let mut cables = 0;
    let mut circuits = Vec::new();
    while cables < model.max_cables
        && let Some(conn) = conns.pop()
    {
        let l_junc = &juncs[conn.left];
        let r_junc = &juncs[conn.right];
        let l_point = l_junc.location;
        let r_point = r_junc.location;
        match (l_junc.get_circuit(), r_junc.get_circuit()) {
            (Some(l_circ), Some(r_circ)) => {
                if ptr::eq(l_circ, r_circ) {
                    println!("{l_point} {r_point} are in the same circuit already");
                    continue;
                } else {
                    println!("{l_point} {r_point} are in different circuits; joining");
                    r_circ.variant.set(CircuitType::SubCircuit(l_circ));
                    l_circ.extend_n_juncs(r_circ.get_n_juncs());
                }
            }
            (Some(l_circ), None) => {
                println!("{l_point} {r_point} adding right to left circuit");
                r_junc.circuit.set(Some(l_circ));
                l_circ.extend_n_juncs(1);
            }
            (None, Some(r_circ)) => {
                println!("{l_point} {r_point} adding left to right circuit");
                l_junc.circuit.set(Some(r_circ));
                r_circ.extend_n_juncs(1);
            }
            (None, None) => {
                println!("{l_point} {r_point} are not in circuits; creating new one");
                circuits.push(Circuit {
                    variant: Cell::new(CircuitType::MasterCircuit(2)),
                });
                let circ_ref = &circuits[circuits.len() - 1];
                l_junc.circuit.set(Some(circ_ref));
                r_junc.circuit.set(Some(circ_ref));
            }
        }
        cables += 1;
    }

    // Find the largest circuits
    let mut circuit_lengths: Vec<usize> = circuits
        .iter()
        .filter_map(|circ| match circ.variant.get() {
            CircuitType::MasterCircuit(n_juncs) => Some(n_juncs),
            _ => None,
        })
        .collect();

    circuit_lengths.sort();

    for len in circuit_lengths.iter() {
        println!("{len}");
    }
    return circuit_lengths[0] * circuit_lengths[1] * circuit_lengths[2];
}

pub fn run(input: Lines) -> usize {
    let model = parse_input(input);
    return calculate_result(model);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let data = include_str!("../tests/day8/sample.txt").lines();
        assert_eq!(run(data), 40);
    }

    #[test]
    #[ignore]
    fn challenge() {
        let data = include_str!("../tests/day8/challenge.txt").lines();
        assert_eq!(run(data), 48989920237096);
    }
}
