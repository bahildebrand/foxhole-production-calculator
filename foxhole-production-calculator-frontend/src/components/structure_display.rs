use foxhole_production_calculator::FactoryRequirementsBuilding;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct StructureDisplayProps {
    pub buildings: Vec<FactoryRequirementsBuilding>,
}

pub struct StructureDisplay {}

impl Component for StructureDisplay {
    type Message = ();
    type Properties = StructureDisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let buildings = &ctx.props().buildings;

        html! {
            <div class="container">
                <label class="label">{ "Structures" }</label>
                <table class="table">
                    <thead>
                        <th>{"Structure"}</th>
                        <th>{"Upgrade"}</th>
                        <th>{"Count"}</th>
                    </thead>
                    <tbody>
                        {
                            buildings.iter().map(|building| {
                                html! {
                                    <tr>
                                        <td>{format!("{}", building.building)}</td>
                                        <td>{format!("{}", building.upgrade.clone().unwrap_or_else(|| "N/A".to_string()))}</td>
                                        <td>{format!("{}", building.count)}</td>
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
