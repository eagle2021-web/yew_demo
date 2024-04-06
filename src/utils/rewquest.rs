use reqwest::Client;
use yew::{Component, ComponentLink, Html, html, InputData, ShouldRender};
use yew::services::FetchService;

use crate::log_to_console;

pub struct JFrameworkService {
    fetch_service: FetchService,
    link: ComponentLink<Self>,
    input_value: String,
    // 初始化输入框的值
    prefix: Vec<String>,
}

pub enum Msg {
    InputChanged(String),
    // 添加一个新的消息类型来处理输入框的变化
    FetchResponse,
    Ok,
}

impl Component for JFrameworkService {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_service: FetchService::default(),
            input_value: String::new(), // 初始化输入框的值
            prefix: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchResponse => {
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Client::new();
                    let url = "http://localhost:8003/hello"; // 修改为您的实际URL
                    let response = client.get(url).send().await.expect("send err.");

                    if response.status().is_success() {
                        let a = response.text().await.unwrap();
                        log_to_console(&a);
                        // Ok(response.text().await?)
                    } else {
                        // 处理非成功响应，可以根据需要进行处理
                        log_to_console(&format!("Request failed with status: {}", response.status()));
                    };
                });
            }
            Msg::InputChanged(value) => {
                // 更新输入框的值
                self.input_value = value;
                return true; // 需要重新渲染组件
            }
            _ => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    type="text"
                    value=self.input_value.clone() // 将输入框的值绑定到组件的状态中
                    oninput=self.link.callback(|e: InputData| Msg::InputChanged(e.value.clone())) // 处理输入框变化事件
                />
                <button class="custom-button" onclick=self.link.callback(|_| Msg::FetchResponse )>{ "Fetch Data" }</button>
            <div>{&self.input_value}</div>
            </div>
        }
    }
}

