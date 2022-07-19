use std::collections::HashMap;
use web_sys::console;
use crate::spheres::{Ball, BallPair, is_ball_in_cell, is_ball_in_cell_diag};
use itertools::Itertools;


#[derive(Debug)]
pub struct QuadTreeNode {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    id: usize,
    depth: u8,
    is_collision_candidate: bool,
    children_are_collision_candidates: bool,
    balls: HashMap<usize, Ball>,
    top_left: ChildNode,
    top_right: ChildNode,
    bottom_left: ChildNode,
    bottom_right: ChildNode,

}

pub type ChildNode = Option<Box<QuadTreeNode>>;

// fn get_child_collisions(child:&Option<Box<QuadTreeNode>>) -> Vec<BallPair> {
//     if child.is_some() {
//
//         return child.as_ref().unwrap().
//                                           find_collision_candidates();
//     }
//     return vec![]
// }


impl QuadTreeNode {
    pub fn new(x: f64, y: f64, w: f64, h: f64, balls: HashMap<usize, Ball>, id: usize, depth: u8) -> ChildNode {
        let size = balls.len();
        if size > 0 {
            //console::log_1(&format!("QTN {} has {}", id, size).into());
        }
        let mut top_left: HashMap<usize, Ball> = HashMap::new();
        let mut top_right: HashMap<usize, Ball> = HashMap::new();
        let mut bottom_left: HashMap<usize, Ball> = HashMap::new();
        let mut bottom_right: HashMap<usize, Ball> = HashMap::new();

        if size == 0
            || depth > 4
            || w < 1.0 {
            return None;
        }


        if size == 1 {
            let elm = balls.values().next().unwrap();
            if 1.0 * elm.next_radius > w {
                return None;
            }
        }

        let new_w = w / 2.0;
        let new_h = h / 2.0;

        let ball_radii = balls.values().map(|ball| ball.radius).collect::<Vec<f64>>();
        //let largest_radius = ball_radii.iter().max_by(|a, b| a.total_cmp(b)).expect("expect one largest element");
        let max_radius = ball_radii.iter().copied().fold(f64::NAN, f64::max);

        if max_radius > w  && size > 1 {
            return Option::from(Box::new(Self {
                x,
                y,
                w,
                h,
                id,
                balls,
                is_collision_candidate: true,
                depth,
                children_are_collision_candidates: false,
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None,
            }));
        }


        for (key, ball) in balls.clone() {
            //top-left cell
            if is_ball_in_cell_diag(x, y, new_w, new_h, &ball) {
                top_left.insert(key, ball);
            }
            //top-right cell
            if is_ball_in_cell_diag(x + new_w, y, new_w, new_h, &ball) {
                top_right.insert(key, ball);
            }
            //bottom-left cell
            if is_ball_in_cell_diag(x, y + new_h, new_w, new_h, &ball) {
                bottom_left.insert(key, ball);
            }

            //bottom-right cell
            if is_ball_in_cell_diag(x + new_w, y + new_h, new_w, new_h, &ball) {
                bottom_right.insert(key, ball);
            }
        }

        let tl_count = top_left.len();
        let tr_count = top_right.len();
        let bl_count = bottom_left.len();
        let br_count = bottom_right.len();

        let tl_node: ChildNode = QuadTreeNode::new(x, y, new_w, new_h, top_left, 10 * id + 1, depth + 1);
        let tr_node: ChildNode = QuadTreeNode::new(x + new_w, y, new_w, new_h, top_right, 10 * id + 2, depth + 1);
        let bl_node: ChildNode = QuadTreeNode::new(x, y + new_h, new_w, new_h, bottom_left, 10 * id + 3, depth + 1);
        let br_node: ChildNode = QuadTreeNode::new(x + new_w, y + new_h, new_w, new_h, bottom_right, 10 * id + 4, depth + 1);


        // am i collision candidate?
        let mut is_collision_candidate = false;

        // if all children are None, it means we have a leaf node
        let all_empty = vec![&tl_node, &tr_node, &bl_node, &br_node]
            .iter()
            .all(|node| node.is_none());


        if all_empty && size > 1 {
            //no children but more than one ball means we have a potential collision
            //console::log_1(&format!("QTN {} has {}", id, balls.len()).into());
            is_collision_candidate = true;
        }

        let any_children_candidate = vec![tl_count, tr_count, bl_count, br_count]
            .iter()
            .any(|item| item > &1);

        Option::from(Box::new(Self {
            x,
            y,
            w,
            h,
            id,
            balls,
            is_collision_candidate,
            depth,
            children_are_collision_candidates: any_children_candidate,
            top_left: tl_node,
            top_right: tr_node,
            bottom_left: bl_node,
            bottom_right: br_node,
        }))
    }

    pub fn search(&self, id:usize) -> Vec<usize> {
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
        if candidates.len() > 0 {
            //    console::log_1(&format!("found candidate {} {}", self.id, candidates.len()).into());
        }

        return candidates;
    }

    pub fn get_rectangles(&self) -> Vec<Rect> {
        let x = self.x;
        let y = self.y;
        let w = self.w;
        let h = self.h;
        let many = self.is_collision_candidate;

        if w < 2.0 {
            return vec![];
        }

        let r: Rect = Rect { x, y, w, h, many };
        let mut rects = vec![r];


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
            let quad_ids_str:Vec<String> = quad_ids.iter().map(|id| id.to_string()).collect();
            let line = quad_ids_str.join(",");
            lines.push(format!(" ball id:{} is in {}", id, line));
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

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub many: bool,
}


