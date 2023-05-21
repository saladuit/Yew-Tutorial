use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::CustomButton;
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html! {
        <form>
            <TextInput name="name"/>
            <CustomButton label="Submit"/>
        </form>
    }
}
