use std::collections::HashSet;

use foxhole_production_calculator_types::Material;
use strum::IntoEnumIterator;
use yew::prelude::*;

pub enum CustomInputsMsg {
    NewInput,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CustomInputsProps {}

pub struct CustomInputs {
    custom_inputs: HashSet<Material>,
}

impl Component for CustomInputs {
    type Message = CustomInputsMsg;
    type Properties = CustomInputsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            custom_inputs: HashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CustomInputsMsg::NewInput => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="container">
            <label class="label">{ "Outside Inputs:" }</label>
            {
                self.custom_inputs.iter().map(|input| {
                    {input}
                }).collect::<Html>()
            }
            <CustomInputSelector />
            <button class="button" onclick={link.callback(|_| CustomInputsMsg::NewInput)}>
                <span class="icon">
                    <i class="fa-solid fa-circle-plus"></i>
                </span>
            </button>
            </div>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct CustomInputSelectorProps {}

#[derive(Default)]
struct CustomInputSelector {
    material_ref: NodeRef,
}

impl Component for CustomInputSelector {
    type Message = ();
    type Properties = CustomInputSelectorProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            material_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="field">
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
            </div>
        }
    }
}
