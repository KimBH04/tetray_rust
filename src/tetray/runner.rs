use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{RecvTimeoutError, Sender, channel}}, thread::{self, JoinHandle}, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};

use crate::tetray::{game::{Command, Game, GameView}, tetromino::Rotate};

#[derive(Clone, Copy)]
pub enum Msg { HardDrop, SoftDrop, Hold, Quit }

pub struct TetrayRunner {
    game: Arc<Mutex<Game>>,
    sender_opt: Option<Sender<Msg>>,
    handle_opt: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl Drop for TetrayRunner {
    fn drop(&mut self) {
        self.stop();
    }
}

impl TetrayRunner {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        TetrayRunner::new_manual_seed(seed as usize)
    }
    
    pub fn new_manual_seed(seed: usize) -> Self {
        Self {
            game: Arc::new(Mutex::new(Game::new(seed))),
            handle_opt: None,
            sender_opt: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }
        
        let (tx, rx) = channel::<Msg>();
        self.sender_opt = Some(tx);

        let game_arc = Arc::clone(&self.game);
        let running_arc = Arc::clone(&self.running);

        let thread = thread::spawn(move || {
            let period = Duration::from_secs(1);
            let mut deadline = Instant::now() + period;

            loop {
                let wait = deadline.saturating_duration_since(Instant::now());
                
                match rx.recv_timeout(wait) {
                    Err(RecvTimeoutError::Timeout) => {
                        let mut game = game_arc.lock().unwrap();
                        if game.is_ended() {
                            break;
                        }

                        game.update();
                        deadline = Instant::now() + period;
                    },
                    Ok(Msg::HardDrop) | Ok(Msg::SoftDrop) | Ok(Msg::Hold) => {
                        deadline = Instant::now() + period;
                    },
                    Ok(Msg::Quit) | Err(RecvTimeoutError::Disconnected) => {
                        running_arc.store(false, Ordering::SeqCst);
                        break;
                    },
                }
            }
        });

        self.handle_opt = Some(thread);
        self.running.store(true, Ordering::SeqCst);
    }

    pub fn stop(&mut self) {
        if let Some(s) = self.sender_opt.take() {
            s.clone().send(Msg::Quit).unwrap();
        }
        if let Some(h) = self.handle_opt.take() {
            h.join().unwrap();
        }
    }

    pub fn try_move(&mut self, column: i32) {
        if !self.running.load(Ordering::SeqCst) {
            return;
        }

        let mut game = self.game.lock().unwrap();
        _ = game.command(Command::Move { col: column });
    }

    pub fn try_relative_rotation(&mut self, delta: Rotate) {
        if !self.running.load(Ordering::SeqCst) {
            return;
        }

        let mut game = self.game.lock().unwrap();
        _ = game.command(Command::Rotate { rot: delta });
    }

    pub fn hard_drop(&mut self) {
        if !self.running.load(Ordering::SeqCst) {
            return;
        }

        let mut game = self.game.lock().unwrap();
        _ = game.command(Command::HardDrop);
        
        if let Some(s) = &self.sender_opt {
            s.clone().send(Msg::HardDrop).unwrap();
        }
    }

    pub fn soft_drop(&mut self) {
        if !self.running.load(Ordering::SeqCst) {
            return;
        }

        let mut game = self.game.lock().unwrap();
        _ = game.command(Command::SoftDrop);

        if let Some(s) = &self.sender_opt {
            s.clone().send(Msg::SoftDrop).unwrap();
        }
    }

    pub fn hold(&mut self) {
        if !self.running.load(Ordering::SeqCst) {
            return;
        }

        let mut game = self.game.lock().unwrap();
        _ = game.command(Command::Hold);

        if let Some(s) = &self.sender_opt {
            s.clone().send(Msg::Hold).unwrap();
        }
    }

    pub fn get_game_state(&self) -> GameView {
        let game = self.game.lock().unwrap();
        game.view()
    }
}