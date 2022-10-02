mod components;

use std::collections::{HashMap, HashSet};

use crate::components::{
    CostDisplay, CustomInputs, InputDisplay, ResourceSelection, StructureDisplay,
};

use foxhole_production_calculator::{FactoryRequirementsBuilding, ResourceGraph};
use foxhole_production_calculator_types::Material;
use yew::prelude::*;

pub struct CalculationClickedArgs {
    pub material: Material,
    pub rate: u64,
}

enum AppMsg {
    CalculationClicked(CalculationClickedArgs),
}

struct App {
    resource_graph: ResourceGraph<'static>,
    buildings: Vec<FactoryRequirementsBuilding>,
    inputs: HashMap<Material, f32>,
    build_cost: HashMap<Material, u64>,
    power: f32,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            resource_graph: ResourceGraph::default(),
            buildings: Vec::new(),
            inputs: HashMap::new(),
            build_cost: HashMap::new(),
            power: 0.0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::CalculationClicked(args) => {
                let reqs = self.resource_graph.calculate_factory_requirements(
                    args.material,
                    args.rate,
                    HashSet::new(),
                );

                log::info!("{:#?}", reqs);
                self.buildings = reqs.buildings;
                self.inputs = reqs.inputs;
                self.power = reqs.power;
                self.build_cost = reqs.build_cost;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let calculation_callback = link.callback(AppMsg::CalculationClicked);

        // FIXME: These clones suck, figure out lifetimes for references later
        let buildings = self.buildings.clone();
        let inputs = self.inputs.clone();
        let build_cost = self.build_cost.clone();
        let power = self.power;
        html! {
            <div class="columns is-centered is-multiline">
                <div class="column is-half">
                    <div class="box">
                        <ResourceSelection {calculation_callback}/>
                    </div>
                </div>
                <div class="column is-half">
                    <div class="box">
                        <CustomInputs/>
                    </div>
                </div>
                <div class="column is-one-third">
                    <div class="box">
                        <StructureDisplay {buildings}/>
                    </div>
                </div>
                <div class="column is-one-third">
                    <div class="box">
                        <InputDisplay {inputs}/>
                    </div>
                </div>
                <div class="column is-one-third">
                    <div class="box">
                        <CostDisplay {power} {build_cost}/>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
