use yew::prelude::*;

mod webcam;
use webcam::Webcam;

mod overlay;

mod controls;
use controls::{ControlCtx, Controls};

#[function_component(App)]
fn app() -> Html {
    let control_ctx = use_state(|| ControlCtx { fit_screen: false });

    html! {
        <div>
            <ContextProvider<UseStateHandle<ControlCtx>> context={control_ctx.clone()}>
                <Controls />
                <Webcam />
            </ContextProvider<UseStateHandle<ControlCtx>>>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
