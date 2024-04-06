use yew::format::{Json, Nothing, Format};
use yew::services::FetchService;
use http::Request;
use yew::services::fetch::Response;
use yew::{Component, ComponentLink, Html, html, ShouldRender};
use anyhow::Error;
use crate::log_to_console;

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
            // log_to_console(&url);
            let get_request = Request::get("http://localhost:8003/hello")
                .header("Connection", "keep-alive")
                .body(Nothing)
                .expect("Failed to build request.");
            let link = self.link.clone();
            let callback = link.callback(|response: Response<Result<String, Error>>| {
                log_to_console(&format!("response = {} ", response.status()));
                if response.status().is_success() {
                    log_to_console("ok");
                    return Msg::Ok;
                } else {
                    log_to_console("NOT ok");
                    return Msg::Ok;
                }
            });
            let task = FetchService::fetch(get_request, callback);
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
