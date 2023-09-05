use bson::DateTime;
use leptos::*;
use leptos_router::*;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct RapiStruct {
    node_val: String,
    node_last_update: DateTime,
    node_name: String,
    node_rw_direction: String,
    node_uid: String,
}

#[derive(Debug, Serialize, Deserialize)]

struct MqttStruct {
    mqtt_lock_to_uid: String,
    mqtt_ip: String,
    mqtt_topic: String,
    mqtt_topic_modif: i32,
    mqtt_rw: String,
}
#[derive(Debug, Serialize, Deserialize)]

struct ModbusStruct {
    mb_lock_to_uid: String,
    mb_ip: String,
    mb_port: String,
    mb_register: String,
    mb_rw: String,
}

#[derive(Debug, Serialize, Deserialize)]

struct NewUidGet {
    uid: String,
}

async fn get_all_node_data(node_name: &str) -> String {
    let mut get_url = "http://127.0.0.1:8000/getall/".to_string();
    get_url.push_str(&node_name);

    let resp = reqwest::get(get_url).await.unwrap().text().await.unwrap();

    resp
}

async fn crt_new_uid(_count: i32) -> String {
    let get_url = "http://127.0.0.1:8000/c/".to_string();

    let resp: NewUidGet = reqwest::get(get_url).await.unwrap().json().await.unwrap();
    log!("{resp:?}");

    resp.uid
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

    let id_for_later = dnode_descr;

    let async_data = create_resource(
        cx,
        || (),
        move |_| async move { get_all_node_data(dnode_descr).await },
    );

    view! { cx,
        <div class="contact-info">
        <p>    {move || match async_data.read(cx) {
            None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
            Some(data) => view! { cx, <ShowData data id_for_later/>  }.into_view(cx)
        }}  </p>
        </div>
    }
}

#[component]
fn ShowData(cx: Scope, data: String, id_for_later: &'static str) -> impl IntoView {
    if id_for_later == "mbstuff" {
        let vect_data_mb: Vec<ModbusStruct> = serde_json::from_str(&data).unwrap();
        let mapped_view = vect_data_mb
            .into_iter()
            .map(|mbstruct| {
                view! { cx,
                    <ShowMbSingleData mbstruct/>
                }
            })
            .collect::<Vec<_>>();
        view! { cx,
            <div>{mapped_view}</div>
        }
    } else if id_for_later == "mqttstuff" {
        let vect_data_mqtt: Vec<MqttStruct> = serde_json::from_str(&data).unwrap();
        let mapped_view = vect_data_mqtt
            .into_iter()
            .map(|mqttstruct| {
                view! { cx,
                    <ShowMqttSingleData mqttstruct/>
                }
            })
            .collect::<Vec<_>>();
        view! { cx,
            <div>{mapped_view}</div>
        }
    } else if id_for_later == "bucket" {
        let vect_data_rapi: Vec<RapiStruct> = serde_json::from_str(&data).unwrap();
        let mapped_view = vect_data_rapi
            .into_iter()
            .map(|rapistruct| {
                view! { cx,
                    <ShowRapiSingleData rapistruct/>
                }
            })
            .collect::<Vec<_>>();
        view! { cx,
            <div>{mapped_view}</div>
        }
    } else {
        view! { cx,
            <div>"No map"</div>
        }
    }
}

#[component]
fn ShowMbSingleData(cx: Scope, mbstruct: ModbusStruct) -> impl IntoView {
    view! { cx,
        <div class="mb_data_single">
            <p> "locked to data node: "{mbstruct.mb_lock_to_uid} </p>
            <p> "mb ip: " {mbstruct.mb_ip} </p>
            <p> "mb port: " {mbstruct.mb_port} </p>
            <p> "mb register: " {mbstruct.mb_register} </p>
            <p> "mb read/write: " {mbstruct.mb_rw} </p>
        </div>
    }
}

#[component]
fn ShowMqttSingleData(cx: Scope, mqttstruct: MqttStruct) -> impl IntoView {
    view! { cx,
        <div class="mqtt_data_single">
            <p> "locked to data node: "{mqttstruct.mqtt_lock_to_uid} </p>
            <p> "mqtt ip: " {mqttstruct.mqtt_ip} </p>
            <p> "mqtt topic: " {mqttstruct.mqtt_topic} </p>
            <p> "mqtt topic modifier: " {mqttstruct.mqtt_topic_modif} </p>
            <p> "mqtt read/write: " {mqttstruct.mqtt_rw} </p>
        </div>
    }
}

#[component]
fn ShowRapiSingleData(cx: Scope, rapistruct: RapiStruct) -> impl IntoView {
    view! { cx,
        <div class="rapi_data_single">
            <p> "Data node uid: " {rapistruct.node_uid} </p>
            <p> "Data node value: "{rapistruct.node_val} </p>
            <p> "Data node last updated: " {rapistruct.node_last_update.to_string()} </p>
            <p> "Data node name: " {rapistruct.node_name} </p>
            <p> "Data node read/write ? : " {rapistruct.node_rw_direction} </p>
        </div>
    }
}

#[component]
fn NewRapiNode(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 1);
    let async_data = create_resource(cx, move || count.get(), crt_new_uid);

    let async_result = move || {
        async_data
            .read(cx)
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    view! { cx,
        <div class="new_node">
        <button on:click= move |_| {
            set_count.update(|n| *n += 1);
        }
        // the class: syntax reactively updates a single class
        // here, we'll set the `red` class when `count` is odd
        class:btn_disabled=move || { count.get() > 1 }
    >
        "Click me"
          </button>
            {async_result}
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
