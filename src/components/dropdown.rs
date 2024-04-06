use wasm_bindgen::JsCast;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::events::ChangeData;
use crate::log_to_console;

pub struct Dropdown {
    link: ComponentLink<Self>,
    selected_option: String,
}

pub enum Msg {
    OptionSelected(String),
}

impl Component for Dropdown {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Dropdown {
            link,
            selected_option: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OptionSelected(option) => {
                log_to_console(&option);
                self.selected_option = option;
                true // 需要重新渲染组件
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false // 不需要重新渲染组件
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <select onchange=self.link.callback(|e: ChangeData| {
                    if let ChangeData::Select(elem) = e {
                    //HtmlSelectElement
                        // 获取选中的选项的值
                        let idx = elem.selected_index();
                        log_to_console(&format!("{}", idx));
                        let option = elem.item(idx as u32).unwrap();
                        // log_to_console(&format!("{}", idx));
                        let node_text = option.dyn_into::<web_sys::HtmlOptionElement>()
                            .ok()
                            .map_or_else(|| "".to_string(), |o| o.value());
                        Msg::OptionSelected(node_text)
                    } else {
                        Msg::OptionSelected("".to_string())
                    }
                })>
                    <option value="" disabled=true selected=true hidden=true>{ "Select an option" }</option>
                    <option value="option1">{ "Option 1111" }</option>
                    <option value="option2">{ "Option 2222" }</option>
                    <option value="option3">{ "Option 3333" }</option>
                </select>
                <div>{ &self.selected_option }</div> // 显示选择的选项的值
            </div>
        }
    }
}
