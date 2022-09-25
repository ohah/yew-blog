use std::rc::Rc;
use yew::{html, ChildrenWithProps, Component, Context, Html, Properties, function_component};

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemProps {
    value: String,
}

pub struct ListItem;

impl Component for ListItem {
    type Message = ();
    type Properties = ListItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <span>
                { ctx.props().value.clone() }
            </span>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

pub struct List;
impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {{
            for ctx.props().children.iter().map(|mut item| {
                let mut props = Rc::make_mut(&mut item.props);
                props.value = format!("item-{}", props.value);
                item
            })
        }}
    }
}
#[function_component(Test)]
pub fn test() -> Html {
  html! {
    <List>
      <ListItem value="a" />
      <ListItem value="b" />
      <ListItem value="c" />
    </List>
  }
}