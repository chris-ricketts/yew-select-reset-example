use log::debug;
use yew::components::Select;
use yew::prelude::*;

const SELECT_OPTION_STRINGS: [&'static str; 3] = ["apple", "pear", "orange"];
const INITIAL_OPTION: &'static str = "apple";

#[derive(Clone, PartialEq)]
struct SelectOption(String);

// Very contrived example! Real life example could be if the API expected/returned values like
// "SHIPPING_US_EXPRESS" but you wanted to display this as "US Express" and you had some logic in the
// ToString to do this conversion.
impl ToString for SelectOption {
    fn to_string(&self) -> String {
        self.0.to_uppercase()
    }
}

struct Model {
    selected_option: String,
}

enum Msg {
    ChangeOption(SelectOption),
    Reset,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            selected_option: String::from(INITIAL_OPTION),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeOption(SelectOption(option)) => {
                debug!("Option changed: {}", option);
                self.selected_option = option;
            }
            Msg::Reset => {
                debug!("Resetting Select");
                self.selected_option = String::from(INITIAL_OPTION)
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <Select<SelectOption>
                    selected=Some(SelectOption(self.selected_option.clone()))
                    onchange=Msg::ChangeOption,
                    options=select_options()
                    />
                <button onclick=|_| Msg::Reset >{ "RESET" }</button>
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
