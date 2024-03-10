use crate::plot::Function;
#[cfg(feature = "ssr")]
use crate::plot::{mk_overlayed_image, plot::PlotConfig};
use base64::{engine::general_purpose, Engine};
use leptos::logging::*;
use leptos::{
    html::{Img, Input},
    *,
};
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::JsString;
use web_sys::{js_sys::Uint8Array, Event};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Graph-BG"/>

        // content for this site
        <body class="bg-base" className="items-center">
            <Router>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </Router>
        </body>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use wasm_bindgen::JsCast;
    use web_sys::{FileReader, HtmlInputElement};

    let initial_length = 0;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_funcs = (0..initial_length)
        .map(|id| (id, String::new(), None))
        .collect::<Vec<(usize, String, Option<String>)>>();
    // Creates a reactive value to update the button
    let (funcs, set_func) = create_signal(initial_funcs);
    let mut next_counter_id = initial_length;

    let (min, set_min) = create_signal(String::new());
    let (max, set_max) = create_signal(String::new());
    let (bg_img, set_bg_img) = create_signal(String::new());
    let (graphs, set_graphs) = create_signal(Vec::<String>::new());
    let (graph_i, set_graph_i) = create_signal(0);

    let input_element: NodeRef<Input> = create_node_ref();
    let img_input_element: NodeRef<Input> = create_node_ref();
    // let img_preview: NodeRef<Img> = create_node_ref();
    let graph_view: NodeRef<Img> = create_node_ref();

    // TODO: add signal to store the images and make the carousel

    let on_change = move |_ev| {
        let img = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window")
            .get_element_by_id("input_image")
            .expect("should have a document on window")
            .dyn_into::<HtmlInputElement>()
            .expect("failed to make HtmlInputElement");

        let set_base_img = Closure::wrap(Box::new(move |event: Event| {
            let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
            let data = element.result().unwrap();
            let file_string = data.dyn_into::<JsString>().unwrap();
            set_bg_img(file_string.as_string().unwrap());
            log!("file loaded: {:?}", file_string);
        }) as Box<dyn FnMut(_)>);

        let reader = FileReader::new().unwrap();
        reader.set_onload(Some(set_base_img.as_ref().unchecked_ref()));
        set_base_img.forget();

        reader
            .read_as_data_url(
                &img.files()
                    .unwrap()
                    .get(0)
                    .expect("should have a file handle."),
            )
            .expect("failed to read image file");
    };

    view! {
        // <script type="text/javascript" inner_html=
        //     r#"
        //         function encodeImageFileAsURL(element) {
        //             var file = element.files[0];
        //             var reader = new FileReader();
        //             reader.onloadend = function() {
        //                 document.getElementById("baseImg").src = reader.result;
        //             }
        //             reader.readAsDataURL(file);
        //         }
        //     "#
        // />
        <div class="p-4">
            <section id="header" class="grid gap-8 items-center text-center bg-surface0 p-4 rounded-lg">
                <div>
                    <h1 class="bg-surface0 text-4xl text-peach">"Graph-BG"</h1>
                </div>
            </section>
        </div>
        <hr class="bg-peach"/>
        <main class="bg-base p-2">
            <section id="mainArea" class="grid gap-8 grid-cols-11">
                <div class="col-span-4">
                    <h4 class="text-2xl p-2 text-peach" align="center" >Graph Settings:</h4>
                    <section id="input" class="grid-rows-2 items-center text-center">
                        <div>
                            // <section class="grid grid-cols-2">
                            <div id="settings">
                                <label for="image">"Base Image:"</label>
                                // <input type="file" name="image" class="bg-overlay2 rounded-lg p-1" onchange="encodeImageFileAsURL(this)" accept="image/png"/>
                                <input
                                    on:change=on_change node_ref=img_input_element
                                    type="file" name="image" id="input_image" class="bg-overlay2 rounded-lg p-1" accept="image/png"/>
                                <form
                                    on:submit=move |ev| {
                                        ev.prevent_default();
                                        match ev.submitter() {
                                            Some(elm) if elm.id() == "addFunc" => {
                                                ev.prevent_default();
                                                // add this counter to the list of counters
                                                set_func.update(move |funcs| {
                                                    // since `.update()` gives us `&mut T`
                                                    // we can just use normal Vec methods like `push`
                                                    match input_element() {
                                                        Some(f) if f.value() != "" => {
                                                            funcs.push((next_counter_id, f.value(), None));
                                                            f.set_value("");
                                                        }
                                                        _ => {}
                                                    }
                                                });
                                                // increment the ID so it's always unique
                                                next_counter_id += 1;
                                            },
                                            Some(elm) if elm.id() == "plot" => {
                                                // generate graph
                                                let min = if min.get() != "" {min.get()} else {"-10".to_string()};
                                                let max = if max.get() != "" {max.get()} else {"10".to_string()};
                                                let query = funcs.get().clone().iter().map(|(_, f, c)| Function {func: f.to_owned(), color: c.clone()}).collect();

                                                spawn_local(async move {
                                                    // if let Some(img_file) = bg_img.get() {
                                                        // log!("image input data: {}", );
                                                        let img_dat = bg_img.get()
                                                            .split_once(";base64,")
                                                            .unwrap_or(("", bg_img.get().as_str()))
                                                            .1
                                                            .to_string();
                                                        // make graph and display it in the preview
                                                        if let Some(graph_elm) = graph_view() {
                                                            // TODO: stop mulitple parallel requests
                                                            match graph(img_dat, query, min, max).await {
                                                                Ok(graph_dat) => {
                                                                    let b64 = format!("data:image/png;base64,{graph_dat}");
                                                                    graph_elm.set_src(&b64);
                                                                    set_graphs.update(|graphs| graphs.push(b64));
                                                                }
                                                                Err(e) => {
                                                                    error!("{e}");
                                                                    // TODO: alert of error.
                                                                }
                                                            }
                                                        }
                                                    // }
                                                });
                                            },
                                            _ => {}
                                        }
                                    }

                                    on:change=move |ev| {
                                        ev.prevent_default();
                                    }
                                >
                                    // <label for="imgIn">"Upload Image"</label><br/>
                                    // <input type="file" id="imgIn" name="image" onchange="encodeImageFileAsURL(this)" accept="image/png"/><br/>
                                    <label for="min">Min:</label><input type="number" id="min"
                                        on:input=move |ev| {
                                            // event_target_value is a Leptos helper function
                                            // it functions the same way as event.target.value
                                            // in JavaScript, but smooths out some of the typecasting
                                            // necessary to make this work in Rust
                                            set_min(event_target_value(&ev));
                                        }
                                        name="query[min]" required value="-10" class="text-crust bg-overlay2 rounded-lg p-1"/><br/>
                                    <label for="max">Max:</label><input type="number" id="max"
                                        on:input=move |ev| {
                                            // event_target_value is a Leptos helper function
                                            // it functions the same way as event.target.value
                                            // in JavaScript, but smooths out some of the typecasting
                                            // necessary to make this work in Rust
                                            set_max(event_target_value(&ev));
                                        }
                                        name="query[max]" required value="10" class="text-crust bg-overlay2 rounded-lg p-1"/><br/>
                                    <label for="function">Function:</label>  // <br/>
                                    <input type="text" id="function" name="query[funcs]" value="" class="text-crust bg-overlay2 rounded-lg p-1" node_ref=input_element/>
                                    <input type="submit" id="addFunc" value=" Add Function " class="bg-sapphire text-crust p-1 rounded-lg"/><br/>
                                    <br/>
                                    <input type="submit" id="plot" value="  Plot  " class="bg-green p-3 text-xl rounded-lg text-crust"/>
                                    <br/>
                                    <br/>
                                    // <p></p>
                                </form>
                            </div>
                            // <img id="baseImg" class="rounded-lg size-fit" align="right" node_ref=img_preview/>
                            // </section>
                        </div>
                        <hr class="bg-peach"/>
                        <div>
                            <h4 class="text-2xl p-2 text-peach">Functions:</h4>
                            <ul>
                                <For
                                    // `each` takes any function that returns an iterator
                                    // this should usually be a signal or derived signal
                                    // if it's not reactive, just render a Vec<_> instead of <For/>
                                    each=funcs
                                    // the key should be unique and stable for each row
                                    // using an index is usually a bad idea, unless your list
                                    // can only grow, because moving items around inside the list
                                    // means their indices will change and they will all rerender
                                    key=|counter| counter.0
                                    // `children` receives each item from your `each` iterator
                                    // and returns a view
                                    children=move |(id, func, color)| {
                                        view! {
                                            <li>
                                                // { id + 1 } "  =>    "
                                                <input
                                                    on:change=move |ev| {
                                                        set_func.update(|f| {
                                                            let new_f = event_target_value(&ev);

                                                            // if new_f.len() == 7 || new_f.len() == 9 {
                                                            f.iter_mut().for_each(|f2| {if f2.0 == id { f2.1 = new_f.clone() }});
                                                            // }
                                                        });
                                                    } value={func}
                                                    class="text-crust bg-overlay2 p-1 rounded-lg text-left"
                                                />
                                                <input
                                                    on:change=move |ev| {
                                                        set_func.update(|f| {
                                                            let new_color = event_target_value(&ev);

                                                            if new_color.len() == 7 || new_color.len() == 9 {
                                                                f.iter_mut().for_each(|func| {if func.0 == id { func.2 = Some(new_color.clone()) }});
                                                            }
                                                        });
                                                    } value={ color.unwrap_or("#FFFFFF".to_string()) }
                                                    class="text-crust bg-overlay2 p-1 rounded-lg text-left"
                                                />
                                                <button
                                                    on:click=move |_| {
                                                        set_func.update(|f| {
                                                            f.retain(|(counter_id, _, _)| counter_id != &id)
                                                        });
                                                    }
                                                    class="text-crust bg-red rounded-lg p-1"
                                                >
                                                    "  Remove  "
                                                </button>
                                            </li>
                                        }
                                    }
                                />
                            </ul>
                        </div>
                    </section>
                </div>
                <div class="col-span-7">
                    <section id="display" class="grid-rows-3" >
                        <div>
                            // TODO: make this display the graph from graphs identified by graph_i
                            <img id="graph" class="rounded-lg size-full" node_ref=graph_view/>
                        </div>
                        <div>
                            // TODO: put the circles here
                        </div>
                        <div>
                            // TODO: keep a list of the previous graphs and add forward and back arrows
                            // here
                            //
                            // OR
                            //
                            // TODO: download button
                            //
                            // OR
                            //
                            // TODO: combo of above
                            //
                            //  <- *********** ->
                        </div>
                    </section>
                </div>
            </section>
        </main>
    }
}

#[server]
async fn graph(
    image_b64: String,
    query: Vec<Function>,
    min: String,
    max: String,
) -> Result<String, ServerFnError> {
    use base64::{engine::general_purpose, Engine as _};

    let image: Vec<u8> = general_purpose::STANDARD.decode(image_b64)?;
    let configs = query.iter().map(|f| PlotConfig::from(f)).collect();
    let min = min.parse()?;
    let max = max.parse()?;

    if min > max || max - min > 100_000 {
        return Err(ServerFnError::ServerError("invalid range".to_string()));
    }

    mk_overlayed_image(&mut image.to_owned(), configs, min, max)
        .map_err(|e| ServerFnError::ServerError(e))
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
        <div>"your princess is in another castle 🥲"</div>
    }
}
