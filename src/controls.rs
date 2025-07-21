use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ControlState {
    pub fit_screen: bool,
    pub show_overlay: bool,
    pub opacity: f64,
}

impl ControlState {
    pub fn new() -> Self {
        ControlState {
            fit_screen: false,
            show_overlay: true,
            opacity: 1.0,
        }
    }
}

pub enum ControlsAction {
    ToggleFitScreen,
    ToggleOverlay,
    SetOpacity(f64),
}

impl Reducible for ControlState {
    type Action = ControlsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new: ControlState = self.as_ref().clone();

        match action {
            ControlsAction::ToggleFitScreen => new.fit_screen = !new.fit_screen,
            ControlsAction::ToggleOverlay => new.show_overlay = !new.show_overlay,
            ControlsAction::SetOpacity(opacity) => new.opacity = opacity,
        }

        new.into()
    }
}

#[function_component(Controls)]
pub fn controls() -> Html {
    let controls = use_context::<UseReducerHandle<ControlState>>().expect("ControlState not found");

    let on_toggle = {
        let store = controls.clone();
        Callback::from(move |_| store.dispatch(ControlsAction::ToggleFitScreen))
    };

    let on_opacity_input = {
        let store = controls.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let v = input.value_as_number();
            store.dispatch(ControlsAction::SetOpacity(v));
        })
    };

    html! {
        <div class="controls">
            <input type="checkbox" checked={controls.fit_screen} onchange={on_toggle} /> <label>{"Fit to Screen"}</label>
            <input
                id="opacity"
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={controls.opacity.to_string()}
                oninput={on_opacity_input}
              />
        </div>
    }
}
