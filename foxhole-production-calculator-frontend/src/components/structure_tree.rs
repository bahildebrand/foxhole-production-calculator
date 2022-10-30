use foxhole_production_calculator::StructureTree;
use indextree::NodeId;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct StructureDisplayProps {
    pub trees: Vec<StructureTree>,
}

pub struct StructureTreeDisplay {}

impl Component for StructureTreeDisplay {
    type Message = ();
    type Properties = StructureDisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let trees = &ctx.props().trees;
        log::debug!("{:#?}", trees);

        html! {
        <div class="content">
            {
                trees
                    .iter()
                    .map(|tree| {
                        tree.roots
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|root| process_node(root, tree))
                            .collect::<Html>()
                    })
                    .collect::<Html>()
            }
        </div>
        }
    }
}

fn process_node(node_id: &NodeId, tree: &StructureTree) -> Html {
    let arena_node = tree.get_arena_node(*node_id).expect("Node should exist");
    let node = arena_node.get();

    if node.is_active() {
        html! {
            <ul>
            {
                // Check if node has children
                if arena_node.last_child().is_some() {
                    html! {
                        <li>{format!("{:.4}", node.count())} {" - "} {node.structure_name()}
                            {
                                node_id.children(&tree.arena).map(|child| process_node(&child, tree)).collect::<Html>()
                            }
                        </li>
                    }
                } else {
                    html! {
                        <li>{format!("{:.4}", node.count())} {" - "} {node.structure_name()}</li>
                    }
                }
            }
            </ul>
        }
    } else {
        html! {}
    }
}
