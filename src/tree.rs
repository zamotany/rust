pub mod tree {
  use std::any::Any;
  use std::cmp::PartialEq;
  use std::collections::HashMap;
  use std::hash::{Hash, Hasher};
  use uuid::Uuid;

  #[derive(Debug)]
  pub struct ChildrenNode {
    pub id: Uuid,
    pub parent: Uuid,
    pub children: Vec<Uuid>,
  }

  impl ChildrenNode {
    pub fn new_inner(id: Uuid, parent: Uuid) -> ChildrenNode {
      ChildrenNode {
        id,
        parent,
        children: Vec::new(),
      }
    }

    pub fn new(id: Uuid, parent: Uuid) -> Node {
      Node::ChildrenNode(ChildrenNode::new_inner(id, parent))
    }
  }

  #[derive(Debug)]
  pub struct LeftRightNode {
    pub id: Uuid,
    pub parent: Uuid,
    pub left: Option<Uuid>,
    pub right: Option<Uuid>,
  }

  impl LeftRightNode {
    pub fn new_inner(id: Uuid, parent: Uuid) -> LeftRightNode {
      LeftRightNode {
        id,
        parent,
        left: None,
        right: None,
      }
    }

    pub fn new(id: Uuid, parent: Uuid) -> Node {
      Node::LeftRightNode(LeftRightNode::new_inner(id, parent))
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
    node_map: HashMap<Uuid, Node>,
  }

  #[derive(Debug)]
  pub enum Error {
    NotFound,
    Unknown,
  }

  impl NodeTree {
    pub fn new() -> NodeTree {
      NodeTree {
        node_map: HashMap::new(),
      }
    }

    pub fn make_root(&mut self) -> Uuid {
      let id = Uuid::new_v4();
      let node = ChildrenNode::new(id, id);
      self.node_map.insert(id, node);
      id
    }

    pub fn get_node(&self, id: Uuid) -> Option<&Node> {
      if let Some(node) = self.node_map.get(&id) {
        Some(node)
      } else {
        None
      }
    }

    pub fn get_node_mut(&mut self, id: Uuid) -> Option<&mut Node> {
      if let Some(node) = self.node_map.get_mut(&id) {
        Some(node)
      } else {
        None
      }
    }

    pub fn make_node(
      &mut self,
      parent_id: Uuid,
      factory: Box<dyn Fn(Uuid, &mut Node) -> Node>,
    ) -> Result<Uuid, Error> {
      let id = Uuid::new_v4();
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
      LeftRightNode::new(id, parent_inner.id)
    }));
    let child_2_id = node_tree.make_node(root_id, Box::new(move |id, parent| {
      let parent_inner = parent.get_inner_mut::<ChildrenNode>().unwrap();
      parent_inner.children.push(id);
      LeftRightNode::new(id, parent_inner.id)
    }));
    println!("root_id={:?} child_1_id={:?} child_2_id={:?}", root_id, child_1_id, child_2_id);
    println!("root_node {:?}", node_tree.get_node(root_id))
  }
}
