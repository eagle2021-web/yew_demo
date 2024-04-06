use yew::prelude::*;

pub struct AntButton {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: String,
    // 添加其他属性
    pub on_click: Callback<()>, // 定义一个回调函数，用于处理点击事件
}

pub enum Msg {
    ButtonClicked,
}

impl Component for AntButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        AntButton { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonClicked => {
                // 当按钮被点击时，调用回调函数处理点击事件
                self.props.on_click.emit(());
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let class = Classes::from("ant-button"); // 使用Classes类型来处理样式类
        html! {
            <button onclick=self.link.callback(|_| Msg::ButtonClicked) class=class>
                { &self.props.text }
            </button>
        }
    }
}
