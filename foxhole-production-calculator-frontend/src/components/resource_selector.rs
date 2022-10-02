use std::{collections::HashMap, str::FromStr};

use foxhole_production_calculator_types::Material;
use strum::IntoEnumIterator;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

pub enum ResourceSelectionMsg {
    Calculate,
    InputChanged,
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
    input_ref: NodeRef,
    button_ref: NodeRef,
    rate: u64,
    outputs: HashMap<Material, u64>,
}

impl ResourceSelection {
    fn get_material(&self) -> Option<Material> {
        let material_element = self.material_ref.cast::<HtmlSelectElement>();

        material_element.map(|element| Material::from_str(&element.value()).unwrap())
    }

    fn get_rate(&self) -> Option<u64> {
        let rate_element = self.input_ref.cast::<HtmlInputElement>();

        rate_element
            .map(|element| element.value().parse().ok())
            .flatten()
    }
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
            outputs: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ResourceSelectionMsg::Calculate => {
                let material_element = self.material_ref.cast::<HtmlSelectElement>();
                if let Some(material_element) = material_element {
                    // This should always match
                    let material = Material::from_str(&material_element.value()).unwrap();

                    ctx.props().calculation_callback.emit(self.outputs.clone());

                    true
                } else {
                    false
                }
            }
            ResourceSelectionMsg::InputChanged => {
                let button_element = self.button_ref.cast::<HtmlButtonElement>();
                let input_element = self.input_ref.cast::<HtmlInputElement>();
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
            ResourceSelectionMsg::OutputCardRemoved(material) => {
                self.outputs.remove(&material);

                true
            }
            ResourceSelectionMsg::OutputCardRateChange((material, rate)) => {
                let entry = self.outputs.entry(material).or_insert(rate);
                *entry = rate;

                true
            }
            ResourceSelectionMsg::OutputAdded => {
                if let (Some(material), Some(rate)) = (self.get_material(), self.get_rate()) {
                    self.outputs.insert(material, rate);

                    true
                } else {
                    log::error!("Can't parse output");

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
                <div class="control">
                <button class="button" onclick={link.callback(|_| ResourceSelectionMsg::OutputAdded)}>
                    <span class="icon">
                        <i class="fa-solid fa-circle-plus"></i>
                    </span>
                </button>
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
                    if let Ok(rate) = input_element.value().parse::<u64>() {
                        input_element.set_class_name("input");

                        callback.emit((material, rate));

                        false
                    } else {
                        input_element.set_placeholder("Invalid input");
                        input_element.set_class_name("input is-danger");

                        true
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
                        onchange={link.callback(|_| OutputCardMsg::RateChanged)}/>
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
