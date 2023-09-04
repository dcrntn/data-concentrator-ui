use leptos::*;
use leptos_router::*;
use reqwest;
use serde_json;

async fn get_all_node_data(node_name: &str) -> serde_json::Value {
    let mut get_url = "http://127.0.0.1:8000/getall/".to_string();
    get_url.push_str(&node_name);

    let resp = reqwest::get(get_url)
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    resp
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <h1>"Data concentrator UI"</h1>
            // this <nav> will show on every routes,
            // because it's outside the <Routes/>
            // note: we can just use normal <a> tags
            // and the router will use client-side navigation
            <nav>
                <a href="/">"Home"</a>
                <a href="/dmap">"Data Map"</a>
            </nav>
            <main>
                <Routes>
                    // / just has an un-nested "Home"
                    <Route path="/" view=|cx| view! { cx,
                        <h3>"Home"</h3>
                    }/>
                    // /contacts has nested routes
                    <Route
                        path="/dmap"
                        view=|cx| view! { cx, <DataNodeList/> }
                    >
                        // if no id specified, fall back
                        <Route path=":id" view=|cx| view! { cx,
                            <DataNodeInfo/>
                        }>
                            <Route path="" view=|cx| view! { cx,
                                <DataNodeDesc/>
                            }/>
                            <Route path="dnodes" view=|cx| view! { cx,
                                <DataNodeData/>
                            }/>
                            <Route path="newdnode" view=|cx| view! { cx,
                               <NewDnodeComp/>
                            }/>
                        </Route>
                        // if no id specified, fall back
                        <Route path="" view=|cx| view! { cx,
                            <div class="select-dnode">
                                "Select a data node to view the information."
                            </div>
                        }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn DataNodeList(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="prot-list">
            // here's our contact list component itself
            <div class="prot-list-prots">
                <A href="rapi">"rAPI"</A>
                <A href="mbtcp">"Modbus TCP"</A>
                <A href="mqtt">"MQTT"</A>
            </div>

            <Outlet/>
        </div>
    }
}

#[component]
fn DataNodeInfo(cx: Scope) -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "rapi" => "REST API",
        "mbtcp" => "Modbus TCP",
        "mqtt" => "MQTT",
        _ => "Data protocol not found!",
    };

    view! { cx,
        <div class="dnode-info">
            <h4>{name}</h4>
            <div class="tabs">
                <A href="" exact=true>"Info"</A>
                <A href="newdnode">"New"</A>
                <A href="dnodes">"Data nodes"</A>
            </div>

            <Outlet/>
        </div>
    }
}

#[component]
fn DataNodeDesc(cx: Scope) -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let dnode_descr = move || match id().as_str() {
        "rapi" => "rAPI is the foundation for the communications of this software!",
        "mbtcp" => "Modbus TCP \"mapper\", you can bind MB registers to the rAPI data nodes!",
        "mqtt" => "MQTT \"mapper\", you can bind MQTT values to the rAPI data nodes!",
        _ => "No description for this data node",
    };

    view! { cx,
        <div class="contact-info">
        <p> {dnode_descr} </p>
        </div>
    }
}

#[component]
fn DataNodeData(cx: Scope) -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let dnode_descr = match id().as_str() {
        "rapi" => "bucket",
        "mbtcp" => "mbstuff",
        "mqtt" => "mqttstuff",
        _ => "",
    };
    let async_data = create_resource(
        cx,
        || (),
        move |_| async move {
            log!("loading data from API");
            get_all_node_data(dnode_descr).await
        },
    );

    let async_value = async_data;

    view! { cx,
        <div class="contact-info">
        <p> {}  </p>
        </div>
    }
}

#[component]
fn NewRapiNode(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="new_node">
            "Test rest api new node"
        </div>
    }
}

#[component]
fn NewMbtcpNode(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="new_node">
            "MB TCP"
        </div>
    }
}

#[component]
fn NewMqttNode(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="new_node">
            "Mqtt"
        </div>
    }
}

#[component]
fn NoNewNode(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="new_node">
            "There are no new nodes for this protocol"
        </div>
    }
}
#[component]
fn NewDnodeComp(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let dnode_descr = move || match id().as_str() {
        "rapi" => view! { cx,
            <NewRapiNode/>
        },
        "mbtcp" => view! { cx,
            <NewMbtcpNode/>
        },
        "mqtt" => view! { cx,
            <NewMqttNode/>
        },
        _ => view! { cx,
            <NoNewNode/>
        },
    };

    dnode_descr
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
