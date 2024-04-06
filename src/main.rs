use futures::StreamExt; // 用于流的扩展方法
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, ReadableStream, ReadableStreamDefaultReader, TextDecoder};
use yew::prelude::*;
use js_sys::{Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
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
                log_to_console("111111");
                println!("resp_value = {:?}", "callback");
                // let callback = self.link.callback(|_: Msg| Msg::FetchData);
                log_to_console("2222");
                // callback.emit(Msg::FetchData); // Trigger the fetch
                log_to_console("3333");
                // Initiate the fetch process
                wasm_bindgen_futures::spawn_local(async move {
                    let window = web_sys::window().expect("no global `window` exists");
                    match fetch_and_log("http://localhost:8003/reactive-stream22").await {
                        Ok(_) => {}
                        Err(_) => {}
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
                <button class="custom-button" onclick=self.link.callback(|_| Msg::FetchData)>{ "Fetch Data" }</button>
                <div>{ &self.data }</div>
            </>
        }
    }
}
#[wasm_bindgen]
pub async fn fetch_and_log(url: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let resp = JsFuture::from(window.fetch_with_str(url)).await?.dyn_into::<Response>()?;
    let reader = resp.body().ok_or("no body")?.get_reader();
// 将`reader`（原本是`Object`类型）转换为`JsValue`。
    let reader_js_value = JsValue::from(reader);
    let mut reader = ReadableStreamDefaultReader::from(reader_js_value);
    let mut bytes = Vec::new();
    let text_decoder = TextDecoder::new().unwrap();

    loop {
        let result = JsFuture::from(reader.read()).await?;
        // let a = result.as_string().unwrap();
        // log_to_console("{:?}", result);
        let value = js_sys::Reflect::get(&result, &JsValue::from_str("value"))?;

        if js_sys::Reflect::get(&result, &JsValue::from_str("done"))?.as_bool().unwrap() {
            break;
        }
        let chunk = Uint8Array::new(&value);
        bytes.extend_from_slice(&chunk.to_vec());
        let mut tmp_bytes = Vec::new();
        tmp_bytes.extend_from_slice(&chunk.to_vec());
        let s = String::from_utf8_lossy(&tmp_bytes);
        // let decoded_str = text_decoder.decode().unwrap();
        log_to_console("---------");
        log_to_console(&format!("Chunk: {}", s));
    }

    let text = String::from_utf8(bytes).map_err(|_| JsValue::from_str("Error converting to string"))?;
    // log_to_console(&text);

    // Example of sending a message once data is complete, adjusting as per your actual Msg type and link handling
    // self.link.send_message(Msg::DataComplete);

    Ok(())
}

// Additional helper functions and stream processing should be implemented here.
fn main() {
    // https://yew.rs/zh-Hans/docs/0.18.0/getting-started/build-a-sample-app
    yew::start_app::<PostRequestComponent>();
}
