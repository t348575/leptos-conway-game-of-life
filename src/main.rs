use leptos::*;

use grid::*;
use modal::*;

mod grid;
mod modal;

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalState {
    playing: bool,
    interval: u32,
    first_call: bool
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let document = document().document_element().unwrap();
    let x = document.client_width();
    let y = document.client_height();

    let x_grids = ((x as f64 / 20.0).floor() - ((x as f64 / 20.0).ceil() / 20.0).ceil() + 5.0).round() as usize;
    let y_grids = ((y as f64 / 20.0).floor() - ((y as f64 / 20.0).ceil() / 20.0).ceil() + 5.0).round() as usize;

    let counters = (0..y_grids).map(|id| (id, create_signal(cx, (0..x_grids).map(|id| (id, create_rw_signal(cx, false))).collect::<Vec<_>>()))).collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(cx, counters);

    let (modal, set_modal) = create_signal(cx, false);

    let state = create_rw_signal(cx, GlobalState { playing: false, interval: 10, first_call: true });

    view! { cx,
        <main class="w-screen h-screen font-sans">
            <div class="center">
                <Grid x_grids y_grids state counters set_counters/>
            </div>
            <Modal active=modal set_modal=set_modal state=state/>
            <button
                class="absolute bottom-2 right-2 bg-indigo-800 w-11 h-11 hover:bg-indigo-900 font-bold rounded-full"
                on:click=move |_| set_modal.update(|value| *value = !*value)
            >
                <i
                    class="fas text-white"
                    class:fa-play=move || !state.get().playing
                    class:fa-pause=move || state.get().playing
                ></i>
            </button>
            <button
                class="absolute bottom-16 right-2 bg-indigo-800 w-11 h-11 hover:bg-indigo-900 font-bold rounded-full"
                on:click=move |_| reset_grid(set_counters)
            >
                <i class="fas fa-redo-alt text-white"></i>
            </button>
            <button
                class="absolute bottom-2 right-16 bg-indigo-800 w-11 h-11 hover:bg-indigo-900 font-bold rounded-full"
                on:click=move |_| gen_random(set_counters, x_grids, y_grids)
            >
                <i class="fas fa-random text-white"></i>
            </button>
        </main>
    }
}

fn reset_grid(set_counters: WriteSignal<GridType>) {
    set_counters.update(|column| {
        for (_, (_, row)) in column {
            row.update(|row_item| row_item.iter_mut().for_each(|(_, cell)| cell.update(|cell_item| *cell_item = false)));
        }
    })
}

fn gen_random(set_counters: WriteSignal<GridType>, x_grids: usize, y_grids: usize) {
    let num_cells = (x_grids * y_grids) as f64;
    let max = get_random(num_cells * 0.01, num_cells * 0.1) as usize;
    for _ in 0..max {
        let x = get_random(0.0, x_grids as f64- 1.0) as usize;
        let y = get_random(0.0, y_grids as f64- 1.0) as usize;

        set_counters.update(|row| row[y].1.1.update(|column| column[x].1.update(|cell| *cell = true)));
    }
}

fn get_random(min: f64, max: f64) -> f64 {
    ((js_sys::Math::random() * max) + min).floor()
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}