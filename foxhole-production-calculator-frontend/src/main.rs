mod components;

use std::collections::{HashMap, HashSet};

use crate::components::{
    CostDisplay, CustomInputs, InputDisplay, ResourceSelection, StructureDisplay,
    StructureTreeDisplay,
};

use foxhole_production_calculator::{FactoryRequirementsBuilding, ResourceGraph, StructureTree};
use foxhole_production_calculator_types::Material;
use yew::prelude::*;

enum AppMsg {
    Calculate(HashMap<Material, u64>),
    CustomInputsUpdate(HashSet<Material>),
}

struct App {
    resource_graph: ResourceGraph<'static>,
    custom_inputs: HashSet<Material>,
    buildings: Vec<FactoryRequirementsBuilding>,
    outputs: HashMap<Material, u64>,
    inputs: HashMap<Material, f32>,
    build_cost: HashMap<Material, u64>,
    power: f32,
    trees: Vec<StructureTree>,
}

impl App {
    fn update_reqs(&mut self) {
        let trees = self
            .resource_graph
            .calculate_factory_requirements(self.outputs.clone(), self.custom_inputs.clone());
        let reqs = self
            .resource_graph
            .factory_requirements_from_trees(&trees, self.custom_inputs.clone());

        self.buildings = reqs.buildings;
        self.inputs = reqs.inputs;
        self.power = reqs.power;
        self.build_cost = reqs.build_cost;
        self.trees = trees;
    }
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            resource_graph: ResourceGraph::default(),
            custom_inputs: HashSet::new(),
            buildings: Vec::new(),
            outputs: HashMap::new(),
            inputs: HashMap::new(),
            build_cost: HashMap::new(),
            power: 0.0,
            trees: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Calculate(outputs) => {
                self.outputs = outputs;

                self.update_reqs();
            }
            AppMsg::CustomInputsUpdate(inputs) => {
                self.custom_inputs = inputs;

                self.update_reqs();
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let calculation_callback = link.callback(AppMsg::Calculate);
        let custom_inputs_callback = link.callback(AppMsg::CustomInputsUpdate);

        // FIXME: These clones suck, figure out lifetimes for references later
        let mut buildings = self.buildings.clone();
        buildings.sort();
        let inputs = self.inputs.clone();
        let build_cost = self.build_cost.clone();
        let power = self.power;
        let trees = self.trees.clone();
        html! {
            <div class="container">
            <section class="hero is-primary">
                <div class="hero-body">
                    <p class="title">{"Foxhole Production Calculator"}</p>
                    <p class="subtitle">{"Calculate requirements for facilities"}</p>
                </div>
            </section>
            <section class="section">
                <div class="columns is-centered is-multiline">
                    <div class="column is-half">
                        <div class="box">
                            <ResourceSelection {calculation_callback}/>
                        </div>
                    </div>
                    <div class="column is-half">
                        <div class="box">
                            <CustomInputs callback={custom_inputs_callback}/>
                        </div>
                    </div>
                    <div class="column is-full">
                        <div class="box">
                            <StructureTreeDisplay {trees}/>
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
            </section>
            <footer class="footer">
                <div class="content has-text-centered">
                    {"Foxhole is a registered trademark of "}<a href="https://www.siegecamp.com/">{"Siege Camp"}</a>
                    {"    |    "}
                    <a href="https://github.com/bahildebrand/foxhole-production-calculator"><i class="fa-brands fa-github"></i></a>
                </div>
            </footer>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
