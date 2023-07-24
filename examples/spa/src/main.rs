use gloo::console;
use js_sys::Date;
use puccinia_s_checkmate::mgen::Move;
use puccinia_s_checkmate::openings::*;
use puccinia_s_checkmate::val::*;
use puccinia_s_checkmate::Game;
use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    Step,
    NewGame,
}

pub struct App {
    value: i64,
    status: String,
    game: Game, // chess game
    log: Vec<Move>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            status: String::from("Fancy a nice game of chess?"),
            game: Game::new(ROOT_BOARD),
            log: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Step => {
                self.value += 1;
                console::log!("step");
                let moves = self.game.legal_moves(self.log.last());
                let l = self.game.score_moves(&moves, 100000, 25, false);
                if l.len() > 0 {
                    let (m, _score) = l[0];
                    self.game.make_move(m);
                    self.log.push(m);
                }
                self.status = match self.game.turn() {
                    WHITE => String::from("White's Turn"),
                    BLACK => String::from("Black's Turn"),
                };
                console::log!("step");
                true // Return true to cause the displayed change to update
            }
            Msg::NewGame => {
                self.game = Game::new(ROOT_BOARD);
                self.status = String::from("Let's play chess");
                self.log.retain(|_| false);
                console::log!("new game");
                true // true=>update display
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut vec = Vec::new();
        for y in (0..8).rev() {
            for x in (0..8).rev() {
                let i = x * 8 + y;
                let mut col = "white";
                if (7 - i / 8) % 2 == i % 2 {
                    col = "black"
                }
                let sq = match self.game.board[i] {
                    K1 => "\u{2654}",
                    Q1 => "\u{2655}",
                    R1 => "\u{2656}",
                    B1 => "\u{2657}",
                    N1 => "\u{2658}",
                    P1 => "\u{2659}",
                    K2 => "\u{265a}",
                    Q2 => "\u{265b}",
                    R2 => "\u{265c}",
                    B2 => "\u{265d}",
                    N2 => "\u{265e}",
                    P2 => "\u{265f}",
                    _ => "",
                };
                vec.push(html! {<div class={col}>{sq}</div>});
            }
        }

        html! {
            <div>
                <div class="chessboard">
                    {vec}
                </div>

                <div class="panel">
                    <button class="button" onclick={ctx.link().callback(|_| Msg::NewGame)}>
                        { "New Game" }
                    </button>

                    <button class="button" onclick={ctx.link().callback(|_| Msg::Step)}>
                        { "Step" }
                    </button>

                    //<button class="button" onclick={ctx.link().batch_callback(|_| vec![Msg::NewGame, Msg::Step])}>
                    //    { "+1, +1" }
                    //</button>
                </div>

                <p class="status">
                    { self.status.clone()}
                </p>

                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
