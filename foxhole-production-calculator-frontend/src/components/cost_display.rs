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
        let build_cost = &ctx.props().build_cost;

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
                                        <td>{format!("{}", material)}</td>
                                        <td>{format!("{}", cost)}</td>
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
