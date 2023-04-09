use gloo_timers::callback::Interval;
use leptos::*;

use crate::GlobalState;

pub type GridType = Vec<(usize, (ReadSignal<Vec<(usize, RwSignal<bool>)>>, WriteSignal<Vec<(usize, RwSignal<bool>)>>))>;

#[component]
pub fn Grid(
    cx: Scope,
    x_grids: usize,
    y_grids: usize,
    counters: ReadSignal<GridType>,
    set_counters: WriteSignal<GridType>,
    state: RwSignal<GlobalState>,
) -> impl IntoView {    
    let (_, set_interval) = create_signal(cx, None);

    create_effect(cx, move |_| {
        let state_inner = state.get();
        if state_inner.first_call {
            state.update(|x| x.first_call = false);
            return;
        }

        if state_inner.playing {
            log!("starting with interval {}", state_inner.interval);
            let interval = Interval::new(state_inner.interval, move || {
                game_tick(set_counters, &x_grids, &y_grids);
            });
            set_interval.update(|x| {
                x.replace(interval);
            });
        } else {
            set_interval.update(|x: &mut Option<Interval>| {
                if let Some(interval) = x.take() {
                    interval.cancel();
                }
            });
        }
    });

    view! { cx,
        <div>
            <For
                each=counters
                key=|row| row.0
                view=move |cx, (row_id, columns)| {
                    view!{ cx,
                        <div class="row">
                            <For
                                each=columns.0
                                key=|column| column.0
                                view=move |cx, (column_id, alive)| {
                                    view! { cx,
                                        <div
                                            class="square"
                                            class:alive=move || alive()
                                            on:click=move |_| set_counters.update(|counter| counter[row_id].1.1.update(|column| column[column_id].1.update(|cell| *cell = !*cell)))
                                        >
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }
                }
            />
        </div>
    }
}

fn game_tick(set_counters: WriteSignal<GridType>, x_grids: &usize, y_grids: &usize) {
    set_counters.update(|grid| {
        for i in 0..grid.len() {
            for j in 0..grid[i].1.0().len() {
                let num_neighbors = get_neighbors(&i, &j, x_grids, y_grids, grid);
                if (grid[i].1.0()[j].1() && num_neighbors < 2) || num_neighbors > 3 {
                    grid[i].1.1.update(|column| column[j].1.update(|cell| *cell = false));
                } else if !grid[i].1.0()[j].1() && num_neighbors == 3 {
                    grid[i].1.1.update(|column| column[j].1.update(|cell| *cell = true));
                }
            }
        }
    });
}

fn get_neighbors(i: &usize, j: &usize, x_grids: &usize, y_grids: &usize, grid: &mut GridType) -> usize {
    let mut count = 0;
    if *i + 1 < *y_grids && grid[i + 1].1.0()[*j].1() {
        count += 1;
    }
    if *i + 1 < *y_grids && *j + 1 < *x_grids && grid[i + 1].1.0()[*j + 1].1() {
        count += 1;
    }
    if *j + 1 < *x_grids && grid[*i].1.0()[j + 1].1() {
        count += 1;
    }
    if *i != 0 && *j + 1 < *x_grids && grid[i - 1].1.0()[j + 1].1() {
        count += 1;
    }
    if *j != 0 && grid[*i].1.0()[j - 1].1() {
        count += 1;
    }
    if *i != 0 && *j != 0 && grid[i - 1].1.0()[j - 1].1() {
        count += 1;
    }
    if *i != 0 && grid[i - 1].1.0()[*j].1() {
        count += 1;
    }
    if *i + 1 < *y_grids && *j != 0 && grid[i + 1].1.0()[j - 1].1() {
        count += 1;
    }
    return count;
}