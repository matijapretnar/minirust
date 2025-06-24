use sauron::{html::text, node, wasm_bindgen, Application, Cmd, Node, Program};

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
            <section class="section">
            <div class="container">
              <div class="field has-addons">
              <p class="control">
              <button class="button" on_click=|_| { Msg::Reset }>
                  <span>Reset</span>
                </button>
              </p>
              <p class="control">
                  <button class="button" on_click=|_| { Msg::Decrement }>
                  <span>Step -</span>
                </button>
              </p>
              <p class="control">
                <button class="button" on_click=|_| { Msg::Increment }>
                  <span>Step +</span>
                </button>
              </p>
            </div>
                <ul>
                {
                    for frame in self.stacks[self.index].frames.iter() {
                        node! {
                            <div class="box">
                            <table class="table is-fullwidth"> {
                                for (x, v) in frame.iter() {
                                    node! {
                                    <tr>
                                        <th>{ text(x) }</th>
                                        <td>{ text(v) }</td>
                                    </tr>}
                                }
                            }
                            </table>
                            </div>
                        }
                    }
                }
                </ul>
        </div>
          </section>
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
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(App::new());
}
