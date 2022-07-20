mod logic;
mod rendering;
mod utils;
mod random;
mod constants;
mod ball;
mod quadtree;
mod geometry;

use logic::{GameState};

use std::cell::RefCell;
use std::rc::Rc;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ console, KeyboardEvent, MouseEvent};

use crate::logic::GameResult;
use crate::rendering::write_text;
use crate::utils::{get_context, get_debug_context, request_animation_frame, window};


thread_local! {
    static GAME: Rc<RefCell<GameState>> = Rc::new(
        RefCell::new(
            GameState::new(
                constants::WIDTH,
                constants::HEIGHT))
    );


    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> =
    Closure::wrap(Box::new(|evt: KeyboardEvent| GAME.with(|game| {
      let play_pause = match &evt.key()[..] {
        "G" | "g" => true,
        _ => false,
      };

      if play_pause {
            game.borrow_mut().pause_play();
            console::log_1(&format!("{} {} ", evt.key(), game.borrow().is_paused).into());
      }
    })) as Box<dyn FnMut(KeyboardEvent)>);


    static HANDLE_MOUSE: Closure<dyn FnMut(MouseEvent)> =
    Closure::wrap(Box::new(|evt: MouseEvent| GAME.with(|game| {
        let clicked = match &evt.button() {
            0 => true,
            _ => false,
            };
        if clicked {
            game.borrow_mut().create_capture_ball(evt.client_x() as f64,evt.client_y() as f64);
            //console::log_1(&format!("pressed {} {} {}", evt.button(), evt.client_x(), evt.client_y()).into());
        }

      }
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
                handle_mousedown.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });


    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let ctx = get_context();
    let debug_ctx = get_debug_context();


    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        GAME.with(|game|
            {
                let mut game = game.borrow_mut();
                if game.result == GameResult::Lost {
                    let _ = f.borrow_mut().take();
                    return;
                }
                if !game.is_paused
                {
                    game.tick();
                    game.render_state(&ctx, true);
                    game.render_quad_tree(&ctx, false);

                    //game.render_debug_ball_quad_info(&debug_ctx);
                }

                // if game.is_paused && game.is_render_debug {
                //     render_quad_tree(&game, &debug_ctx);
                //     game.is_render_debug = false;
                // }
            }
        );

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

