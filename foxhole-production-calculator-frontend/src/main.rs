use foxhole_production_calculator::ResourceGraph;
use foxhole_production_calculator_types::Material;
use strum::IntoEnumIterator;
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
                        <p><ResourceSelection /></p>
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

struct ResourceSelection {}

impl Component for ResourceSelection {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();
        html! {
            <div class="container">
            <label class="label">{ "Resource Output" }</label>
            <div class="field is-grouped">
                <div class="control">
                    <div class="select">
                        <select>
                            {
                                Material::iter().map(|material| {
                                    html! { <option> { format!("{}", material) } </option> }
                                }).collect::<Html>()
                            }
                        </select>
                    </div>
                </div>
                <div class="control">
                    <input class="input" type="text" placeholder="100"/>
                </div>
            </div>
            <div class="field">
                <div class="control">
                    <button class="button is-link">{ "Calculate" }</button>
                </div>
            </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
