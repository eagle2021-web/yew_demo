use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::Nothing;
use yew::web_sys::console;

enum Msg {
    AddOne,
    ToggleColor,
    FetchData,
    FetchSuccess(String), // Define a message variant to handle successful fetch response
    FetchFailed, // Define a message variant to handle failed fetch response
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    color: String,
    fetch_task: Option<FetchTask>, // Store the fetch task to be able to cancel it if needed
}

fn log_to_console(message: &str) {
    console::log_1(&message.into());
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            color: "red".into(),
            fetch_task: None,
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
            Msg::FetchData => {
                // Construct the request
                let request = Request::get("http://localhost:8003/stu")
                    .body(Nothing)
                    .expect("Failed to build request.");

                // Send the request and store the task
                let task = FetchService::fetch(request, self.link.callback(|response: Response<Result<String, _>>| {
                    if response.status().is_success() {
                        Msg::FetchSuccess(response.into_body().unwrap())
                    } else {
                        Msg::FetchFailed
                    }
                })).expect("Failed to start request");

                self.fetch_task = Some(task);
                false // Do not trigger a re-render immediately
            }
            Msg::FetchSuccess(response) => {
                // Handle successful fetch response here
                // For example, you can log the response or update the state
                log::info!("Fetch success! Response: {}", response);
                log_to_console(&format!("Fetch success! Response: {}", response));
                true // Trigger a re-render
            }
            Msg::FetchFailed => {
                // Handle failed fetch response here
                // For example, you can log the failure or show an error message
                log::error!("Fetch failed!");
                true // Trigger a re-render
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    // 在 update 方法中输出信息


    // 在 view 方法中的按钮点击回调中调用 log_to_console 方法
    fn view(&self) -> Html {
        html! {
            <div style="display: flex; justify-content: center; align-items: center; height: 100vh;">
                <div style=format!("width: 100px; height: 100px; background-color: {}", self.color)>
                    <button class="custom-button" onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                    <button onclick=self.link.callback(|_| Msg::ToggleColor)>{ "Toggle Color" }</button>
                    <button onclick=self.link.callback(|_| {
                        log_to_console("Fetch Data button clicked");
                        Msg::FetchData
                    })>{ "Fetch Data" }</button>
                    <p>{ self.value }</p>
                </div>
            </div>
        }
    }
}

fn main() {
    // https://yew.rs/zh-Hans/docs/0.18.0/getting-started/build-a-sample-app
    yew::start_app::<Model>();
}
