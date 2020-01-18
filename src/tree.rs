pub mod tree {
  use std::any::Any;
  use std::cmp::PartialEq;
  use std::collections::HashMap;
  use std::hash::{Hash, Hasher};

  #[derive(Debug, Copy, Clone)]
  pub struct Id(pub u32);

  impl Hash for Id {
    fn hash<H>(&self, state: &mut H)
    where
      H: Hasher,
    {
      let Id(unpacked_id) = self;
      unpacked_id.hash(state);
    }
  }

  impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
    }
  }
  impl Eq for Id {}

  #[derive(Debug)]
  pub struct IdGenerator {
    nextId: Id,
  }

  impl IdGenerator {
    pub fn new() -> IdGenerator {
      IdGenerator { nextId: Id(0) }
    }

    pub fn next(&mut self) -> Id {
      let Id(unpacked_id) = self.nextId;
      self.nextId = Id(unpacked_id + 1);
      return Id(unpacked_id);
    }
  }

  #[derive(Debug)]
  pub struct ChildrenNode {
    pub id: Id,
    pub parent: Id,
    pub children: Vec<Id>,
  }

  impl ChildrenNode {
    pub fn new(id: Id, parent: Id) -> ChildrenNode {
      ChildrenNode {
        id,
        parent,
        children: Vec::new(),
      }
    }
  }

  #[derive(Debug)]
  pub struct LeftRightNode {
    pub id: Id,
    pub parent: Id,
    pub left: Option<Id>,
    pub right: Option<Id>,
  }

  impl LeftRightNode {
    pub fn new(id: Id, parent: Id) -> LeftRightNode {
      LeftRightNode {
        id,
        parent,
        left: None,
        right: None,
      }
    }
  }

  #[derive(Debug)]
  pub enum Node {
    ChildrenNode(ChildrenNode),
    LeftRightNode(LeftRightNode),
  }

  impl Node {
    fn unpack_inner<T: 'static>(node: &dyn Any) -> Option<&T> {
      match node.downcast_ref::<T>() {
        Some(casted_node) => Some(casted_node),
        None => None,
      }
    }

    fn unpack_inner_mut<T: 'static>(node: &mut dyn Any) -> Option<&mut T> {
      match node.downcast_mut::<T>() {
        Some(casted_node) => Some(casted_node),
        None => None,
      }
    }

    pub fn get_inner<T: 'static>(&self) -> Option<&T> {
      match self {
        Node::ChildrenNode(inner_node) => Node::unpack_inner(inner_node),
        Node::LeftRightNode(inner_node) => Node::unpack_inner(inner_node),
      }
    }

    pub fn get_inner_mut<T: 'static>(&mut self) -> Option<&mut T> {
      match self {
        Node::ChildrenNode(inner_node) => Node::unpack_inner_mut(inner_node),
        Node::LeftRightNode(inner_node) => Node::unpack_inner_mut(inner_node),
      }
    }
  }

  #[derive(Debug)]
  pub struct NodeTree {
    id_generator: IdGenerator,
    node_map: HashMap<Id, Node>,
  }

  #[derive(Debug)]
  pub enum Error {
    NotFound,
    Unknown,
  }

  impl NodeTree {
    pub fn new() -> NodeTree {
      NodeTree {
        id_generator: IdGenerator::new(),
        node_map: HashMap::new(),
      }
    }

    pub fn make_root(&mut self) -> Id {
      let id = self.id_generator.next();
      let node = Node::ChildrenNode(ChildrenNode::new(id, id));
      self.node_map.insert(id, node);
      id
    }

    pub fn get_node(&self, id: Id) -> Option<&Node> {
      if let Some(node) = self.node_map.get(&id) {
        Some(node)
      } else {
        None
      }
    }

    pub fn get_node_mut(&mut self, id: Id) -> Option<&mut Node> {
      if let Some(node) = self.node_map.get_mut(&id) {
        Some(node)
      } else {
        None
      }
    }

    pub fn make_node(
      &mut self,
      parent_id: Id,
      factory: Box<dyn Fn(Id, &mut Node) -> Node>,
    ) -> Result<Id, Error> {
      let id = self.id_generator.next();
      if let Some(parent) = self.get_node_mut(parent_id) {
        let node = factory(id, parent);
        self.node_map.insert(id, node);
        Ok(id)
      } else {
        Err(Error::NotFound)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::tree::{ChildrenNode, LeftRightNode, Node, NodeTree};

  #[test]
  fn can_create_node() {
    let mut node_tree = NodeTree::new();
    let root_id = node_tree.make_root();
    let child_1_id = node_tree.make_node(root_id, Box::new(move |id, parent| {
      let parent_inner = parent.get_inner_mut::<ChildrenNode>().unwrap();
      parent_inner.children.push(id);
      Node::LeftRightNode(LeftRightNode::new(id, parent_inner.id))
    }));
    let child_2_id = node_tree.make_node(root_id, Box::new(move |id, parent| {
      let parent_inner = parent.get_inner_mut::<ChildrenNode>().unwrap();
      parent_inner.children.push(id);
      Node::LeftRightNode(LeftRightNode::new(id, parent_inner.id))
    }));
    println!("root_id={:?} child_1_id={:?} child_2_id={:?}", root_id, child_1_id, child_2_id);
    println!("root_node {:?}", node_tree.get_node(root_id))
  }
}
