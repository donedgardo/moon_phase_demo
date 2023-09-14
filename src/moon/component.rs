use leptos::{component, IntoView};
use std::time::SystemTime;
use crate::moon::Moon;

#[component]
pub fn MoonAppMenu(cx: leptos::Scope) -> impl IntoView {
    let moon = Moon::new(SystemTime::now());
    let emoji = moon.phase_emoji();
    let phase = moon.phase_name();
    leptos::view! { cx,
        <div
          id="app"
          class="mx-auto dark:flex hidden flex-col justify-center items-center space-y-4 text-stone-200"
        >
          <p style="font-size: 180px">{emoji}</p>
          <h1 class="text-6xl font-bold underline text-center">Lunar Harvest</h1>
          <h2 class="text-5xl font-bold text-center">{phase}</h2>
          <p class="text-1xl text-center" hx-get="/game" hx-target="#app" hx-swap="outerHTML">
            <em>Come back on a a new moon.</em>
          </p>
        </div>
        <div
          id="app"
          class="mx-auto flex dark:hidden flex-col space-y-4 text-xl"
        >
          <p>Enable the night,</p>
          <p>Dark skies bring secret features,</p>
          <p>Mode of moon whispers.</p>
        </div>
    }
}
