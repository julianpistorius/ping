use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService};

pub struct Model {
    value: String,
    console: ConsoleService,
}

pub enum Msg {
    GotInput(String),
    Clicked,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            value: "".into(),
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
            }
            Msg::Clicked => {
                self.console.log(&self.value);
                //CALL THE GET_PAYLOAD.js FUNCTION HERE!!!
                self.value = "".to_string();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <textarea rows=1,
                        value=&self.value,
                        oninput=|e| Msg::GotInput(e.value),
                        placeholder="",>
                    </textarea>
                     <button onclick=|_| Msg::Clicked,>{ "Ping" }</button>
                </div>
                <div>
                    {&self.value}
                </div>
            </div>
        }
    }
}
