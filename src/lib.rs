mod ball;
mod constants;
mod geometry;
mod logic;
mod quadtree;
mod random;
mod rendering;
mod utils;

use logic::GameState;

use js_sys::Function;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement, KeyboardEvent, MouseEvent};

use crate::logic::ChangeState;
use crate::logic::GameResult;
use crate::rendering::write_text;
use crate::utils::{document, get_context, get_debug_context, request_animation_frame, window};

thread_local! {
    static GAME: Rc<RefCell<GameState>> = Rc::new(
        RefCell::new(
            GameState::new(
                constants::WIDTH,
                constants::HEIGHT))
    );


    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> =
    Closure::wrap(Box::new(|evt: KeyboardEvent| GAME.with(|game| {

        let change: ChangeState = match &evt.key()[..] {
                "G" | "g" => ChangeState::PlayPause,
                "N" | "n" => {
                    if game.borrow().check_win_lose() == GameResult::Won {
                        ChangeState::NextLevel
                    } else {
                        ChangeState::NoChange
                    }
                }
                "R" | "r" => {
                    ChangeState::RestartLevel
                }

                _=> ChangeState::NoChange
       };

        match change {
            ChangeState::PlayPause => {
                game.borrow_mut().pause_play();
            },
            ChangeState::NextLevel => {
                game.borrow_mut().next_level();
            },
            ChangeState::RestartLevel => {
                game.borrow_mut().restart();
            }
            ChangeState::Quit => {
                game.borrow_mut().quit();
            },
    ChangeState::NoChange => {

            },
}

    })) as Box<dyn FnMut(KeyboardEvent)>);


    static HANDLE_MOUSE: Closure<dyn FnMut(MouseEvent)> =
    Closure::wrap(Box::new(|evt: MouseEvent| GAME.with(|game| {
        let clicked = match &evt.button() {
            0 => true,
            _ => false,
            };
        let game_result: GameResult = game.borrow().check_win_lose();
        if clicked {
            match game_result {
                GameResult::Playing => {
                  //  game.borrow_mut().create_capture_ball(evt.client_x() as f64,evt.client_y() as f64);
                    game.borrow_mut().create_capture_ball(evt.offset_x() as f64,evt.offset_y() as f64);

                },
                GameResult::Lost => {
                    game.borrow_mut().restart();
                },
                GameResult::Won => {
                    game.borrow_mut().next_level();
                },
        }
      }}
    )) as Box<dyn FnMut(MouseEvent)>);

}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    //let this_window = window();
    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .add_event_listener_with_callback(
                "keydown",
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });

    HANDLE_MOUSE.with(|handle_mousedown| {
        window()
            .add_event_listener_with_callback(
                "mousedown",
                handle_mousedown
                    .as_ref()
                    .dyn_ref::<Function>()
                    .unwrap_throw(),
            )
            .unwrap_throw();
    });

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut ctx = get_context();
    //let debug_ctx = get_debug_context();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        GAME.with(|game| {
            let mut doc = document();

            let root_container = document()
                .get_element_by_id("score")
                .unwrap_throw()
                .dyn_into::<HtmlElement>()
                .unwrap_throw();

            let mut game = game.borrow_mut();
            root_container.set_inner_html(&*game.get_stats());

            let goal_container = document()
                .get_element_by_id("goal")
                .unwrap_throw()
                .dyn_into::<HtmlElement>()
                .unwrap_throw();

            goal_container.set_inner_html(&*game.get_goal());

            match game.result {
                GameResult::Playing => {
                    if !game.is_paused {
                        game.tick();
                        game.render_state(&mut ctx, true);
                    }
                }
                GameResult::Lost => {
                    console::log_1(&format!("result: {:#?}", game.result).into());
                    game.render_lost(&mut ctx);
                }
                GameResult::Won => {
                    console::log_1(&format!("result: {:#?}", game.result).into());
                    game.render_won(&mut ctx);
                }
            }
        });

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
