/// A data-structure that, given a forest, allows dynamic-connectivity queries.
/// Meaning deletion of an edge (u,v) and checking whether two vertecies are still connected.
///
/// # Complexity
/// The preprocessing phase runs in O(n) time, where n is the the number of vertecies in the forest.
/// Deletion runs in O(log n) and checking for connectivity runs in O(1) time.
///
/// # Sources
/// used Wikipedia as reference: <https://en.wikipedia.org/wiki/Dynamic_connectivity>
pub struct DecrementalConnectivity<'a> {
    adjacent: &'a Vec<Vec<usize>>,
    component: Vec<usize>,
    count: usize,
    visited: Vec<usize>,
    dfs_id: usize,
}
impl<'a> DecrementalConnectivity<'a> {
    //expects the parent of a root to be itself
    pub fn new(adjacent: &'a Vec<Vec<usize>>) -> Result<Self, String> {
        let n = adjacent.len();
        if !is_forest(adjacent) {
            return Err("input graph is not a forest!".to_string());
        }
        let mut tmp = DecrementalConnectivity {
            adjacent,
            component: vec![0; n],
            count: 0,
            visited: vec![0; n],
            dfs_id: 1,
        };
        tmp.component = tmp.calc_component();
        Ok(tmp)
    }

    pub fn connected(&self, u: usize, v: usize) -> Option<bool> {
        match (self.component.get(u), self.component.get(v)) {
            (Some(a), Some(b)) => Some(a == b),
            _ => None,
        }
    }

    // original adjacency will not be modified by this function
    // expects for the graph to have an edge (u,v)
    pub fn delete(&mut self, u: usize, v: usize) {
        if self.component[u] != self.component[v] {
            return;
        }

        let mut queue: Vec<usize> = Vec::new();

        if self.is_smaller(u, v) {
            queue.push(u);
            self.dfs_id += 1;
            self.visited[v] = self.dfs_id;
        } else {
            queue.push(v);
            self.dfs_id += 1;
            self.visited[u] = self.dfs_id;
        }
        while !queue.is_empty() {
            let current = queue[0];
            self.dfs_step(&mut queue, self.dfs_id);
            self.component[current] = self.count;
        }
        self.count += 1;
    }

    fn calc_component(&mut self) -> Vec<usize> {
        let mut visited: Vec<bool> = vec![false; self.adjacent.len()];
        let mut comp: Vec<usize> = vec![0; self.adjacent.len()];

        for i in 0..self.adjacent.len() {
            if visited[i] {
                continue;
            }
            let mut queue: Vec<usize> = vec![i];
            while let Some(current) = queue.pop() {
                if !visited[current] {
                    queue.append(&mut self.adjacent[current].clone());
                }
                visited[current] = true;
                comp[current] = self.count;
            }
            self.count += 1;
        }
        comp
    }

    fn is_smaller(&mut self, u: usize, v: usize) -> bool {
        let mut u_queue: Vec<usize> = vec![u];
        let u_id = self.dfs_id;
        self.visited[v] = u_id;
        self.dfs_id += 1;

        let mut v_queue: Vec<usize> = vec![v];
        let v_id = self.dfs_id;
        self.visited[u] = v_id;
        self.dfs_id += 1;

        // parallel depth first search
        while !u_queue.is_empty() && !v_queue.is_empty() {
            self.dfs_step(&mut u_queue, u_id);
            self.dfs_step(&mut v_queue, v_id);
        }
        u_queue.is_empty()
    }

    fn dfs_step(&mut self, queue: &mut Vec<usize>, dfs_id: usize) {
        let u = queue.pop().unwrap();
        let comp = self.component[u];
        self.visited[u] = dfs_id;
        for v in self.adjacent[u].iter() {
            if self.visited[*v] == dfs_id || self.component[*v] != comp {
                continue;
            }
            queue.push(*v);
        }
    }
}

fn is_forest(adjacent: &Vec<Vec<usize>>) -> bool {
    let mut visited = vec![false; adjacent.len()];
    for node in 0..adjacent.len() {
        if visited[node] {
            continue;
        }
        if has_cycle(adjacent, &mut visited, node, node) {
            return false;
        }
    }
    true
}

fn has_cycle(
    adjacent: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    node: usize,
    parent: usize,
) -> bool {
    visited[node] = true;
    for &neighbour in adjacent[node].iter() {
        if !visited[neighbour] {
            if has_cycle(adjacent, visited, neighbour, node) {
                return true;
            }
        } else if neighbour != parent {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    // test forest (remember the assumptoin that roots are adjacent to themselves)
    //              _              _
    //             \ /            \ /
    //              0              7
    //            / | \            |
    //           1  2  3           8
    //         /   / \
    //        4   5   6
    #[test]
    fn construction_test() {
        let adjacent = vec![
            vec![0, 1, 2, 3],
            vec![0, 4],
            vec![0, 5, 6],
            vec![0],
            vec![1],
            vec![2],
            vec![2],
            vec![7, 8],
            vec![7],
        ];
        let dec_con = super::DecrementalConnectivity::new(&adjacent).unwrap();
        assert_eq!(dec_con.component, vec![0, 0, 0, 0, 0, 0, 0, 1, 1])
    }
    #[test]
    fn query_test() {
        let adjacent = vec![
            vec![0, 1, 2, 3],
            vec![0, 4],
            vec![0, 5, 6],
            vec![0],
            vec![1],
            vec![2],
            vec![2],
            vec![7, 8],
            vec![7],
        ];
        let mut dec_con1 = super::DecrementalConnectivity::new(&adjacent).unwrap();
        assert!(dec_con1.connected(3, 4).unwrap());
        assert!(dec_con1.connected(5, 0).unwrap());
        assert!(!dec_con1.connected(2, 7).unwrap());
        assert!(dec_con1.connected(0, 9).is_none());
        dec_con1.delete(0, 2);
        assert!(dec_con1.connected(3, 4).unwrap());
        assert!(!dec_con1.connected(5, 0).unwrap());
        assert!(dec_con1.connected(5, 6).unwrap());
        assert!(dec_con1.connected(8, 7).unwrap());
        dec_con1.delete(7, 8);
        assert!(!dec_con1.connected(8, 7).unwrap());
        dec_con1.delete(7, 8);
        dec_con1.delete(1, 4);
        assert!(!dec_con1.connected(1, 4).unwrap());

        let mut dec_con2 = super::DecrementalConnectivity::new(&adjacent).unwrap();
        dec_con2.delete(4, 1);
        assert!(!dec_con1.connected(1, 4).unwrap());

        let mut dec_con3 = super::DecrementalConnectivity::new(&adjacent).unwrap();
        dec_con3.delete(1, 4);
        assert!(!dec_con1.connected(4, 1).unwrap());
    }
}
