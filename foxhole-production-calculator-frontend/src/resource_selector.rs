use std::str::FromStr;

use foxhole_production_calculator_types::Material;
use strum::IntoEnumIterator;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::CalculationClickedArgs;

pub enum ResourceSelectionMsg {
    Calculate,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ResourceSelectionProps {
    pub calculation_callback: Callback<CalculationClickedArgs>,
}

pub struct ResourceSelection {
    material_ref: NodeRef,
    input_ref: NodeRef,
}

impl Component for ResourceSelection {
    type Message = ResourceSelectionMsg;
    type Properties = ResourceSelectionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            material_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ResourceSelectionMsg::Calculate => {
                let input_element = self.input_ref.cast::<HtmlInputElement>();
                let material_element = self.material_ref.cast::<HtmlSelectElement>();
                if let (Some(input_element), Some(material_element)) =
                    (input_element, material_element)
                {
                    // FIXME: actually handle this, currently will crash on bad user input
                    let rate = input_element.value().parse::<u64>().unwrap();
                    // This should always match, should probably handle this better
                    let material = Material::from_str(&material_element.value()).unwrap();

                    let callback_args = CalculationClickedArgs { material, rate };
                    ctx.props().calculation_callback.emit(callback_args);

                    true
                } else {
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
                    // FIXME: Make input only take positive integers
                    <input class="input" type="text" placeholder="100" ref={self.input_ref.clone()}/>
                </div>
            </div>
            <div class="field">
                <div class="control">
                    <button
                        class="button is-link"
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
