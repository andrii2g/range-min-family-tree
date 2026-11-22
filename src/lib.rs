#![forbid(unsafe_code)]

//! Range minimum queries via lowest common ancestors in a min-Cartesian tree.

/// A node's position in the original array.
pub type Index = usize;

/// A tree node and its links, indexed by positions in the original array.
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub value: i64,
    pub parent: Option<Index>,
    pub left: Option<Index>,
    pub right: Option<Index>,
}

/// A range minimum and the two upward paths that meet there.
#[derive(Debug, PartialEq, Eq)]
pub struct Query {
    pub minimum_index: Index,
    pub left_path: Vec<Index>,
    pub right_path: Vec<Index>,
}

/// A min-Cartesian tree ordered by (value, original index).
#[derive(Debug)]
pub struct CartesianTree {
    nodes: Vec<Node>,
    root: Option<Index>,
}

impl CartesianTree {
    /// Builds a tree in linear time. An empty slice produces an empty tree.
    pub fn new(values: &[i64]) -> Self {
        let mut nodes: Vec<Node> = Vec::with_capacity(values.len());
        let mut spine: Vec<Index> = Vec::new();

        for (index, &value) in values.iter().enumerate() {
            let mut left = None;
            // Keep earlier equal values above later ones.
            while let Some(&top) = spine.last() {
                if nodes[top].value <= value {
                    break;
                }
                left = spine.pop();
            }

            let parent = spine.last().copied();
            nodes.push(Node {
                value,
                parent,
                left,
                right: None,
            });
            if let Some(child) = left {
                nodes[child].parent = Some(index);
            }
            if let Some(parent) = parent {
                nodes[parent].right = Some(index);
            }
            spine.push(index);
        }

        let root = spine.first().copied();
        Self { nodes, root }
    }

    pub fn root(&self) -> Option<Index> {
        self.root
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Finds the leftmost minimum in the inclusive interval `[left, right]`.
    ///
    /// Both paths include their endpoint and stop at the lowest common
    /// ancestor. Returns `None` for reversed or out-of-bounds intervals.
    pub fn query(&self, left: Index, right: Index) -> Option<Query> {
        if left > right || right >= self.nodes.len() {
            return None;
        }

        let mut left_path = self.path_to_root(left);
        let mut right_path = self.path_to_root(right);
        // Read the paths from the root down to find their last shared node.
        let shared = left_path
            .iter()
            .rev()
            .zip(right_path.iter().rev())
            .take_while(|(left, right)| left == right)
            .count();
        let minimum_index = left_path[left_path.len() - shared];
        left_path.truncate(left_path.len() - shared + 1);
        right_path.truncate(right_path.len() - shared + 1);

        Some(Query {
            minimum_index,
            left_path,
            right_path,
        })
    }

    fn path_to_root(&self, start: Index) -> Vec<Index> {
        std::iter::successors(Some(start), |&index| self.nodes[index].parent).collect()
    }
}
