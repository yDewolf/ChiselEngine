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

    pub fn node_ready(args: &[&u32]) {
        println!("{}", args[0]);
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

    pub fn add_node(mut self, mut node: Node) {
        node.id = (self.node_count + 1) as usize;
        node.connect_signal(&Self::node_ready, "ready".to_string());

        self.nodes.push(node);
        self.node_count = self.node_count + 1;
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
    pub id: usize, // Index to the position of the node in SceneTree.nodes vector

    parent: usize,
    pub children: Vec<usize>,

    signals: Vec<Signal>
}

impl Node {
    pub fn new(id: usize) -> Self {
        let children: Vec<usize> = Vec::new();
        let parent: usize = 0;
        let mut signals: Vec<Signal> = Vec::new();

        let ready_signal = Signal::new("ready".to_string(), id);
        signals.push(ready_signal);

        Node {
            id,
            parent,
            children,
            signals
        }
    }

    // Called when scene is loaded
    pub fn _ready(& self) {
        self.emit_signal("ready".to_string(), &[&(self.id as u32)]);
    }

    // Called every frame
    pub fn _process(&self) {

    }

    pub fn add_child(mut self, node: Node) {
        self.children.push(node.id);
        node.set_parent(self.id);
        // Send signal to tree so the node is also added to the tree and node_count updates
    }

    pub fn set_parent(mut self, parent: usize) {
        self.parent = parent;
    }

    pub fn get_parent(self) -> usize {
        self.parent
    }


    pub fn emit_signal(&self, signal_name: String, params: &[&u32]) {
        let signal = self.get_signal(signal_name).unwrap();
        signal.emit(params);
    }

    pub fn connect_signal(&mut self, receiver: &'static dyn Fn(&[&u32]), signal_name: String) {
        let signal: &mut Signal = self.get_mutable_signal(signal_name).unwrap();

        signal.connect(receiver);
    }

    fn get_mutable_signal(&mut self, signal_name: String) -> Result<&mut Signal, ()> {
        let target_signal: &mut Signal;

        if self.signals.is_empty() {
            return Err(())
        }

        for signal in &mut self.signals {
            if signal.name == signal_name {
                target_signal = signal;

                return Ok(target_signal);
            }
        }

        return Err(())

    }

    fn get_signal(&self, signal_name: String) -> Result<&Signal, ()> {
        let target_signal: &Signal;

        if self.signals.is_empty() {
            return Err(())
        }

        for signal in &self.signals {
            if signal.name == signal_name {
                target_signal = signal;

                return Ok(target_signal);
            }
        }

        return Err(())

    }
}

pub struct Signal {
    name: String,
    sender: usize,
    receivers: Vec<&'static dyn Fn(&[&u32])>
}

impl Signal {
    pub fn new(name: String, node_id: usize) -> Self {
        let receivers: Vec<&'static dyn Fn(&[&u32])> = Vec::new();

        Signal {
            name,
            sender: node_id,
            receivers
        }
    }

    pub fn connect(&mut self, receiver: &'static dyn Fn(&[&u32])) {
        self.receivers.push(receiver);
    }

    pub fn emit(&self, args: &[&u32]) {
        for receiver in &self.receivers {
            receiver(args)
        }
    }
}
