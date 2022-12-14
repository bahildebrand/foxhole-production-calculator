use std::collections::HashMap;

use foxhole_production_calculator_types::Material;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CostDisplayProps {
    pub power: f32,
    pub build_cost: HashMap<Material, u64>,
}

pub struct CostDisplay {}

impl Component for CostDisplay {
    type Message = ();
    type Properties = CostDisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let power = ctx.props().power;
        let mut build_cost: Vec<(String, u64)> = ctx
            .props()
            .build_cost
            .iter()
            .map(|(material, cost)| (material.to_string(), *cost))
            .collect();
        build_cost.sort();

        html! {
            <div class="container">
                <label class="label">{ format!("Power: {}", power) }</label>
                <label class="label">{ "Build Costs:" }</label>
                <table class="table">
                    <thead>
                        <th>{"Material"}</th>
                        <th>{"Cost"}</th>
                    </thead>
                    <tbody>
                        {
                            build_cost.iter().map(|(material, cost)| {
                                html! {
                                    <tr>
                                        <td>{ material.to_string() }</td>
                                        <td>{ cost.to_string() }</td>
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
