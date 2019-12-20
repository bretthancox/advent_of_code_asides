use crate::input_file_handler::file_open;


#[derive(Debug, Clone)]
pub struct Node {
    /* Each planet/asteroid/other heavenly body is a Node
    consisting of its own position in the tree of nodes (index),
    it's literal name (string_value), its immediate parent (which can
    be None or exactly 1, hence the Option), and its descendants 
    (which can be >1, hence the vector).*/
    index: usize,
    string_value: String,
    parent: Option<usize>,
    descendants: Vec<usize>,
}

impl PartialEq for Node {
    /*Without implementing PartialEq, could not compare
    names between different nodes*/
    fn eq(&self, comparator: &Self) -> bool {
        self.string_value == comparator.string_value
    }
}

impl Node {
    fn new(index: usize, name: &str) -> Self {
        /*Implemented on Node type, so Self is a Node.
        Creates a new Node and returns it for assignment*/
        Self {
            index,
            string_value: String::from(name),
            parent: None,
            descendants: vec![],
        }
    }
    fn _return_index(&self) -> usize {
        self.index
    }
    fn _return_string_value(&self) -> &str {
        &self.string_value
    }
    fn _return_parent(&self) -> Option<usize> {
        self.parent
    }
    fn _return_descendants(&self) -> Vec<usize> {
        self.descendants.clone()
    }
}



#[derive(Debug, Clone, Default)]
pub struct ChristmasTree {
    /*Thematically-named type for storing the Nodes. 
    Bodies == heavenly bodies.*/
    bodies: Vec<Node>,
}


impl ChristmasTree {
    fn update_node_relationships(&mut self) {
        /*Use the input_file_handler file_open function to open the day6 inputs,
        unwrap the Result to gain access to the string of the file.
        Split the string at newlines to get the separate "PRNT)CHLD" strings
        and then for each string, trim it and pass it to use_the_split_string*/
        let file_input = file_open();
        let input_string = file_input.unwrap();
        input_string.split("\n").for_each(|o| self.use_the_split_string(o.trim()));
    }
    

    fn use_the_split_string(&mut self, parent_and_child: &str) {
        /*Split each string passed to the function (e.g. "PRNT)CHLD")
        into a vector of ["PRNT", "CHLD"].
        For the parent and child, use put_nodes_in_trees to return the 
        index for the parent and child Nodes.*/
        let splits = parent_and_child.split(")").collect::<Vec<&str>>();
        let parent_index = self.put_nodes_in_tree(splits[0]);
        let child_index = self.put_nodes_in_tree(splits[1]);
        /*With the parent and child Node locations now known:
            1) For the child Node, retrieve the parent. As a Node can have only
            one parent, any Some(_) result means we are trying to add a second 
            parent and something has gone wrong.
                a) If the result is None, write the parent index to the child Node
                as Some(parent_index)
            2) For the parent Node, any number of children are valid, so push the
            child Node index to the descendant vector*/
        match self.bodies[child_index].parent {
            Some(_) => panic!("Trying to write to existing parent field"),
            None => self.bodies[child_index].parent = Some(parent_index),
        }
        self.bodies[parent_index].descendants.push(child_index);
    }


    fn put_nodes_in_tree(&mut self, name: &str) -> usize {
        /*Check if any Nodes in the tree have the name of the
        current object. If true, return the Node location.
        If false, create a new Node and push it to the tree with
        and index == length of the tree vector before it's added.
        Return the new Node index.*/
        for node in &self.bodies {
            if node.string_value == name {
                return node.index;
            }
        }
        let index_to_set = self.bodies.len();
        self.bodies.push(Node::new(index_to_set, name));
        index_to_set
    }

    
    fn count_chain_to_root(&self, node_index: usize) -> usize {
        //I count back along the parent line for any given node, until I hit root
        match self.bodies[node_index].parent {
            Some(n) => 1 + self.count_chain_to_root(n),
            None => 0,
        }
    }


    fn sum_all_chains_to_root(&self) -> usize {
        /*I iterate over the tree vector, reducing the chain from node to root
        for every node in the vector. I return the sum*/
        self.bodies.iter().fold(0, |acc, n| {
            let node_chain_length = self.count_chain_to_root(n.index);
            acc + node_chain_length
        })
    }


    fn find_index_for_known_string(&self, value: &str) -> Option<usize> {
        /*For a given string, return an Option containing the index, or None if not found*/
        for node in &self.bodies {
            if node.string_value == value.to_string() {
                return Some(node.index)
            }
        }
        None
    }


    fn chain_between_two_points(&self, point_one: &str, point_two: &str) -> usize {
        /*I do gather the indices for two points and collect the final distance between them.
        I also subtract 2 from the answer returned so that the orbit of each point is not counted,
        per AOC directions*/
        let point_one_index = match self.find_index_for_known_string(point_one) {
            Some(x) => x,
            None => panic!("Didn't find {} in the tree!", point_one),
        };
        let point_two_index = match self.find_index_for_known_string(point_two) {
            Some(x) => x,
            None => panic!("Didn't find {} in the tree!", point_two),
        };

        let count = self.move_slow_node(point_one_index, point_two_index, point_one_index, point_two_index) - 2;
        count
    }


    fn move_slow_node(&self, slow_node: usize, fast_node: usize, no_touch_slow: usize, no_touch_fast: usize) -> usize {
        /*I move the slow node up its parent chain every time the fast node completes a full recur to root
        If the fast node returns Some, it has found a point where both chains meet. The sum of both chains is then
        initiated and the result returned*/
        match self.move_fast_node(slow_node, fast_node) {
            Some(n) => self.sum_all_chains_to_node(n, no_touch_slow, no_touch_fast),
            None => self.move_slow_node(self.bodies[slow_node].parent.unwrap(), fast_node, no_touch_slow, no_touch_fast),
        }
    }


    fn move_fast_node(&self, slow_node: usize, fast_node: usize) -> Option<usize> {
        /*If the slow and fast node match, return an Option containing the matching index.
        Recur through the fast node, propogating an Option back up the recur for return.*/
        if self.bodies[slow_node].index == self.bodies[fast_node].index {
            return Some(slow_node); //The Node to return is irrelevant; they match if you're here
        } else {
             if self.bodies[fast_node].parent.is_some() {
                let par = self.bodies[fast_node].parent.unwrap();
                match self.move_fast_node(slow_node, par) {
                    Some(x) => return Some(x),
                    None => return None,
                } 
            } else {
                    None
                }
            }
        }

    
    fn sum_all_chains_to_node(&self, node_target: usize, node_index_one: usize, node_index_two: usize) -> usize {
        /*I iterate over the tree vector, reducing the chain from node to target node
        for every node in the custom vector. I return the sum*/
        let san_to_you: Vec<usize> = vec![node_index_one, node_index_two];
        println!("{:?} {:?} {:?}", self.bodies[node_index_one], self.bodies[node_index_two], san_to_you);
        san_to_you.iter().fold(0, |acc, &n| {
            let node_chain_length = self.count_chain_to_node(n, node_target);
            acc + node_chain_length
            })
        }   


    fn count_chain_to_node(&self, node_index: usize, node_target: usize) -> usize {
        //I count back along the parent line for any given node, until I hit the target node
        if node_index == node_target {
            return 0
        };
        match self.bodies[node_index].parent {
            Some(n) => 1 + self.count_chain_to_node(n, node_target),
            None => 0,
        }
    }
}


pub fn six_two() {

    let mut tree = ChristmasTree::default();
    tree.update_node_relationships();

    let orbits = tree.sum_all_chains_to_root();
    println!("{}", orbits);

    let chain_from_me_to_santa = tree.chain_between_two_points(&"YOU", &"SAN");
    println!("{}", chain_from_me_to_santa);
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;//{assert_eq, assert_ne};

    #[test]
    fn test_example_from_aoc() {
        let mut test_tree = ChristmasTree::default();
        let test_input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        test_input.split("\n").for_each(|o| test_tree.use_the_split_string(o.trim()));
        let orbits = test_tree.sum_orbits();
        assert_eq!(orbits, 42);
    }

    #[test]
    fn test_count_descendants() {
        let mut test_tree = ChristmasTree::default();
        let test_input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        test_input.split("\n").for_each(|o| test_tree.use_the_split_string(o.trim()));
        let direct_children = test_tree.bodies.iter().fold(0, |acc, node| acc + test_tree._count_descendants(node.index));
        assert_eq!(direct_children, 11);
    }

    #[test]
    fn test_two_points() {
        let mut test_tree = ChristmasTree::default();
        let test_input = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
        test_input.split("\n").for_each(|o| test_tree.use_the_split_string(o.trim()));
        let chain_between = test_tree.chain_between_two_points(&"YOU", &"SAN");
        println!("{}", chain_between);
        assert_eq!(chain_between, 4);
    }

    #[test]
    fn test_two_points_new_you() {
        let mut test_tree = ChristmasTree::default();
        let test_input = String::from("COM)B\nK)YOU\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nI)SAN");
        test_input.split("\n").for_each(|o| test_tree.use_the_split_string(o.trim()));
        let chain_between = test_tree.chain_between_two_points(&"YOU", &"SAN");
        println!("{}", chain_between);
        assert_eq!(chain_between, 4);
    }

    #[test]
    fn test_two_points_rewritten_input() {
        println!("===\ntest_two_points_rewritten_input()\n===");
        let mut test_tree = ChristmasTree::default();
        let test_input = String::from("COM)B\nB)G\nG)H\nB)C\nC)D\nD)E\nE)F\nE)J\nJ)K\nK)L\nD)I\nI)SAN\nK)YOU");
        test_input.split("\n").for_each(|o| test_tree.use_the_split_string(o.trim()));
        let chain_between = test_tree.chain_between_two_points(&"YOU", &"SAN");
        println!("{}", chain_between);
        assert_eq!(chain_between, 4);
    }

    #[test]
    fn test_two_points_self_built_data() {
        let mut test_tree = ChristmasTree::default();
        let test_input = String::from("COM)B\nB)C\nC)D\nD)E\nE)SAN\nB)F\nF)G\nF)H\nH)I\nI)J\nI)K\nK)YOU\nJ)L");
        test_input.split("\n").for_each(|o| test_tree.use_the_split_string(o.trim()));
        let chain_between = test_tree.chain_between_two_points(&"YOU", &"SAN");
        println!("{}", chain_between);
        assert_eq!(chain_between, 7);
    }
}