use futures::StreamExt; // 用于流的扩展方法
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, ReadableStream};
use yew::prelude::*;
use js_sys::{Uint8Array};
use yew::web_sys::console;
pub enum Msg {
    FetchData,
    DataChunk(Vec<u8>),
    DataComplete,
    Error(String),
}

pub struct PostRequestComponent {
    link: ComponentLink<Self>,
    data: String,
}
fn log_to_console(message: &str) {
    console::log_1(&message.into());
}
impl Component for PostRequestComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            data: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                println!("resp_value = {:?}", "callback");
                let callback = self.link.callback(|_: Msg| Msg::FetchData);
                callback.emit(Msg::FetchData); // Trigger the fetch
                // Initiate the fetch process
                wasm_bindgen_futures::spawn_local(async move {
                    let window = web_sys::window().expect("no global `window` exists");
                    match JsFuture::from(window.fetch_with_str("http://localhost:8003/reactive-stream22")).await {
                        Ok(resp_value) => {
                            let resp: Response = resp_value.dyn_into().unwrap();
                            if resp.ok() {
                                let text = JsFuture::from(resp.text().unwrap()).await.unwrap();
                                let text_str = text.as_string().unwrap();
                                // 假设我们直接将整个响应体作为一个数据块处理
                                self.link.send_message(Msg::DataChunk(text_str.into_bytes()));
                                self.link.send_message(Msg::DataComplete);
                            } else {
                                self.link.send_message(Msg::Error("Failed to fetch".to_string()));
                            }
                        },
                        Err(error) => {
                            self.link.send_message(Msg::Error(error.as_string().unwrap_or_else(|| "Unknown error".to_string())));
                        }
                    }
                });
            }
            Msg::DataChunk(chunk) => {
                // Handle a chunk of data
                // let decoder = TextDecoder::new_with_label("utf-8").unwrap();
                // let text = decoder.decode_with_u8_array(&chunk).unwrap();
                // self.data.push_str(&text);
            }
            Msg::DataComplete => {
                // Handle completion of data streaming
            }
            Msg::Error(error) => {
                // Handle error
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::FetchData)>{ "Fetch Data" }</button>
                <div>{ &self.data }</div>
            </>
        }
    }
}

// Additional helper functions and stream processing should be implemented here.
fn main() {
    // https://yew.rs/zh-Hans/docs/0.18.0/getting-started/build-a-sample-app
    yew::start_app::<PostRequestComponent>();
}
