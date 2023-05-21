use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::style;

#[derive (Properties, PartialEq)]

pub struct Properties {
    pub title: String,
    pub color: Color,
    pub on_load: Callback<String>,
}

#[derive (PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Error => "error".to_owned(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Properties) -> Html {
    let stylesheet = style!(r#"
    .normal {
        color: white;
        font-size: 75px;
    }
    .ok {
        color: green;
        font-size: 75px;
    }

    .error {
        color: red;
        font-size: 75px;
    }
    "#
    ).unwrap();

    props.on_load.emit("MainTitle loaded!!!!".to_owned());

    html! {
        <div class={stylesheet}>
        <h1 class={props.color.to_string()}>{&props.title}</h1>
        </div>
    }
}
