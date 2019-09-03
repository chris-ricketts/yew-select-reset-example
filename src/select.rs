
use yew::callback::Callback;
use yew::html::{ChangeData, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::macros::{html, Properties};

use log::warn;
use std::fmt::{Debug, Formatter, Error};

/// `Select` component.
pub struct Select<T> {
    props: Props<T>,
}

/// Internal message of the component.
pub enum Msg {
    /// This message indicates the option with id selected.
    Selected(Option<usize>),
}

/// Properties of `Select` component.
#[derive(PartialEq, Properties)]
pub struct Props<T> {
    /// Initially selected value.
    pub selected: Option<T>,
    /// Disabled the component's selector.
    pub disabled: bool,
    /// Options are available to choose.
    pub options: Vec<T>,
    /// Callback to handle changes.
    #[props(required)]
    pub onchange: Callback<T>,
}

impl <T: Debug> Debug for Props<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Props")
            .field("selected", &self.selected)
            .field("disabled", &self.disabled)
            .field("options", &self.options)
            .finish()
    }
}

impl<T> Component for Select<T>
    where
        T: PartialEq + Clone + 'static + Debug,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(value) => {
                if let Some(idx) = value {
                    let item = self.props.options.get(idx - 1).cloned();
                    if let Some(value) = item {
                        self.props.onchange.emit(value);
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        warn!("Props changed for select {:?}", self.props);
        true
    }
}

impl<T> Renderable<Select<T>> for Select<T>
    where
        T: ToString + PartialEq + Clone + Debug + 'static,
{
    fn view(&self) -> Html<Self> {
        log::debug!("Select re-rendering now");
        let selected = self.props.selected.as_ref();
        let view_option = |value: &T| {
            let flag = selected == Some(value);
            html! {
                <option selected=flag value={value.to_string()}>{ value.to_string() }</option>
            }
        };
        html! {
            <select disabled=self.props.disabled
                    onchange=|event| {
                        match event {
                            ChangeData::Select(elem) => {
                                let value = elem.selected_index().map(|x| x as usize);
                                Msg::Selected(value)
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                    }>
                <option disabled=true selected=selected.is_none()>
                    { "↪" }
                </option>
                { for self.props.options.iter().map(view_option) }
            </select>
        }
    }
}


