use crate::ball::BallState::{Expanding, Normal, Shrinking, Vanish};
use crate::ball::{
    balls_distance_squared, calc_moment_of_collision, resolve_collision_active, Ball, BallPair,
    BallPairIds, ACTIVE_BALL, BLACK_BALL, BLUE_BALL, BROWN_BALL, CYAN_BALL, FORREST_BALL,
    GRAY_BALL, GREEN_BALL, MAGENTA_BALL, MAROON_BALL, NAVY_BALL, OLIVE_BALL, ORANGE, ORANGE_BALL,
    PURPLE_BALL, RED_BALL, SILVER_BALL, TEAL_BALL, WHITE_BALL, YELLOW_BALL,
};
use crate::geometry::{Point, Rect, RenderingRect};
use crate::quadtree::{ChildNode, QuadTreeNode};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use js_sys::Math::random;
use web_sys::console;
use crate::constants::{HEIGHT, WIDTH};

#[derive(Debug)]
pub struct GameState {
    pub rect: Rect,
    pub objects: HashMap<usize, Ball>,
    pub next_id: usize,
    pub captured: u8,
    pub shots: u8,
    pub result: GameResult,
    pub tree: Option<Box<QuadTreeNode>>,
    pub is_paused: bool,
    pub is_render_debug: bool,
}

impl GameState {
    pub fn gen_next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    pub fn insert_object(&mut self, ball: &mut Ball) {
        let new_id = self.gen_next_id();
        ball.id = new_id;
        self.objects.insert(new_id, *ball);
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut new_state = Self {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                w: width as f64,
                h: height as f64,
            },
            objects: HashMap::new(),
            next_id: 1,
            captured: 0,
            shots: 3,
            result: GameResult::Playing,
            tree: None,
            is_paused: false,
            is_render_debug: false,
        };

        new_state.insert_object(Ball::random_ball(0, width, height, BLACK_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, WHITE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, GRAY_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, RED_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, BLUE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, GREEN_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, YELLOW_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, CYAN_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, MAGENTA_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, TEAL_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, OLIVE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, PURPLE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, NAVY_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, MAROON_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, FORREST_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, SILVER_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, ORANGE_BALL).borrow_mut());
        new_state.insert_object(Ball::random_ball(0, width, height, BROWN_BALL).borrow_mut());

        let rect = Rect {
            x: 0.0,
            y: 0.0,
            w: width as f64,
            h: height as f64,
        };
        let qt = QuadTreeNode::new(rect, 0, 0);
        //console::log_1(&format!(" quad tree: {:#?}", qt).into());  //console::log_1(&format!(" quad tree: {:#?}", qt).into());
        new_state.tree = qt;

        new_state
    }

    pub fn check_win_lose(&self) {}

    pub fn get_rectangles(&self) -> Vec<RenderingRect> {
        if self.tree.is_none() {
            return vec![];
        }

        self.tree.as_ref().unwrap().get_rectangles()
    }

    pub fn pause_play(&mut self) {
        self.is_paused = !self.is_paused;

        if self.is_paused {
            self.is_render_debug = true;
        }
    }

    fn handle_collisions(&mut self) {
        if self.tree.is_none() {
            return;
        }

        // find collisions candidates

        //console::log_1(&format!(" #balls {} {:#?}", self.objects.values().len(), self.objects.values()).into());
        let mut candidates: HashSet<BallPairIds> = HashSet::new();
        for ball in self.objects.values() {
            //candidates.extend(self.tree.as_ref().expect("").search(ball.id));
            let found = self.tree.as_ref().expect("").search(ball.id);
            //console::log_1(&format!(" ball {}, candidates {} {:#?}", ball.id, candidates.len(), found).into());

            for f in found {
                candidates.insert(f);
            }
            // console::log_1(&format!(" ball {}, candidates {}", ball.id, candidates.len()).into());
        }

        // check collisions
        //
        //console::log_1(&format!("checking collisions {}", candidates.len()).into());

        for cand in candidates {
            let mut first_ball_state = self
                .objects
                .get_mut(&cand.first)
                .expect("this ball should exist")
                .ball_state;
            let mut second_ball_state = self
                .objects
                .get(&cand.second)
                .expect("this ball should exist")
                .ball_state;
            let ball1 = self
                .objects
                .get(&cand.first)
                .expect("this ball should exist");
            let ball2 = self
                .objects
                .get(&cand.second)
                .expect("this ball should exist");

            let ball_pair = BallPair {
                first: *ball1,
                second: *ball2,
            };
            let calc_dist = balls_distance_squared(*ball1, *ball2);
            let my_dist = ball1.radius * ball1.radius + ball2.radius * ball2.radius;

            if ball_pair.is_collision_bb() {
                //let t = calc_moment_of_collision(ball1, ball2);

                // if first_ball_state == Normal && second_ball_state == Normal {
                //     let mut second_ball = self
                //         .objects
                //         .get_mut(&cand.second)
                //         .expect("this ball should exist");
                //
                //     second_ball.next_velocity.y = - 1.01*second_ball.velocity.y;
                //     second_ball.next_velocity.x = - 0.99*second_ball.velocity.x;
                //
                //     let mut first_ball = self
                //         .objects
                //         .get_mut(&cand.first)
                //         .expect("this ball should exist");
                //     first_ball.next_velocity.y = -1.01*first_ball.velocity.y;
                //     first_ball.next_velocity.x = -0.99*first_ball.velocity.x;
                //
                //
                //     // let nid = self.gen_next_id();
                //     // if random() > 0.97 {
                //     //     self.insert_object(&mut Ball::random_ball(nid, WIDTH - 10, HEIGHT - 10, BROWN_BALL))
                //     // }
                // }

                if (first_ball_state == Shrinking || first_ball_state == Expanding)
                    && second_ball_state == Normal
                {
                    let mut second_ball = self
                        .objects
                        .get_mut(&cand.second)
                        .expect("this ball should exist");

                    second_ball.next_ball_state = Expanding;
                }

                if (second_ball_state == Shrinking || second_ball_state == Expanding)
                    && first_ball_state == Normal
                {
                    let mut first_ball = self
                        .objects
                        .get_mut(&cand.first)
                        .expect("this ball should exist");
                    first_ball.next_ball_state = Expanding;
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.objects.retain(|_key, obj| obj.ball_state != Vanish);

        for (_, obj) in &mut self.objects {
            obj.tick();
        }

        self.update_quadtree();
        self.handle_collisions();

        if self.objects.len() == 0 {
            return;
        }

        for (_, obj) in &mut self.objects {
            obj.apply_tick_changes();
        }
    }

    fn update_quadtree(&mut self) {
        let rect = self.rect;
        let mut qt = QuadTreeNode::new(rect, 0, 0);

        for ball in self.objects.clone().values() {
            qt.as_deref_mut().expect("").insert_ball(ball);
        }
        self.tree = qt;
        //console::log_1(&format!(" quad tree: {:#?}", self.tree).into());
    }

    pub fn show_state(&self) {
        for (_, obj) in &self.objects {
            console::log_1(&format!("{} {}", obj.pos.x, obj.pos.y).into());
        }
    }

    pub fn create_capture_ball(&mut self, x: f64, y: f64) {
        if self.shots == 0 {
            return;
        }
        if !(Point { x, y }).is_in_rect(&self.rect) {
            return;
        }

        let mut active_ball = Ball::new(
            self.gen_next_id(),
            Point { x, y },
            Point { x: 0.0, y: 0.0 },
            ACTIVE_BALL,
            Expanding,
        );
        self.insert_object(active_ball.borrow_mut());
        self.shots -= 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Playing,
    Lost,
    Won,
}
