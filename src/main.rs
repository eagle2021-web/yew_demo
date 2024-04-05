use yew::prelude::*;

enum Msg {
    AddOne,
    ToggleColor,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    color: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            color: "red".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::ToggleColor => {
                self.color = if self.color == "red" { "green".into() } else { "red".into() };
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style="display: flex; justify-content: center; align-items: center; height: 100vh;">
                <div style=format!("width: 100px; height: 100px; background-color: {}", self.color)>
                    <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                    <button onclick=self.link.callback(|_| Msg::ToggleColor)>{ "Toggle Color" }</button>
                    <p>{ self.value }</p>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
