use leptos::*;

use crate::GlobalState;

#[component]
pub fn Modal(
    cx: Scope,
    active: ReadSignal<bool>,
    set_modal: WriteSignal<bool>,
    state: RwSignal<GlobalState>
) -> impl IntoView {
    let close_icon = "&#8203;";
    view!{cx, 
        {move || if active.get() {
            view! { cx,
                <div class="fixed z-10 inset-0 overflow-y-auto" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                    <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
                        <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" aria-hidden="true"></div>
                        <span class="hidden inline-block align-middle h-screen" aria-hidden="true">{close_icon}</span>
                        <div
                            class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all my-8 align-middle max-w-7xl w-full"
                        >
                            <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                                <div class="sm:flex sm:items-start">
                                    <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                                        <h1 class="text-4xl leading-3 font-medium text-gray-900" id="modal-title">
                                            "Conway's Game of Life"
                                        </h1>
                                        <div class="mt-10">
                                            <div class="mb-4">
                                                <label class="block text-gray-700 text-sm font-bold mb-2" for="gameInterval">
                                                    "Game interval"
                                                </label>
                                                <input
                                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                                    id="gameInterval"
                                                    type="number"
                                                    placeholder="Game interval (ms)"
                                                    min="10"
                                                    value={state.get().interval}
                                                    on:change=move |ev| {
                                                        state.update(|state| {
                                                            state.interval = event_target_value(&ev).parse().unwrap()
                                                        });
                                                    }
                                                />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
                                <button
                                    on:click=move |_| {
                                        set_modal.set(false);
                                        state.update(|v| v.playing = true)
                                    }
                                    type="button"
                                    class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-indigo-800 text-base font-medium text-white hover:bg-indigo-900 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"
                                >
                                    "Start"
                                </button>
                                <button
                                    on:click=move |_| set_modal.set(false)
                                    type="button"
                                    class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
                                >
                                    "Cancel"
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else {
            view! {cx, <div></div>}
        }}
    }
}