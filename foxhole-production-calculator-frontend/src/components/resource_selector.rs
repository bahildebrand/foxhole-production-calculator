use std::string::ToString;
use std::{collections::HashMap, str::FromStr};

use foxhole_production_calculator_types::Material;
use itertools::sorted;
use strum::IntoEnumIterator;
use web_sys::{HtmlInputElement, HtmlSelectElement, InputEvent};
use yew::prelude::*;

pub enum ResourceSelectionMsg {
    OutputAdded,
    OutputCardRemoved(Material),
    OutputCardRateChange((Material, u64)),
}

#[derive(Clone, PartialEq, Properties)]
pub struct ResourceSelectionProps {
    pub calculation_callback: Callback<HashMap<Material, u64>>,
}

pub struct ResourceSelection {
    material_ref: NodeRef,
    outputs: HashMap<Material, u64>,
}

impl ResourceSelection {
    fn get_material(&self) -> Option<Material> {
        let material_element = self.material_ref.cast::<HtmlSelectElement>();

        material_element.map(|element| Material::from_str(&element.value()).unwrap())
    }
}

impl Component for ResourceSelection {
    type Message = ResourceSelectionMsg;
    type Properties = ResourceSelectionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            material_ref: NodeRef::default(),
            outputs: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            ResourceSelectionMsg::OutputCardRemoved(material) => {
                self.outputs.remove(&material);
                props.calculation_callback.emit(self.outputs.clone());

                true
            }
            ResourceSelectionMsg::OutputCardRateChange((material, rate)) => {
                {
                    let entry = self.outputs.entry(material).or_insert(rate);
                    *entry = rate;
                }
                props.calculation_callback.emit(self.outputs.clone());

                true
            }
            ResourceSelectionMsg::OutputAdded => {
                if let Some(material) = self.get_material() {
                    self.outputs.insert(material, 1);
                    props.calculation_callback.emit(self.outputs.clone());

                    true
                } else {
                    log::error!("Can't parse material");

                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let outputs = &self.outputs;
        let remove_callback = link.callback(ResourceSelectionMsg::OutputCardRemoved);
        let rate_change_callback = link.callback(ResourceSelectionMsg::OutputCardRateChange);

        // Remove materials from list if already present
        let material_list = sorted(Material::iter().filter_map(|material| {
            if !outputs.contains_key(&material) {
                Some(material.to_string())
            } else {
                None
            }
        }))
        .collect::<Vec<String>>();

        html! {
            <div class="container">
            <label class="label">{ "Resource Output" }</label>
            <div class="field is-grouped">
                <div class="control">
                    <div class="select">
                        <select ref={self.material_ref.clone()}>
                            {
                                material_list.iter().map(|material| {
                                    html! { <option> { format!("{}", material) } </option> }
                                }).collect::<Html>()
                            }
                        </select>
                    </div>
                </div>
                <div class="control">
                <button class="button" onclick={link.callback(|_| ResourceSelectionMsg::OutputAdded)}>
                    <span class="icon">
                        <i class="fa-solid fa-circle-plus"></i>
                    </span>
                </button>
                </div>
            </div>
            {
                outputs.iter().map(|(material, rate)| {
                    html! {
                        <OutputCard
                            material={*material}
                            rate={*rate}
                            remove_callback={remove_callback.clone()}
                            rate_change_callback={rate_change_callback.clone()}/>
                    }
                }).collect::<Html>()
            }
            </div>
        }
    }
}

enum OutputCardMsg {
    RemoveOutput,
    RateChanged,
}

#[derive(Clone, PartialEq, Properties)]
struct OutputCardProps {
    material: Material,
    rate: u64,
    remove_callback: Callback<Material>,
    rate_change_callback: Callback<(Material, u64)>,
}

struct OutputCard {
    input_ref: NodeRef,
}

impl Component for OutputCard {
    type Message = OutputCardMsg;
    type Properties = OutputCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            OutputCardMsg::RemoveOutput => {
                let material = ctx.props().material;
                let callback = &ctx.props().remove_callback;

                callback.emit(material);

                true
            }
            OutputCardMsg::RateChanged => {
                let material = ctx.props().material;
                let callback = &ctx.props().rate_change_callback;
                let input_element = self.input_ref.cast::<HtmlInputElement>();
                if let Some(input_element) = input_element {
                    let input_string = input_element.value();
                    if let Ok(rate) = input_string.parse::<u64>() {
                        input_element.set_class_name("input");

                        callback.emit((material, rate));

                        false
                    } else {
                        input_element.set_placeholder("Invalid input");
                        input_element.set_class_name("input is-danger");

                        false
                    }
                } else {
                    log::error!("Couldn't find input element");

                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let material = ctx.props().material;
        let rate = ctx.props().rate;

        html! {
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">{material}</p>
                    <input class="input" type="number" min="0" placeholder="Unit/Hour" value={format!("{}", rate)}
                        ref={self.input_ref.clone()}
                        oninput={link.callback(|_: InputEvent| OutputCardMsg::RateChanged)}/>
                    <button class="button" onclick={link.callback(|_| OutputCardMsg::RemoveOutput)}>
                        <span class="icon">
                            <i class="fa-solid fa-circle-xmark"></i>
                        </span>
                    </button>
                </header>
            </div>
        }
    }
}
