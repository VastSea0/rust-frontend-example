use gloo::console;
use yew::{html, Component, Context, Html, Properties, Callback, InputEvent, TargetCast};
use web_sys::HtmlInputElement;

pub enum Msg {
    Increment,
    Decrement,
    Add,
    UpdateInput(String),
    Remove(usize),
}

pub struct App {
    value: i64,
    todos: Vec<String>,
    input_value: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            todos: vec![],
            input_value: "".to_string(),

        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one");
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
            Msg::Add => {
                if !self.input_value.is_empty() {
                    self.todos.push(self.input_value.to_string());
                    self.input_value = "".to_string();
                }
                true
            }
            Msg::UpdateInput(new_value) => {
                self.input_value = new_value;
                true
            }
            Msg::Remove(index) => {
                self.todos.remove(index);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateInput(input.value())
        });


        let ondelete = |index| {
            ctx.link().callback(move |_|  Msg::Remove(index))
        };


        html! {
            <div class="app">
                <div class="panel">

                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>


                    <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>




                    <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                        { "+1, +1" }
                    </button>

                </div>


                <p class="counter">
                    { self.value }

                </p>
                <p>
                    {
                        if self.value == 40 {
                         html! {
                           <center>
                            <p><b>{ "HELO" }</b></p>
                            </center>
                            }
                    }else {
                        html! {}
                    }
                    }
                </p>


                <div class="todo">
                    <input
                        type="text"
                        placeholder="To-Do Ekle"
                        value={self.input_value.clone()}
                        oninput={oninput}
                    />
                    <button onclick={ctx.link().callback(|_| Msg::Add)}>
                        { "Ekle" }
                    </button>
                </div>
                 <ul>
                    { for self.todos.iter().enumerate().map(|(index, todo)| html! {

                        <li class="row" >
                            { index } {" | "} { todo }
                            <button onclick={ondelete(index)}>{"x"}</button>
                        </li>


                    }) }
                </ul>
                <p class="footer">


                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}