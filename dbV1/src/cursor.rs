use crate::{table::Table, leaf_node::LeafNode};

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
        self.cell_num = 0;
        self.page_num = self.table.root_page_num;
        let root_data = self.table.pager.get_page(self.page_num);
        let num_cells = LeafNode::leaf_node_num_cells(root_data);
        self.end_of_table = *num_cells == 0;
        println!("table_start\npage number: {} has {} cells", self.page_num, LeafNode::leaf_node_num_cells(root_data));
    }

    pub fn table_end(&mut self) {
        let root_data = self.table.pager.get_page(self.table.root_page_num);
        let num_cells = LeafNode::leaf_node_num_cells(root_data);
        self.cell_num = *num_cells;
        self.end_of_table = false;
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
            println!(" === accessing new cell === ");
            return LeafNode::leaf_node_value(page, num_cells);
        }
    }
}