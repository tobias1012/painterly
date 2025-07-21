use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ControlState {
    pub fit_screen: bool,
    pub show_overlay: bool,
    pub opacity: f64,
    pub image_src: Option<String>,
}

impl ControlState {
    pub fn new() -> Self {
        ControlState {
            fit_screen: false,
            show_overlay: true,
            opacity: 1.0,
            image_src: None,
        }
    }
}

pub enum ControlsAction {
    ToggleFitScreen,
    ToggleOverlay,
    SetOpacity(f64),
    SetImage(String),
}

impl Reducible for ControlState {
    type Action = ControlsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new: ControlState = self.as_ref().clone();

        match action {
            ControlsAction::ToggleFitScreen => new.fit_screen = !new.fit_screen,
            ControlsAction::ToggleOverlay => new.show_overlay = !new.show_overlay,
            ControlsAction::SetOpacity(opacity) => new.opacity = opacity,
            ControlsAction::SetImage(url) => new.image_src = Some(url),
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

    let on_file_upload = {
        let store = controls.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                let url = web_sys::Url::create_object_url_with_blob(&file)
                    .expect("Failed to create object URL");
                store.dispatch(ControlsAction::SetImage(url));
            }
        })
    };

    html! {
        <div class="controls" style="position: fixed; top: 10px; left: 10px; z-index: 10;">
            <input id="screenfit" type="checkbox" checked={controls.fit_screen} onchange={on_toggle} /> <label for="screenfit">{"Fit to Screen"}</label>
            <input
                id="opacity"
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={controls.opacity.to_string()}
                oninput={on_opacity_input}
              />
            <input type="file" accept="image/*" oninput={on_file_upload} />
        </div>
    }
}
