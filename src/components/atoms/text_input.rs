use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    html! {
        <div>
        <input type="text" name={props.name.clone()}/>
        </div>
    }
}
