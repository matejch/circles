use std::borrow::Borrow;
use std::collections::HashMap;
use web_sys::console;
use crate::ball::{Ball, BallPair, is_ball_in_cell, is_ball_in_cell_diag};
use itertools::{all, any, Itertools};
use crate::geometry::{Cells, Rect};


#[derive(Debug)]
pub struct QuadTreeNode {
    rect: Rect,
    id: usize,
    depth: u8,
    balls: HashMap<usize, Ball>,
    top_left: ChildNode,
    top_right: ChildNode,
    bottom_left: ChildNode,
    bottom_right: ChildNode,
}

pub type ChildNode = Option<Box<QuadTreeNode>>;


impl QuadTreeNode {
    pub fn new(rect: Rect, id: usize, depth: u8) -> ChildNode {
        // let w = rect.w;
        // let size = balls.len();
        // if size > 0 {
        //console::log_1(&format!("QTN {} has {}", id, size).into());
        // }
        // let mut top_left: HashMap<usize, Ball> = HashMap::new();
        // let mut top_right: HashMap<usize, Ball> = HashMap::new();
        // let mut bottom_left: HashMap<usize, Ball> = HashMap::new();
        // let mut bottom_right: HashMap<usize, Ball> = HashMap::new();

        // if size == 0
        //     || depth > 6
        //     || w < 1.0 {
        //     return None;
        // }


        // if size == 1 {
        //     let elm = balls.values().next().unwrap();
        //     if 1.0 * elm.next_radius > w {
        //         return None;
        //     }
        // }
        //
        // let ball_radii = balls.values().map(|ball| ball.radius).collect::<Vec<f64>>();
        // //let largest_radius = ball_radii.iter().max_by(|a, b| a.total_cmp(b)).expect("expect one largest element");
        // let max_radius = ball_radii.iter().copied().fold(f64::NAN, f64::max);

        // if max_radius > w && size > 1 {
        //     return Option::from(Box::new(Self {
        //         rect,
        //         id,
        //         balls,
        //         depth,
        //         top_left: None,
        //         top_right: None,
        //         bottom_left: None,
        //         bottom_right: None,
        //     }));
        // }


        // for (key, ball) in balls.clone() {
        //     //top-left cell
        //     if is_ball_in_cell_diag(x, y, new_w, new_h, &ball) {
        //         top_left.insert(key, ball);
        //     }
        //     //top-right cell
        //     if is_ball_in_cell_diag(x + new_w, y, new_w, new_h, &ball) {
        //         top_right.insert(key, ball);
        //     }
        //     //bottom-left cell
        //     if is_ball_in_cell_diag(x, y + new_h, new_w, new_h, &ball) {
        //         bottom_left.insert(key, ball);
        //     }
        //
        //     //bottom-right cell
        //     if is_ball_in_cell_diag(x + new_w, y + new_h, new_w, new_h, &ball) {
        //         bottom_right.insert(key, ball);
        //     }
        // }

        // let tl_count = top_left.len();
        // let tr_count = top_right.len();
        // let bl_count = bottom_left.len();
        // let br_count = bottom_right.len();
        //
        // let new_cells = rect.split_to_four_cells();
        //
        // let tl_node: ChildNode = QuadTreeNode::new(new_cells.0, top_left, 10 * id + 1, depth + 1);
        // let tr_node: ChildNode = QuadTreeNode::new(new_cells.1, top_right, 10 * id + 2, depth + 1);
        // let bl_node: ChildNode = QuadTreeNode::new(new_cells.2, bottom_left, 10 * id + 3, depth + 1);
        // let br_node: ChildNode = QuadTreeNode::new(new_cells.3, bottom_right, 10 * id + 4, depth + 1);

        let balls: HashMap<usize, Ball> = HashMap::new();

        Option::from(Box::new(Self {
            rect,
            id,
            depth,
            balls,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }))
    }

    pub fn contains(&self, id: usize) -> bool {
        return self.balls.contains_key(&id);
    }

    pub fn insert_ball(&mut self, ball: Ball) {
        let ball_box = ball.bounding_rect_next();
        let node_cell = self.rect.borrow();
        let new_cells = node_cell.split_to_four_cells();

        // which cell does the ball belong to?
        match node_cell.where_is_point_relative_to_center(ball.next_position) {
            Cells::TopLeft => {
                if ball_box.fits(new_cells.0) {
                    // ball fits the cell, try one depth further
                    if self.top_left.is_none() {
                        self.top_left = Option::from(Box::new(Self {
                            rect: new_cells.0.clone(),
                            id: 10 * self.id + 1,
                            depth: self.depth + 1,
                            balls: HashMap::new(),
                            top_left: None,
                            top_right: None,
                            bottom_left: None,
                            bottom_right: None,
                        }));
                    }
                    let mut curr_cell = self.top_left.as_ref().expect("top left should exist");
                    curr_cell.insert_ball(ball);
                    if curr_cell.balls.len()>1 {
                        
                    }

                } else {
                    // ball doesn't fit, add it to current QTN
                    self.balls.insert(ball.id, ball);
                }
            }
            Cells::TopRight => {}
            Cells::BottomLeft => {}
            Cells::BottomRight => {}
        }
    }


    pub fn search(&self, id: usize) -> Vec<usize> {
        let mut rez: Vec<usize> = vec![];
        if self.balls.contains_key(&id) {
            rez.push(self.id);
        }
        if self.top_left.is_some() {
            let node = self.top_left.as_ref().unwrap();
            rez.extend(node.search(id));
        }
        if self.top_right.is_some() {
            let node = self.top_right.as_ref().unwrap();
            rez.extend(node.search(id));
        }
        if self.bottom_left.is_some() {
            let node = self.bottom_left.as_ref().unwrap();
            rez.extend(node.search(id));
        }
        if self.bottom_right.is_some() {
            let node = self.bottom_right.as_ref().unwrap();
            rez.extend(node.search(id));
        }

        rez.sort();
        return rez;
    }

    pub fn find_collision_candidates(&self) -> Vec<BallPair> {
        let mut candidates = vec![];

        if self.is_collision_candidate {
            let combos = self.balls.values().cloned().collect::<Vec<Ball>>();

            for c in combos.iter().combinations(2) {
                candidates.push(BallPair { first: *c[0], second: *c[1] })
            }
        }


        candidates.extend(get_child_collisions(&self.top_left));
        candidates.extend(get_child_collisions(&self.top_right));
        candidates.extend(get_child_collisions(&self.bottom_left));
        candidates.extend(get_child_collisions(&self.bottom_right));
        // if candidates.len() > 0 {
        //     console::log_1(&format!("found candidate {} {}", self.id, candidates.len()).into());
        //     for c in &candidates {
        //         console::log_1(&format!("candidate {} {}", c.first.id, c.second.id).into());
        //     };
        //
        // }

        return candidates;
    }

    pub fn get_rectangles(&self) -> Vec<Rect> {
        let x = self.x;
        let y = self.y;
        let w = self.w;
        let h = self.h;
        let mut many = false;
        let mut rects = vec![];

        if w < 2.0 {
            return vec![];
        }


        if self.top_left.is_some() {
            let tl = self.top_left.as_ref().unwrap();
            let additional = tl.get_rectangles();
            rects.extend(additional);
        }

        if self.top_right.is_some() {
            let tl = self.top_right.as_ref().unwrap();
            let additional = tl.get_rectangles();
            rects.extend(additional);
        }

        if self.bottom_left.is_some() {
            let tl = self.bottom_left.as_ref().unwrap();
            let additional = tl.get_rectangles();
            rects.extend(additional);
        }

        if self.bottom_right.is_some() {
            let tl = self.bottom_right.as_ref().unwrap();
            let additional = tl.get_rectangles();
            rects.extend(additional);
        }
        many = all(vec!(self.bottom_left, self.bottom_right), |item| item.is_none()) && self.balls.len() > 1;
        let r: Rect = Rect { x, y, w, h, many };
        rects.push(r);

        return rects;
    }


    pub fn info_collisions(&self) -> Vec<String> {
        let mut lines = vec![];

        if (self.is_collision_candidate || self.children_are_collision_candidates) {
            let keys: Vec<String> = self.balls.keys().map(|key| key.to_string()).collect();
            let keyo = keys.join(",");
            lines.push(format!(" {} is:{} children:{} balls:{:#?}", self.id, self.is_collision_candidate, self.children_are_collision_candidates, keyo));
        } else {
            return lines;
        }


        if self.top_left.is_some() {
            let node = self.top_left.as_ref().unwrap();
            lines.extend(node.info_collisions());
        }

        if self.top_right.is_some() {
            let node = self.top_right.as_ref().unwrap();
            lines.extend(node.info_collisions());
        }


        if self.bottom_left.is_some() {
            let node = self.bottom_left.as_ref().unwrap();
            lines.extend(node.info_collisions());
        }

        if self.bottom_right.is_some() {
            let node = self.bottom_right.as_ref().unwrap();
            lines.extend(node.info_collisions());
        }

        return lines;
    }

    pub fn info_ball_quads(&self) -> Vec<String> {
        let mut lines = vec![];

        for id in self.balls.keys().sorted() {
            let quad_ids = self.search(*id);
            let quad_ids_str: Vec<String> = quad_ids.iter().map(|id| id.to_string()).collect();
            let line = quad_ids_str.join(",");
            lines.push(format!(" ball id:{} {:#?} is in {}", id, self.balls.get(id).expect("").color, line));
        }

        return lines;
    }
}


fn get_child_collisions(node: &ChildNode) -> Vec<BallPair> {
    if node.is_some() {
        let cands = node.as_ref().unwrap().find_collision_candidates();
        if cands.len() > 0 {
            //console::log_1(&format!("candidates in this node: {:#?}", cands).into());
            return cands;
        }
    }
    return vec![];
}




