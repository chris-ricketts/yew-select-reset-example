use log::debug;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use yew::components::Select;
use yew::prelude::*;

const SELECT_OPTION_STRINGS: [&'static str; 3] = ["apple", "pear", "orange"];
const INITIAL_OPTION: &'static str = "apple";

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
    selected_option: Option<SelectOption>,
    selected_enum_option: Option<EnumOption>,
}

enum Msg {
    ChangeOption(SelectOption),
    Reset,
    ChangeEnumOption(EnumOption),
    ResetEnum,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            selected_option: Some(SelectOption(String::from(INITIAL_OPTION))),
            selected_enum_option: Some(EnumOption::Apple),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeOption(option) => {
                debug!("Option changed: {:?}", option);
                self.selected_option = Some(option);
            }
            Msg::Reset => {
                debug!("Resetting Select");
                self.selected_option = Some(SelectOption(String::from(INITIAL_OPTION)))
            }
            Msg::ChangeEnumOption(option) => self.selected_enum_option = Some(option),
            Msg::ResetEnum => self.selected_enum_option = Some(EnumOption::Apple),
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <Select<SelectOption>
                        selected=self.selected_option.clone(),
                        onchange=Msg::ChangeOption,
                        options=select_options()
                        />
                    <button onclick=|_| Msg::Reset >{ "RESET" }</button>
                </div>
                <div>
                    <Select<EnumOption>
                        selected=self.selected_enum_option,
                        onchange=Msg::ChangeEnumOption,
                        options=EnumOption::iter().collect::<Vec<_>>()
                        />
                    <button onclick=|_| Msg::ResetEnum >{ "RESET" }</button>
                </div>
            </div>
        }
    }
}

fn select_options() -> Vec<SelectOption> {
    SELECT_OPTION_STRINGS
        .iter()
        .map(|s| SelectOption(s.to_string()))
        .collect()
}

fn main() {
    web_logger::init();
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
