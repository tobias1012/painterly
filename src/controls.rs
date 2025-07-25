use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ControlState {
    pub fit_screen: bool,
    pub lock_overlay: bool,
    pub opacity: f64,
    pub image_src: Option<String>,
    pub canvas_width: u32,
    pub canvas_height: u32,
}

impl ControlState {
    pub fn new() -> Self {
        ControlState {
            fit_screen: false,
            lock_overlay: false,
            opacity: 0.2,
            image_src: None,
            canvas_width: 800,
            canvas_height: 600,
        }
    }
}

pub enum ControlsAction {
    ToggleFitScreen,
    ToggleOverlay,
    SetOpacity(f64),
    SetImage(String),
    SetCanvasSize(u32, u32),
}

impl Reducible for ControlState {
    type Action = ControlsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new: ControlState = self.as_ref().clone();

        match action {
            ControlsAction::ToggleFitScreen => new.fit_screen = !new.fit_screen,
            ControlsAction::ToggleOverlay => new.lock_overlay = !new.lock_overlay,
            ControlsAction::SetOpacity(opacity) => new.opacity = opacity,
            ControlsAction::SetImage(url) => new.image_src = Some(url),
            ControlsAction::SetCanvasSize(width, height) => {
                new.canvas_width = width;
                new.canvas_height = height;
            }
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

    let on_canvas_size_change = {
        let store = controls.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            let id = input.id();
            if let Ok(size) = value.parse::<u32>() {
                match id.as_str() {
                    "xlength" => {
                        store.dispatch(ControlsAction::SetCanvasSize(size, store.canvas_height))
                    }
                    "ylength" => {
                        store.dispatch(ControlsAction::SetCanvasSize(store.canvas_width, size))
                    }
                    _ => {}
                }
            }
        })
    };

    html! {
        <div class="controls" style="position: fixed; top: 10px; left: 10px; z-index: 10; background: rgba(255, 255, 255, 0.8); padding: 10px; border-radius: 5px;">
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

            <input id="xlength" type="number" placeholder="Canvas width" oninput={on_canvas_size_change.clone()}/>
            <input id="ylength" type="number" placeholder="Canvas height" oninput={on_canvas_size_change}/>
        </div>
    }
}
