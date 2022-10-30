use std::collections::HashMap;

use foxhole_production_calculator_types::Material;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputDisplayProps {
    pub inputs: HashMap<Material, f32>,
}

pub struct InputDisplay {}

impl Component for InputDisplay {
    type Message = ();
    type Properties = InputDisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut inputs: Vec<(String, f32)> = ctx
            .props()
            .inputs
            .iter()
            .map(|(material, cost)| (material.to_string(), *cost))
            .collect();
        inputs.sort_by(|a, b| a.0.cmp(&b.0));

        html! {
            <div class="container">
                <label class="label">{ "Inputs:" }</label>
                <table class="table">
                    <thead>
                        <th>{"Material"}</th>
                        <th>{"Count"}</th>
                    </thead>
                    <tbody>
                        {
                            inputs.iter().map(|(material, count)| {
                                html! {
                                    <tr>
                                        <td>{material.to_string()}</td>
                                        <td>{format!("{}", count)}</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                        <tr>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
    }
}
