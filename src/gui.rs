use fritiofr_chess::*;
use fritiofr_chess::error::GameApplyMoveError;
use ggez::*;
use ggez::{
    conf,
    event::*,
    glam::*,
    graphics::*,
    Context, GameResult,
    input::{
        keyboard::*,
        keyboard::{KeyCode, KeyMods, KeyInput, KeyboardContext},
        mouse::*,
    },
};
use std::{thread, time};
//use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};

use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::*;

const C_RISE: (u8, u8, u8, u8) = (220, 60, 130, 255);
const SEE_RICE: (u8, u8, u8, u8) = (220, 120, 170, 255);
const royal: (u8, u8, u8, u8) = (230, 200, 20, 255);

impl event::EventHandler<ggez:: GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 2;
        while timer::check_update_time(ctx, DESIRED_FPS) {

        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas;
        if self.D_SEKT {
            canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(C_RISE));
        }else {
            canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        }
        
        if self.check_for_mate(ctx, &mut canvas) {
            canvas.finish(ctx)?;
            return Ok(());
        }
      
        self.draw_board(ctx, &mut canvas);
        self.draw_pieces(ctx, &mut canvas);
        if self.mouse_down{
            self.move_piece(ctx, &mut canvas);
        }
        
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_down = true;
        self.set_pick_pos(y, x);
        self.mouse_x = x;
        self.mouse_y = y;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.set_drop_pos(y, x);
        self.mouse_down = false;
        match self.do_move() {
            Ok(T) => {
                let temp = self.piece_map.get(&self.pick_pos).unwrap();
                self.piece_map.insert(
                    self.drop_pos, 
                    temp.clone(),
                );
                self.piece_map.remove(&self.pick_pos);
            }
            _ => (),
        }
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        xrel: f32,
        yrel: f32,
    ) -> GameResult {
        self.mouse_x = x;
        self.mouse_y = y;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        let DSEKT: HashSet<KeyCode> = vec![KeyCode::D, KeyCode::S, KeyCode::E, KeyCode::K, KeyCode::T].into_iter().collect(); 
        let dif: HashSet<KeyCode> = ctx.keyboard.pressed_keys().clone();
        if dif == DSEKT {
            self.D_SEKT = true;
            self.paint_board = self.build_paint_board(ctx);
            println!("DSEKT!");
        }
        Ok(())
    }
}

pub struct State {
    mouse_down: bool,
    pick_pos: (u32, u32),
    drop_pos: (u32, u32),
    paint_board: Vec<Mesh>,
    game: Game,
    piece_map: HashMap<(u32, u32), Image>,
    mouse_x: f32,
    mouse_y: f32,
    D_SEKT: bool,
    mate_anim_state: bool,
}



impl State {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<State> {
        let mut state = State {
            mouse_down: false,
            pick_pos: (0, 0),
            drop_pos: (0, 0),
            paint_board: Vec::new(),
            game: Game::start_pos(),
            piece_map: Self::build_piece_map(ctx),
            mouse_x: 0.0,
            mouse_y: 0.0,
            D_SEKT: false,
            mate_anim_state: false,
        };
        state.paint_board = state.build_paint_board(ctx);
        Ok(state)
    }

    pub fn check_for_mate(
        &mut self,
        ctx: &mut Context,
        mut canvas: &mut Canvas,
    ) -> bool {
        if self.game.is_checkmate() {
            if self.mate_anim_state {
                *canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(C_RISE));
            }else {
                *canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(royal));
            }
            let mut text = graphics::Text::new("Check mate!");
            match self.game.get_turn() {
                fritiofr_chess::Color::Black => {
                    text = graphics::Text::new("Check mate! \nWhite wins!");
                    
                }
                fritiofr_chess::Color::White => {
                    text = graphics::Text::new(
                        "Check mate! \nBlack wins!"
                    )
                }
            }
            let mut param = DrawParam::new();
            text.set_scale( PxScale{x: 200.0, y: 200.0});
            
            param = param.dest(vec2(350.0, 600.0));
            self.mate_anim_state = !self.mate_anim_state;
            self.paint_board = self.build_paint_board(ctx);
            self.draw_board(ctx, &mut canvas);
            self.draw_pieces(ctx, &mut canvas);
            canvas.draw(&text, param);
            let millis = time::Duration::from_millis(50);
            thread::sleep(millis);
            return true;
        }

        if self.game.is_stalemate() {
            if self.mate_anim_state {
                *canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(C_RISE));
            }else {
                *canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(royal));
            }
            let mut text = graphics::Text::new("Stalemate!");

            let mut param = DrawParam::new();
            text.set_scale( PxScale{x: 200.0, y: 200.0});
            
            param = param.dest(vec2(350.0, 600.0));
            self.mate_anim_state = !self.mate_anim_state;
            self.paint_board = self.build_paint_board(ctx);
            self.draw_board(ctx, &mut canvas);
            self.draw_pieces(ctx, &mut canvas);
            canvas.draw(&text, param);
            let millis = time::Duration::from_millis(50);
            thread::sleep(millis);
            return true;
        }
        false
    }

    pub fn set_pick_pos(&mut self, x: f32, y: f32) {
        let mut x_ = x;
        let mut y_ = y;

        x_ -= 100.0;
        y_ -= 100.0;

        x_ = x_ / 200.0;
        y_ = y_ / 200.0;


        let mut x_ = x_ as u32;
        let mut y_ = y_ as u32;

        self.pick_pos = (y_, x_);
    }

    pub fn set_drop_pos(&mut self, x: f32, y: f32) {
        let mut x_ = x;
        let mut y_ = y;

        x_ -= 100.0;
        y_ -= 100.0;

        x_ = x_ / 200.0;
        y_ = y_ / 200.0;

        let mut x_ = x_ as u32;
        let mut y_ = y_ as u32;

        self.drop_pos = (y_, x_);
    }

    pub fn draw_pieces(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
    ) {
        let mut param = graphics::DrawParam::new();

        for (key, value) in &self.piece_map {
            let transform = &self.build_transform(ctx, key.0, key.1, 1, 1);
            param = param.scale(transform.0);
            param = param.dest(transform.1);
            if key == &(
                self.pick_pos.0 as u32
                , self.pick_pos.1 as u32) && self.mouse_down {
                continue;
            }
            canvas.draw(
                self.piece_map.get(&key).unwrap()
                ,param
            );
        }
        
    }

    pub fn build_piece_map(
        ctx: &Context
    ) -> HashMap<(u32, u32), Image> {
        let mut map: HashMap<(u32, u32), Image> = HashMap::new();
        
        map.insert((0, 0), graphics::Image::from_path(ctx, "/b_rook_2x_ns.png").unwrap());
        map.insert((7, 0), graphics::Image::from_path(ctx, "/b_rook_2x_ns.png").unwrap());

        map.insert((1, 0), graphics::Image::from_path(ctx, "/b_knight_2x_ns.png").unwrap());
        map.insert((6, 0), graphics::Image::from_path(ctx, "/b_knight_2x_ns.png").unwrap());

        map.insert((2, 0), graphics::Image::from_path(ctx, "/b_bishop_2x_ns.png").unwrap());
        map.insert((5, 0), graphics::Image::from_path(ctx, "/b_bishop_2x_ns.png").unwrap());

        map.insert((3, 0), graphics::Image::from_path(ctx, "/b_queen_2x_ns.png").unwrap());
        map.insert((4, 0), graphics::Image::from_path(ctx, "/b_king_2x_ns.png").unwrap());

        for i in 0..8 {
            map.insert((i,1), graphics::Image::from_path(ctx, "/b_pawn_2x_ns.png").unwrap());  
        }

        for i in 0..8 {
            map.insert((i,6), graphics::Image::from_path(ctx, "/w_pawn_2x_ns.png").unwrap());  
        }

        map.insert((3, 7), graphics::Image::from_path(ctx, "/w_queen_2x_ns.png").unwrap());  
        map.insert((4, 7), graphics::Image::from_path(ctx, "/w_king_2x_ns.png").unwrap());  

        map.insert((2, 7), graphics::Image::from_path(ctx, "/w_bishop_2x_ns.png").unwrap());  
        map.insert((5, 7), graphics::Image::from_path(ctx, "/w_bishop_2x_ns.png").unwrap());  
        
        map.insert((1, 7), graphics::Image::from_path(ctx, "/w_knight_2x_ns.png").unwrap());  
        map.insert((6, 7), graphics::Image::from_path(ctx, "/w_knight_2x_ns.png").unwrap());  

        map.insert((0, 7), graphics::Image::from_path(ctx, "/w_rook_2x_ns.png").unwrap());  
        map.insert((7, 7), graphics::Image::from_path(ctx, "/w_rook_2x_ns.png").unwrap());
        map
    }

    pub fn do_move(
        &mut self,
    ) -> Result::<(), GameApplyMoveError> {
        let mut moves = self.game.gen_moves(self.pick_pos.0 as usize, self.pick_pos.1 as usize);
        match moves {
            None => {
                return Err(GameApplyMoveError::InvalidMove);
            },
            _ => (),
        }
        let mut moves = moves.unwrap();
        let mv = moves.iter().find(|mv| mv.to() == (self.drop_pos.0 as usize, self.drop_pos.1 as usize));
        if let Some(mv) = mv {
            return self.game.apply_move(*mv);
        }
        Err(GameApplyMoveError::InvalidMove)
    }

    pub fn move_piece(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) {
        match self.game.get_board().get_tile(self.pick_pos.0 as usize, self.pick_pos.1 as usize) {
            None => (),
            _ => {
                canvas.draw(
                    self.piece_map.get(&self.pick_pos).unwrap()
                    , graphics::DrawParam::new().scale(vec2(0.2, 0.2)).dest(vec2(self.mouse_x, self.mouse_y))
                );
            }
        }
    }

    pub fn draw_board(
        &self, 
        ctx: &mut Context, 
        canvas: &mut graphics::Canvas,
    ) {
        let mut param = graphics::DrawParam::new();
        for i in 0..8 {
            param = param.dest(vec2(35.0, 170.0 + 200.0 * i as f32));
            let c = (65 + i) as u8 as char;
            let c = c.to_string();
            let mut c = Text::new(c);
            c.set_scale(PxScale{x: 50.0, y: 50.0});
            canvas.draw(&c, param);
        }
        for i in 0..8 {
            param = param.dest(vec2(190.0 + 200.0 * i as f32, 1730.0));
            let c = (49 + i) as u8 as char;
            let c = c.to_string();
            let mut c = Text::new(c);
            c.set_scale(PxScale{x: 50.0, y: 50.0});
            canvas.draw(&c, param);
        }
        for i in &self.paint_board {
            canvas.draw(i, Vec2::new(0.0, 0.0));
        }
    }

    pub fn build_transform(
        &self,
        ctx: &ggez::Context,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    ) -> (glam::Vec2, glam::Vec2) {
        let scale = glam::Vec2::new(
            0.2,
            0.2,
        );
        let (x, y) = (y, x);
        let pos = glam::Vec2::new(
            (y as i32 * 200 - 60 + 200) as f32,
            (x as i32 * 200 - 65 + 200) as f32,
        );
        (scale, pos)
    }


    pub fn build_paint_board(&self, ctx: &mut Context) -> Vec<Mesh> {
        let mut vec: Vec<Mesh> = Vec::new();
        let mut c = graphics::Color::WHITE;
        let mut c_ = graphics::Color::BLACK;
        if(self.D_SEKT || self.game.is_checkmate() || self.game.is_stalemate()) {
            c_ = graphics::Color::from(SEE_RICE);
            c = graphics::Color::from(royal);
            if self.mate_anim_state {
                c = graphics::Color::from(C_RISE);
                c_ = graphics::Color::from(royal);
            }
            
        }
        for i in 0..64 {
            let mut clr = c;
            if (i / 8) % 2 == 0 {
                if i % 2 == 0 {
                    clr = c_;
                }
            }else if i % 2 != 0 {
                clr = c_;
            }
            vec.push(
                graphics::Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    graphics::Rect {
                        x: (i % 8) as f32 * 200.0 + 100.0,
                        y: (i / 8) as f32 * 200.0 + 100.0,
                        w: 200.0,
                        h: 200.0,
                    },
                    clr, 
                ).unwrap()
            );
        }
        vec
    }
}