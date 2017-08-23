use std::ops::IndexMut;

/// Immutable Tree
///
/// A Tree that can only add more nodes but can't update the current
/// values at all. You can choose between two variants when creating the tree:
/// `Emphemeral` and `Persistent`. The former discards the previous version of
/// the tree and the latter keeps previous versions. As of right now only the
/// `Emphemeral` type is implemented.
#[derive(Debug, Clone)]
pub struct ITree<T> {
    nodes: Vec<INode<T>>,
}

impl<T> ITree<T> {
    /// Create a new empty `IBTree`
    pub fn new() -> Self{
        Self { nodes: Vec::new() }
    }

    /// Get the root node if it exists
    pub fn root(&self) -> Option<&INode<T>> {
        self.nodes.get(0)
    }

    /// Get the `INode` of the given `Id` if it exists
    pub fn get(&self, node: NodeId) -> Option<&INode<T>> {
        let NodeId(x) = node;
        self.nodes.get(x)
    }

    /// Adds a child to a node in the tree. If the tree is empty it discards the
    /// given `NodeId` and creates the root node with the given value. Returns
    /// the `NodeId` of the inserted node.
    pub fn add_node(&mut self, node: NodeId, value: T) -> NodeId {
        if self.nodes.is_empty() {
            self.nodes.push(INode::new(value, None));
            NodeId(0)
        } else {
            let index = NodeId(self.nodes.len());
            let NodeId(x) = node;
            // What if index out of bounds?
            self.nodes.index_mut(x).insert(index);
            self.nodes.push(INode::new(value, Some(node)));
            index
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeId(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct INode<T> {
    value: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> INode<T> {

    /// Create a new `IBTree` node
    fn new(value: T, parent: Option<NodeId>) -> Self {
        Self { value, parent, children: Vec::new() }
    }

    /// Assign the left side value. Only works once
    fn insert(&mut self, value: NodeId) {
        self.children.push(value);
    }

    /// Get the `INode`'s parent `NodeId`
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get the `INode`'s parent `NodeId`
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    /// Get the `INode`'s children `NodeId`s
    pub fn children(&self) -> &Vec<NodeId> {
        &self.children
    }
}

#[test]
fn insert() {

    /// Shorthand for NodeId
    macro_rules! n {
        ($x:expr) => {
            NodeId($x)
        }
    }

    let mut tree = ITree::new();
    // 0
    tree.add_node(n!(0),0);
    // 0 -> 1
    tree.add_node(n!(0),1);
    // 0 -> 1
    // |--> 2
    tree.add_node(n!(0),2);
    // 0 -> 1
    // |--> 2 -> 3
    tree.add_node(n!(2),3);

    // Check stored values are correct
    assert_eq!(*tree.get(n!(0)).unwrap().value(), 0);
    assert_eq!(*tree.get(n!(1)).unwrap().value(), 1);
    assert_eq!(*tree.get(n!(2)).unwrap().value(), 2);
    assert_eq!(*tree.get(n!(3)).unwrap().value(), 3);

    // Check children values are correct
    assert_eq!(tree.get(n!(0)).unwrap().parent(), None);
    assert_eq!(tree.get(n!(1)).unwrap().parent(), Some(n!(0)));
    assert_eq!(tree.get(n!(2)).unwrap().parent(), Some(n!(0)));
    assert_eq!(tree.get(n!(3)).unwrap().parent(), Some(n!(2)));

    // Check children values are correct
    assert_eq!(tree.get(n!(0)).unwrap().children(), &vec![n!(1), n!(2)]);
    assert_eq!(tree.get(n!(1)).unwrap().children(), &vec![]);
    assert_eq!(tree.get(n!(2)).unwrap().children(), &vec![n!(3)]);
    assert_eq!(tree.get(n!(3)).unwrap().children(), &vec![]);
}
