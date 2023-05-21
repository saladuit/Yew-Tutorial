use yew::prelude::*;
use crate::components::molecules::custom_form::CustomForm;
use gloo::console::log;
// use serde::{Serialize, Deserialize};
use stylist::{yew::styled_component, Style};

mod components;

use components::atoms::main_title::{MainTitle, Color};

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();

    html! {
        <div class ={stylesheet}>
        <MainTitle title="Hi there" color={Color::Normal} on_load={main_title_load}/>
        <CustomForm/>
        // <MainTitle title="Visitor" color={Color::Ok}/>
        // <MainTitle title="Dead" color={Color::Error}/>
        <p class={css!("color: red; font-size: 75px;")}>{"My name I Saladin Afoh"}</p>
        </div>
    }
}

// fn list_to_html(list: Vec<&str>) -> Vec<Html> {
//     list.iter().map(|item| html!{<li>{item}</li>}).collect()
// }
