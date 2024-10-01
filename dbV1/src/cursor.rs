use crate::{leaf_node::LeafNode, node::{Node, NodeType}, table::Table, internal_node::InternalNode};

pub struct Cursor<'a> {
    pub table: &'a mut Table,
    pub cell_num: i32,
    pub page_num: usize,
    pub end_of_table: bool,
}

impl<'a> Cursor<'a> {
    pub fn new(table: &'a mut Table) -> Cursor<'a> {
        let page_num = table.root_page_num;
        let cell_num = 0;
        Cursor {
            table,
            cell_num,
            page_num,
            end_of_table: false
        }
    }

    pub fn table_start(&mut self) {
        Self::table_find(self, 0);
        let node = self.table.pager.get_page(self.page_num);
        let num_cells = *LeafNode::leaf_node_num_cells(node);
        self.end_of_table = num_cells == 0;
    }
    
    // // This function is replaced by the new table_start as this implementation only
    // // works with a single node and is therefore not suitable for multilevel traversal
    // // accross the b tree.

    // pub fn table_start(&mut self) {
    //     self.cell_num = 0;
    //     self.page_num = self.table.root_page_num;
    //     let root_data = self.table.pager.get_page(self.page_num);
    //     let num_cells = LeafNode::leaf_node_num_cells(root_data);
    //     self.end_of_table = *num_cells == 0;
    // }

    // // This function was replaced by the table_find function. As table_end does not main-
    // // tain order while inserting into the database, while table_find does.

    // pub fn table_end(&mut self) {
    //     let root_data = self.table.pager.get_page(self.table.root_page_num);
    //     let num_cells = LeafNode::leaf_node_num_cells(root_data);
    //     self.cell_num = *num_cells;
    //     self.end_of_table = false;
    // }

    pub fn table_find(&mut self, key: i32) {
        let root_page_num = self.table.root_page_num;
        let root_node = self.table.pager.get_page(root_page_num);

        if Node::get_node_type(&root_node) == NodeType::NodeLeaf {
            return LeafNode::leaf_node_search(self, root_page_num, key);
        } else {
            println!("searching internal nodes");
            // std::process::exit(1);
            return InternalNode::internal_node_find(self, root_page_num, key);
        }
    }

    pub fn advance_cursor(&mut self) {
        let page_num = self.page_num;
        let node = self.table.pager.get_page(page_num);
        self.cell_num += 1;
        if self.cell_num >= *LeafNode::leaf_node_num_cells(node) {
            self.end_of_table = true
        };
    }

    pub fn cursor_value(&mut self) -> &mut [u8] {
        let page_num = self.page_num;
        let page = self.table.pager.get_page(page_num);
        let num_cells = *LeafNode::leaf_node_num_cells(page);
        // let mut node = LeafNode::new(page);
        // return LeafNode::leaf_node_value(page, self.cell_num);

        if self.cell_num < num_cells {
            // We are pointing at a valid cell within bounds
            return LeafNode::leaf_node_value(page, self.cell_num);
        } else {
            return LeafNode::leaf_node_value(page, num_cells);
        }
    }
}