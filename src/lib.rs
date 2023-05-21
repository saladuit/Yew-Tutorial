use yew::prelude::*;
use gloo::console::log;
use serde::{Serialize, Deserialize};
use stylist::yew::styled_component;


#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let name = "Saladin";
    let my_object = MyObject {
        username: "Saladin".to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("My name is", name);
    log!("My object is", serde_json::to_string_pretty(&my_object).unwrap());
    let class = "MyClasses";
    let message: Option<&str> = None;
    let tasks = vec!("Learn Rust", "Learn Yew", "Make a web app");
    html! {
        <>
        <h1 class={class}>{ "Hello World!!!!" }</h1>
    if class == "MyClasses" {
        <p>{ "This is a paragraph" }</p>
    }
    else {
        <p>{ "This is my first yew app!" }</p>
    }
    if let Some(message) = message {
            <p>{ message }</p>
        } else
        {
            <p>{ "There is no message" }</p>
        }
        <ul>
        {list_to_html(tasks)}
        </ul>
    </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html!{<li>{item}</li>}).collect()
}
