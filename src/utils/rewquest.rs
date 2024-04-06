use yew::format::{Json, Nothing, Format};
use yew::services::FetchService;
use http::Request;
use yew::services::fetch::Response;
use yew::{Component, ComponentLink, Html, html, ShouldRender};
use anyhow::Error;
use crate::log_to_console;
use http::Uri;
use reqwest::{Client};
use crate::utils::reader_util::ReaderUtil;

pub struct JFrameworkService {
    fetch_service: FetchService,
    link: ComponentLink<Self>,
}

pub enum Msg {
    FetchResponse(String),
    Ok
}

impl Component for JFrameworkService {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_service: FetchService::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg { Msg::FetchResponse(url) => {
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
            _ => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
                <button class="custom-button" onclick=self.link.callback(|_| Msg::FetchResponse("https://my.api/v1/resource1111".to_string()))>{ "1222222" }</button>
             // <AntButton on_click=self.link.callback(|_| Msg::FetchResponse("https://my.api/v1/resource1111".to_string())) text="aaaaaa">
        }
    }
}
