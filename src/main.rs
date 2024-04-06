// 用于流的扩展方法
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Blob, BlobPropertyBag, Url, window};
use yew::prelude::*;
use yew::web_sys::console;

use crate::components::button::AntButton;

use crate::utils::reader_util::ReaderUtil;
use crate::utils::rewquest::JFrameworkService;

mod models;
mod utils;
mod components;

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
                    match ReaderUtil::fetch_and_log("http://localhost:8003/reactive-stream22").await {
                        Ok(s) => {
                            link.send_message(Msg::DownloadFile("filename.txt".into(), s));
                        }
                        Err(_) => {}
                    }
                });
            }
            Msg::DataChunk(_chunk) => {

            }
            Msg::DataComplete => {

            }
            Msg::Error(_error) => {

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
                        <div>
                <AntButton text="Click me" on_click=self.link.callback(|_| Msg::FetchData)/>
                <JFrameworkService/>
            </div>
            </>
        }
    }
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
