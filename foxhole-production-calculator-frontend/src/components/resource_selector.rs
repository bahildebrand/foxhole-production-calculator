use std::str::FromStr;

use foxhole_production_calculator_types::Material;
use strum::IntoEnumIterator;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::CalculationClickedArgs;

pub enum ResourceSelectionMsg {
    Calculate,
    InputChanged,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ResourceSelectionProps {
    pub calculation_callback: Callback<CalculationClickedArgs>,
}

pub struct ResourceSelection {
    material_ref: NodeRef,
    input_ref: NodeRef,
    button_ref: NodeRef,
    rate: u64,
}

impl Component for ResourceSelection {
    type Message = ResourceSelectionMsg;
    type Properties = ResourceSelectionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            material_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            button_ref: NodeRef::default(),
            rate: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ResourceSelectionMsg::Calculate => {
                let material_element = self.material_ref.cast::<HtmlSelectElement>();
                if let Some(material_element) = material_element {
                    // This should always match
                    let material = Material::from_str(&material_element.value()).unwrap();

                    let callback_args = CalculationClickedArgs {
                        material,
                        rate: self.rate,
                    };
                    ctx.props().calculation_callback.emit(callback_args);

                    true
                } else {
                    false
                }
            }
            ResourceSelectionMsg::InputChanged => {
                log::info!("Changed");
                let input_element = self.input_ref.cast::<HtmlInputElement>();
                let button_element = self.button_ref.cast::<HtmlButtonElement>();
                if let (Some(input_element), Some(button_element)) = (input_element, button_element)
                {
                    if let Ok(rate) = input_element.value().parse::<u64>() {
                        self.rate = rate;
                        button_element.set_disabled(false);
                        input_element.set_class_name("input");

                        false
                    } else {
                        input_element.set_placeholder("Invalid input");
                        input_element.set_class_name("input is-danger");
                        button_element.set_disabled(true);

                        true
                    }
                } else {
                    log::error!("Could not find resource selection input element");
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
            <label class="label">{ "Resource Output" }</label>
            <div class="field is-grouped">
                <div class="control">
                    <div class="select">
                        <select ref={self.material_ref.clone()}>
                            {
                                Material::iter().map(|material| {
                                    html! { <option> { format!("{}", material) } </option> }
                                }).collect::<Html>()
                            }
                        </select>
                    </div>
                </div>
                <div class="control">
                    <input class="input" type="text" placeholder="Unit/Hour"
                        ref={self.input_ref.clone()}
                        onchange={link.callback(|_| ResourceSelectionMsg::InputChanged)}/>
                </div>
            </div>
            <div class="field">
                <div class="control">
                    <button
                        class="button is-link"
                        ref={self.button_ref.clone()}
                        onclick={link.callback(|_| ResourceSelectionMsg::Calculate)}
                        >
                        { "Calculate" }
                    </button>
                </div>
            </div>
            </div>
        }
    }
}
