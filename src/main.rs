use std::fmt::format;
use std::fs;
use futures::StreamExt; // 用于流的扩展方法
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, ReadableStream, ReadableStreamDefaultReader, TextDecoder, window, BlobPropertyBag, Blob, Url};
use yew::prelude::*;
use js_sys::{Reflect, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::web_sys::console;
use serde::{Serialize, Deserialize};
use crate::models::ChatCompletionData;

mod models;
mod utils;

pub enum Msg {
    FetchData,
    DataChunk(Vec<u8>),
    DataComplete,
    Error(String),
    DownloadFile(String, String), // 新增消息类型
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
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let window = web_sys::window().expect("no global `window` exists");
                    match fetch_and_log("http://localhost:8003/reactive-stream22").await {
                        Ok(s) => {
                            link.send_message(Msg::DownloadFile("filename.txt".into(), s));
                        }
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
            Msg::DownloadFile(file_name, content) => {
                download_file(&file_name, &content);
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
pub async fn fetch_and_log(url: &str) -> Result<String, JsValue> {
    let window = web_sys::window().unwrap();
    let resp = JsFuture::from(window.fetch_with_str(url)).await?.dyn_into::<Response>()?;
    let reader = resp.body().ok_or("no body")?.get_reader();
// 将`reader`（原本是`Object`类型）转换为`JsValue`。
    let reader_js_value = JsValue::from(reader);
    let mut reader = ReadableStreamDefaultReader::from(reader_js_value);
    let mut bytes = Vec::new();
    let mut all = String::new();
    loop {
        let result = JsFuture::from(reader.read()).await?;
        let value = Reflect::get(&result, &JsValue::from_str("value"))?;
        let comp = Reflect::get(&result, &JsValue::from_str("done"));
        if let Ok(js_val) = comp {
            if js_val.as_bool().unwrap() {
                console::log_1(&"Reading completed".into());
                break;
            }
        } else {
            console::log_1(&"Error reading stream".into());
        }
        let chunk = Uint8Array::new(&value);
        bytes.extend_from_slice(&chunk.to_vec());
        let mut tmp_bytes = Vec::new();
        tmp_bytes.extend_from_slice(&chunk.to_vec());
        let s = String::from_utf8_lossy(&tmp_bytes);
        // let decoded_str = text_decoder.decode().unwrap();
        log_to_console("---------");
        log_to_console(&format!("Chunk: {}", s));
        let text = String::from_utf8_lossy(&bytes);
        for each in text.split("data:").into_iter() {
            let js:serde_json::error::Result<ChatCompletionData> = serde_json::from_str(each);
            if js.is_ok() {
                let obj = js.unwrap();
                log_to_console(&format!("obj = {:?}", obj));
            } else {
                log_to_console("------------------------------");
                log_to_console(each);
                log_to_console("---------1111");
            }
        }
    }

    let text = String::from_utf8(bytes).map_err(|_| JsValue::from_str("Error converting to string"))?;
    // log_to_console(&text);
    // fs::write("a.txt", text).expect("write err");
    // Example of sending a message once data is complete, adjusting as per your actual Msg type and link handling
    // self.link.send_message(Msg::DataComplete);

    Ok(text)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn download_file(file_name: &str, content: &str) {
    let window = window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    // 将String转换成JsValue
    let mut opts = BlobPropertyBag::new();
    opts.type_("text/plain");
    let content_array = js_sys::Array::new();
    content_array.push(&JsValue::from_str(content));

    // 创建Blob
    match Blob::new_with_str_sequence_and_options(&content_array, &opts) {
        Ok(blob) => {
            let blob_url = Url::create_object_url_with_blob(&blob).expect("Failed to create blob URL");

            // 创建一个临时的a标签用于下载
            let a = document.create_element("a").expect("Failed to create element");
            a.set_attribute("href", &blob_url).expect("Failed to set href");
            a.set_attribute("download", file_name).expect("Failed to set download attribute");
            a.set_attribute("style", "display: none;").expect("Failed to set style");

            document.body().expect("Document should have a body").append_child(&a).expect("Failed to append child");

            // 触发下载
            let a = a.dyn_ref::<web_sys::HtmlElement>().expect("Failed to cast to HtmlElement");
            a.click();

            // 清理
            a.remove();
            Url::revoke_object_url(&blob_url).expect("Failed to revoke blob URL");
        },
        Err(e) => log(&format!("Error creating blob: {:?}", e)),
    };
}
// Additional helper functions and stream processing should be implemented here.
fn main() {
    // https://yew.rs/zh-Hans/docs/0.18.0/getting-started/build-a-sample-app
    yew::start_app::<PostRequestComponent>();
}
