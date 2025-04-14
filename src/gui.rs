use sauron::{
    html::text, html::units::px, jss, node, wasm_bindgen, Application, Cmd, Node, Program,
};

use crate::state;

enum Msg {
    Increment,
    Decrement,
    Reset,
}

struct App {
    index: usize,
    stacks: Vec<state::Stack>,
}

impl App {
    fn new() -> Self {
        let mut state = crate::State::new();
        let stmt = crate::Statement::fibonacci(10);
        stmt.run(&mut state);
        App {
            index: 0,
            stacks: state.stacks(),
        }
    }
}

impl Application for App {
    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <button class="count" on_click=|_|{Msg::Reset} >Reset</button>
                <input type="button"
                    value="-"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
                <ul>
                {
                    for frame in self.stacks[self.index].frames.iter() {
                        node! {
                            <li> {text(frame)} </li>
                        }
                    }
                }
                </ul>
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Increment => self.index += 1,
            Msg::Decrement => self.index -= 1,
            Msg::Reset => self.index = 0,
        }
        Cmd::none()
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body":{
                font_family: "verdana, arial, monospace",
            },

            "main":{
                width: px(30),
                height: px(100),
                margin: "auto",
                text_align: "center",
            },

            "input, .count":{
                font_size: px(40),
                padding: px(30),
                margin: px(30),
            }
        }]
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(App::new());
}
