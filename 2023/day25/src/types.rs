use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Diagram<'a> {
    connections: HashMap<&'a str, HashSet<&'a str>>,
}
impl<'a> Diagram<'a> {
    pub fn from_input<'b>(input: &'b str) -> Diagram where 'b: 'a {
        let mut connections : HashMap<&str, HashSet<&str>> = HashMap::new();

        for line in input.lines() {
            let line_parts : Vec<&str> = line.split(": ").collect();
            let from_part = line_parts[0].trim();
            let to_parts : HashSet<&str> = line_parts[1].split(" ").map(|x| x.trim()).collect();

            for to_part in to_parts {
                connections.entry(from_part).or_default().insert(to_part);
                connections.entry(to_part).or_default().insert(from_part);
            }
        }

        Diagram {
            connections
        }
    }

    pub fn get_standalone_wires(&self, debug: bool) -> Vec<(&'a str, &'a str)> {
        let mut result = Vec::new();

        for (&node, connections) in &self.connections {
            if debug { println!("Testing node {node}..."); }

            for &connection_a in connections {
                let mut has_transitive = false;

                for &connection_b in connections {
                    if connection_a == connection_b {
                        continue;
                    }

                    let b_connections = self.connections.get(connection_b).unwrap();
                    for &other in b_connections {
                        if other == node || other == connection_a {
                            continue;
                        }

                        let other_other = self.connections.get(other).unwrap();
                        if other_other.contains(connection_a) {
                            if debug { println!(" + Node {node} has connection: {node} -> {connection_a} -> {connection_b} -> {other} -> {node}"); }
                            has_transitive = true;
                            break;
                        }
                    }

                    if has_transitive {
                        break;
                    }                
                }

                if !has_transitive {
                    if debug { println!(" + Connection {node}->{connection_a} has no transitive connection, marking as candidate!")};
                    if !result.contains(&(connection_a, node)) {
                        if debug { println!(" + Pushing..."); }
                        result.push((node, connection_a));
                    } 
                }
            }
        }

        if debug { println!("{result:#?}"); }

        assert_eq!(result.len(), 3);

        result
    }

    pub fn remove_connections(&mut self, connections: &Vec<(&'a str, &'a str)>) {
        for &connection in connections {
            self.connections.get_mut(connection.0).unwrap().remove(connection.1);
            self.connections.get_mut(connection.1).unwrap().remove(connection.0);
        }
    }

    pub fn get_group_sizes(&self, debug: bool) -> Vec<usize> {
        let mut groups : Vec<HashSet<&str>> = Vec::new();

        for &node in self.connections.keys() {
            if groups.iter().find(|s| s.contains(node)).is_some() {
                continue;
            }

            let mut group = HashSet::new();

            let mut group_stack = Vec::new();
            group_stack.push(node);

            while let Some(n) = group_stack.pop() {
                if group.contains(n) {
                    continue;
                }

                group.insert(n);
                group_stack.extend(self.connections.get(n).unwrap());
            }

            groups.push(group);
        }

        if debug { println!("Groups: {groups:#?}"); }
        groups.iter().map(|g| g.len()).collect()
    }
}