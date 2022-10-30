use std::{cell::RefCell, rc::Rc};

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
                            .map(|root| process_node(root, tree, &active_callback, tree_idx))
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
        html! {
            <ul>
            {
                // Check if node has children
                if arena_node.last_child().is_some() {
                    html! {
                        <li>{enumerate_options(node, tree, active_callback.clone(), tree_idx)}
                            {
                                node_id.children(&tree.arena).map(|child| process_node(&child, tree, active_callback, tree_idx)).collect::<Html>()
                            }
                        </li>
                    }
                } else {
                    html! {
                        <li>{enumerate_options(node, tree, active_callback.clone(), tree_idx)}</li>
                    }
                }
            }
            </ul>
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
    log::debug!("{:#?}", *options_ref);
    options_ref
        .iter()
        .map(|node_id| {
            let node = tree.get_node(*node_id).expect("Node should exist");
            let display_text = format!("{:.4} - {}", node.count(), node.structure_name());

            html! {
                <StructureOptionButton
                    node_id={*node_id}
                    tree_idx={tree_idx}
                    active={node.is_active()}
                    active_callback={active_callback.clone()}
                    display_text={display_text}/>
            }
        })
        .collect::<Html>()
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

        html! {
            <button class="button" onclick={link.callback(|_| StructureOptionButtonMsg::Activated)}>
                {display_text}
            </button>
        }
    }
}
