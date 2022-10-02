use std::collections::HashSet;
use std::str::FromStr;

use foxhole_production_calculator_types::Material;
use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub enum CustomInputsMsg {
    NewInput,
    RemoveInput(Material),
}

#[derive(Clone, PartialEq, Properties)]
pub struct CustomInputsProps {
    pub callback: Callback<HashSet<Material>>,
}

pub struct CustomInputs {
    custom_inputs: HashSet<Material>,
    material_ref: NodeRef,
}

impl Component for CustomInputs {
    type Message = CustomInputsMsg;
    type Properties = CustomInputsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            custom_inputs: HashSet::new(),
            material_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CustomInputsMsg::NewInput => {
                let material_element = self.material_ref.cast::<HtmlSelectElement>();

                match material_element {
                    Some(material_element) => {
                        let material = Material::from_str(&material_element.value()).unwrap();
                        self.custom_inputs.insert(material);

                        let update_callback = &ctx.props().callback;
                        update_callback.emit(self.custom_inputs.clone());

                        true
                    }
                    None => {
                        log::error!("Can't find material element");

                        false
                    }
                }
            }
            CustomInputsMsg::RemoveInput(material) => {
                self.custom_inputs.remove(&material);

                let update_callback = &ctx.props().callback;
                update_callback.emit(self.custom_inputs.clone());

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let custom_inputs = &self.custom_inputs;
        let remove_callback = link.callback(CustomInputsMsg::RemoveInput);

        html! {
            <div class="container">
            <label class="label">{ "Outside Inputs:" }</label>
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
                <button class="button" onclick={link.callback(|_| CustomInputsMsg::NewInput)}>
                    <span class="icon">
                        <i class="fa-solid fa-circle-plus"></i>
                    </span>
                </button>
            </div>
            </div>
            {
                custom_inputs.iter().map(|material| {
                    html! {
                        <CustomInputCard material={*material} remove_callback={remove_callback.clone()}/>
                    }
                }).collect::<Html>()
            }
            </div>
        }
    }
}

enum CustomInputCardMsg {
    RemoveMaterial,
}

#[derive(Clone, PartialEq, Properties)]
struct CustomInputCardProps {
    pub material: Material,
    pub remove_callback: Callback<Material>,
}

struct CustomInputCard {}

impl Component for CustomInputCard {
    type Message = CustomInputCardMsg;
    type Properties = CustomInputCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CustomInputCardMsg::RemoveMaterial => {
                let remove_callback = &ctx.props().remove_callback;
                let material = ctx.props().material;

                remove_callback.emit(material);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let material = &ctx.props().material;
        let link = ctx.link();

        html! {
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{material}</p>
                    <button class="button" onclick={link.callback(|_| CustomInputCardMsg::RemoveMaterial)}>
                        <span class="icon">
                            <i class="fa-solid fa-circle-xmark"></i>
                        </span>
                    </button>
                </header>
            </div>
        }
    }
}
