use std::borrow::Borrow;
use web_sys::console;
use crate::logic::Point;
use crate::spheres::Ball;

#[derive(Debug)]
pub struct QuadTreeNode {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    id: u64,
    balls: Vec<Ball>,
    top_left: Option<Box<QuadTreeNode>>,
    top_right: Option<Box<QuadTreeNode>>,
    bottom_left: Option<Box<QuadTreeNode>>,
    bottom_right: Option<Box<QuadTreeNode>>,

}

fn is_ball_in_cell(x: f64, y: f64, w: f64, h: f64, ball: &Ball) -> bool {
    if ball.pos.x < x || ball.pos.x > x + w || ball.pos.y < y || ball.pos.y > y + h {
        return false;
    }
    return true;
}

impl QuadTreeNode {
    pub fn new(x: f64, y: f64, w: f64, h: f64, balls: Vec<Ball>, id: u64) -> Option<Box<QuadTreeNode>> {
        let mut top_left: Vec<Ball> = Vec::new();
        let mut top_right: Vec<Ball> = Vec::new();
        let mut bottom_left: Vec<Ball> = Vec::new();
        let mut bottom_right: Vec<Ball> = Vec::new();

        let mut tl_node: Option<Box<QuadTreeNode>>;
        let mut tr_node: Option<Box<QuadTreeNode>>;
        let mut bl_node: Option<Box<QuadTreeNode>>;
        let mut br_node: Option<Box<QuadTreeNode>>;

        if balls.len() == 1 && 2.0 * balls[0].radius > w {
            return None;
        }

        let new_w = w / 2.0;
        let new_h = h / 2.0;

        for ball in balls.iter().cloned() {
            //top-left check
            if is_ball_in_cell(x, y, new_w, new_h, &ball) {
                top_left.push(ball);
            }
            //top-right check
            if is_ball_in_cell(x + new_w, y, new_w, new_h, &ball) {
                top_right.push(ball);
            }
            //bottom-left check
            if is_ball_in_cell(x, y + new_h, new_w, new_h, &ball) {
                bottom_left.push(ball);
            }

            //bottom-right check
            if is_ball_in_cell(x + new_w, y + new_h, new_w, new_h, &ball) {
                bottom_right.push(ball);
            }
        }

        if top_left.len() > 0 {
            tl_node = QuadTreeNode::new(x, y, new_w, new_h, top_left, 10 * id + 1);
        } else {
            tl_node = None;
        }

        if top_right.len() > 0 {
            tr_node = QuadTreeNode::new(x + new_w, y, new_w, new_h, top_right, 10 * id + 2);
        } else {
            tr_node = None;
        }

        if bottom_left.len() > 0 {
            bl_node = QuadTreeNode::new(x, y + new_h, new_w, new_h, bottom_left, 10 * id + 3);
        } else {
            bl_node = None;
        }

        if bottom_right.len() > 0 {
            br_node = QuadTreeNode::new(x + new_w, y + new_h, new_w, new_h, bottom_right, 10 * id + 4);
        } else {
            br_node = None;
        }

        // if all children are None, it means we have a leaf node
        if tl_node.is_none() && tr_node.is_none() && bl_node.is_none() && br_node.is_none() && balls.len() > 1 {
            console::log_1(&format!("QTN {} has {}", id, balls.len()).into());
        }

        Option::from(Box::new(Self {
            x,
            y,
            w,
            h,
            id,
            balls,
            top_left: tl_node,
            top_right: tr_node,
            bottom_left: bl_node,
            bottom_right: br_node,
        }))
    }

    pub fn search(&self, ball: Ball) -> bool {
        todo!()
    }

    pub fn insert(&self) {
        todo!()
    }

    pub fn remove(&self) {
        todo!()
    }

    pub fn get_rectangles(&self) -> Vec<Rect> {
        let x = self.x;
        let y = self.y;
        let w = self.w;
        let h = self.h;
        let many = self.balls.len() > 1;

        let r: Rect = Rect { x, y, w, h, many };
        let mut rects = vec![r];

        //let top_left_rects = self.top_left.as_ref().unwrap().get_rectangles();

        if self.top_left.is_some() {
            let tl = self.top_left.as_ref().unwrap();
            // console::log_1(&format!("in get_rectangles, got TL " ).into());
            // console::log_1(&format!("TL = {} {} {} {}",tl.x, tl.y, tl.w, tl.h ).into());

            let additional = tl.get_rectangles();
            rects.extend(additional);
        }

        if self.top_right.is_some() {
            let tl = self.top_right.as_ref().unwrap();
            // console::log_1(&format!("in get_rectangles, got TR " ).into());
            // console::log_1(&format!("TL = {} {} {} {}",tl.x, tl.y, tl.w, tl.h ).into());
            let additional = tl.get_rectangles();
            rects.extend(additional);
        }

        if self.bottom_left.is_some() {
            let tl = self.bottom_left.as_ref().unwrap();
            // console::log_1(&format!("in get_rectangles, got BL " ).into());
            // console::log_1(&format!("TL = {} {} {} {}",tl.x, tl.y, tl.w, tl.h ).into());
            let additional = tl.get_rectangles();
            rects.extend(additional);
        }

        if self.bottom_right.is_some() {
            let tl = self.bottom_right.as_ref().unwrap();
            // console::log_1(&format!("in get_rectangles, got BR " ).into());
            // console::log_1(&format!("TL = {} {} {} {}",tl.x, tl.y, tl.w, tl.h ).into());
            let additional = tl.get_rectangles();
            rects.extend(additional);
        }

        // let top_right_rects = self.top_right..as_ref().unwrap().get_rectangles();
        // let bottom_left_rects = self.bottom_left..as_ref().unwrap().get_rectangles();
        // let bottom_right_rects = self.bottom_right..as_ref().unwrap().get_rectangles();


        //rects.extend(top_left_rects);

        return rects;
    }
}

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub many: bool,
}

#[cfg(test)]
mod testovi {
    use crate::quadtree::QuadTreeNode;
    use crate::spheres::{Ball, BLACK_BALL, BLUE_BALL, GRAY_BALL, RED_BALL, WHITE_BALL};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn construct_quadtree() {
        let width = 800;
        let height = 600;

        let balls = vec![
            Ball::random_ball(width, height, BLACK_BALL),
            Ball::random_ball(width, height, WHITE_BALL),
            Ball::random_ball(width, height, GRAY_BALL),
            Ball::random_ball(width, height, RED_BALL),
            Ball::random_ball(width, height, BLUE_BALL),
        ];

        let qt = QuadTreeNode::new(0.0, 0.0, width as f64, height as f64, balls, 0);
        println!("{}", format!("{:#?}", qt));
        assert_eq!(true, true)
    }
}
