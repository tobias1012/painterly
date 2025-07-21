use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ControlCtx {
    pub fit_screen: bool,
}

#[function_component(Controls)]
pub fn controls() -> Html {
    let controls = use_context::<UseStateHandle<ControlCtx>>().expect("ControlCtx not found");

    let on_toggle = {
        let controls = controls.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            controls.set(ControlCtx {
                fit_screen: input.checked(),
            });
        })
    };

    html! {
        <div class="controls">
            <input type="checkbox" checked={controls.fit_screen} onchange={on_toggle} /> <label>{"Fit to Screen"}</label>
        </div>
    }
}
