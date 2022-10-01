use foxhole_production_calculator::ResourceGraph;
use yew::prelude::*;

struct Model {
    #[allow(dead_code)]
    resource_graph: ResourceGraph<'static>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            resource_graph: ResourceGraph::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();
        html! {
            <div class="columns is-centered is-multiline">
                <div class="column is-full">
                    <div class="box">
                        <p>{"Output Selection"}</p>
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
    yew::start_app::<Model>();
}
