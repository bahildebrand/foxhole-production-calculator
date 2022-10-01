mod resource_selector;

use std::collections::HashSet;

use crate::resource_selector::ResourceSelection;

use foxhole_production_calculator::ResourceGraph;
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
    #[allow(dead_code)]
    resource_graph: ResourceGraph<'static>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            resource_graph: ResourceGraph::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::CalculationClicked(args) => {
                let output = self.resource_graph.calculate_factory_requirements(
                    args.material,
                    args.rate,
                    HashSet::new(),
                );

                log::info!("{:#?}", output);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let calculation_callback = link.callback(AppMsg::CalculationClicked);
        html! {
            <div class="columns is-centered is-multiline">
                <div class="column is-full">
                    <div class="box">
                        <p><ResourceSelection {calculation_callback}/></p>
                    </div>
                </div>
                <div class="column is-half">
                    <div class="box">
                        <p>{"Structures"}</p>
                    </div>
                </div>
                <div class="column is-half">
                    <div class="box">
                        <p>{"Inputs"}</p>
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
