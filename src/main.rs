use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
//use yew::components::Select;
use crate::select::Select;
use yew::prelude::*;

mod select;


#[derive(Clone, Debug, PartialEq)]
struct SelectOption(String);

// Very contrived example! Real life example could be if the API expected/returned values like
// "SHIPPING_US_EXPRESS" but you wanted to display this as "US Express" and you had some logic in the
// ToString to do this conversion.
impl ToString for SelectOption {
    fn to_string(&self) -> String {
        self.0.to_uppercase()
    }
}

#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
enum EnumOption {
    Apple,
    Pear,
    Orange,
}

struct Model {
    selected_enum_option: Option<EnumOption>,
}

enum Msg {
    ChangeEnumOption(EnumOption),
    ResetEnum,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            selected_enum_option: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeEnumOption(option) => self.selected_enum_option = Some(option),
            Msg::ResetEnum => self.selected_enum_option = Some(EnumOption::Apple)
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        (html! {
            <div>
                <div>
                    <Select<EnumOption>
                        selected=self.selected_enum_option,
                        onchange=Msg::ChangeEnumOption,
                        options=EnumOption::iter().collect::<Vec<_>>()
                        />
                    // FIXME After clicking reset, the select item should be set to Apple,
                    // but after the second time this is done, it remains on whatever was last clicked
                    // Notably, the DOM as represented by the inspector panel in firefox shows the correct state of the model, just that the page doesn't render it.
                    <button onclick=|_| Msg::ResetEnum >{ "RESET" }</button>
                </div>
            </div>
        })
    }
}


fn main() {
    web_logger::init();
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}




