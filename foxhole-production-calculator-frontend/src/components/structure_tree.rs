use std::{cell::RefCell, collections::HashSet, rc::Rc};

use foxhole_production_calculator::{StructureTree, StructureTreeNode};
use indextree::NodeId;
use yew::prelude::*;

pub enum StructureTreeDisplayMsg {
    ActiveChanged((usize, NodeId)),
}

#[derive(Clone, PartialEq, Properties)]
pub struct StructureTreeDisplayProps {
    pub trees: Rc<RefCell<Vec<StructureTree>>>,
    pub tree_update_callback: Callback<()>,
}

pub struct StructureTreeDisplay {}

impl Component for StructureTreeDisplay {
    type Message = StructureTreeDisplayMsg;
    type Properties = StructureTreeDisplayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StructureTreeDisplayMsg::ActiveChanged((tree_idx, node_id)) => {
                let props = ctx.props();
                let mut trees = props.trees.borrow_mut();

                trees[tree_idx].activate_node(node_id);

                props.tree_update_callback.emit(());

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let trees = &ctx.props().trees;
        let active_callback = ctx.link().callback(StructureTreeDisplayMsg::ActiveChanged);

        html! {
        <div class="content">
            {
                trees
                    .borrow()
                    .iter()
                    .enumerate()
                    .map(|(tree_idx, tree)| {
                        tree.roots
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|root| {
                                html! {
                                    <ul>
                                        { process_node(root, tree, &active_callback, tree_idx) }
                                    </ul>
                                }
                            })
                            .collect::<Html>()
                    })
                    .collect::<Html>()
            }
        </div>
        }
    }
}

fn process_node(
    node_id: &NodeId,
    tree: &StructureTree,
    active_callback: &Callback<(usize, NodeId)>,
    tree_idx: usize,
) -> Html {
    let arena_node = tree.get_arena_node(*node_id).expect("Node should exist");
    let node = arena_node.get();

    if node.is_active() {
        {
            // Check if node has children
            if arena_node.last_child().is_some() {
                html! {
                    <li><div class="buttons has-addons m0" style="margin: 0;">{format!("{}: ", node.output())}{enumerate_options(node, tree, active_callback.clone(), tree_idx)}</div>
                        <ul style="margin-bottom: 0; margin-top: 0;">
                        {
                            node_id.children(&tree.arena).map(|child| process_node(&child, tree, active_callback, tree_idx)).collect::<Html>()
                        }
                        </ul>
                    </li>
                }
            } else {
                html! {
                    <li>
                        <div class="buttons has-addons m0">{format!("{}: ", node.output())}{enumerate_options(node, tree, active_callback.clone(), tree_idx)}</div>
                    </li>
                }
            }
        }
    } else {
        html! {}
    }
}

fn enumerate_options(
    node: &StructureTreeNode,
    tree: &StructureTree,
    active_callback: Callback<(usize, NodeId)>,
    tree_idx: usize,
) -> Html {
    let options = node.options();
    let options_ref = options.borrow();
    let mut name_set = HashSet::new();

    html! {
        {
            options_ref
            .iter()
            .filter_map(|node_id| {
                let node = tree.get_node(*node_id).expect("Node should exist");
                let display_text = format!("{:.3} - {}", node.count(), node.structure_name());

                if !name_set.contains(&display_text) {
                    name_set.insert(display_text.clone());

                    Some(html! {
                        <StructureOptionButton
                        node_id={*node_id}
                        tree_idx={tree_idx}
                        active={node.is_active()}
                        active_callback={active_callback.clone()}
                        display_text={display_text}/>
                    })
                } else {
                    None
                }
            })
            .collect::<Html>()
        }
    }
}

pub enum StructureOptionButtonMsg {
    Activated,
}

#[derive(Clone, PartialEq, Properties)]
pub struct StructureOptionButtonProps {
    pub node_id: NodeId,
    pub tree_idx: usize,
    pub active: bool,
    pub active_callback: Callback<(usize, NodeId)>,
    pub display_text: String,
}

pub struct StructureOptionButton {}

impl Component for StructureOptionButton {
    type Message = StructureOptionButtonMsg;
    type Properties = StructureOptionButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StructureOptionButtonMsg::Activated => {
                let props = ctx.props();
                let node_id = props.node_id;
                let callback = &props.active_callback;
                let tree_idx = props.tree_idx;

                callback.emit((tree_idx, node_id));

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let display_text = &ctx.props().display_text;
        let class = if ctx.props().active {
            "button is-rounded is-primary is-small"
        } else {
            "button is-rounded is-small"
        };

        html! {
            <button class={class} onclick={link.callback(|_| StructureOptionButtonMsg::Activated)}>
                {display_text}
            </button>
        }
    }
}
