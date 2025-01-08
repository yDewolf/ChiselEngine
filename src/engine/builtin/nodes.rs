
pub struct SceneTree {
    pub root: Node,
    nodes: Vec<Node>,
    node_count: u32
}

impl SceneTree {
    pub fn _ready(&self) {
        let ready_order: Vec<&Node> = self.get_deep_children(&self.root);

        for node in ready_order {
            node._ready();
        }
    }

    pub fn get_node(&self, id: usize) -> &Node {
        return self.nodes.get(id).unwrap();
    }

    pub fn get_deep_children(&self, node: &Node) -> Vec<&Node> {
        let mut children: Vec<&Node> = Vec::new();

        for child in &node.children {
            let child_node = self.get_node(*child);
            let mut child_children = self.get_deep_children(child_node);
            
            children.append(&mut child_children);
            children.push(child_node)
        }

        return children;
    }


}

pub struct Scene {
    tree: SceneTree
}

impl Scene {
    pub fn get_tree(self) -> SceneTree {
        return self.tree;
    }
}


pub struct Node {
    parent: usize,
    pub id: usize, // Index to the position of the node in SceneTree.nodes vector
    
    pub children: Vec<usize>,
    //path: Vec<usize>
}

impl Node {
    pub fn new(id: usize) -> Self {
        let children: Vec<usize> = Vec::new();
        let parent: usize = 0;

        Node {
            parent,
            id,
            children
        }
    }

    // Called when scene is loaded
    pub fn _ready(&self) {

    }

    // Called every frame
    pub fn _process(&self) {

    }

    pub fn add_child(mut self, mut node: Node) {
        self.children.push(node.id);
        node.parent = self.id; 
        // Send signal to tree so the node is also added to the tree and node_count updates
    }
}

