use std::collections::HashMap;

use bson::DateTime;
use leptos::ev::SubmitEvent;
use leptos::html::{Input, Select};
use leptos::svg::Svg;
use leptos::*;
use leptos_router::*;
use reqwest;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, USER_AGENT,
};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct RapiStruct {
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
    mqtt_topic_modif: String,
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

async fn crt_new_uid(count: i32) -> String {
    let mut _to_ret = String::from("");
    if count == 1 {
        let get_url = "http://127.0.0.1:8000/c/".to_string();

        let resp: NewUidGet = reqwest::get(get_url).await.unwrap().json().await.unwrap();
        _to_ret = resp.uid;
    } else {
        // Not the best method but might do the trick for now
        _to_ret = String::from("");
    }

    _to_ret
}

async fn post_data(url: &str, hmap: HashMap<&str, String>) -> String {
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
        headers
    }

    let form_url = format!("{url}");
    let rw_client = reqwest::Client::new();
    let res = rw_client
        .post(form_url)
        .json(&hmap)
        .headers(construct_headers())
        .send()
        .await;

    let mut ret_val = String::new();

    match res {
        Ok(resp_in) => {
            log!("{resp_in:?}");
            if resp_in.status() == 200 {
                ret_val = String::from("200");
            }
        }
        Err(err) => {
            log!("{err:?}");
            ret_val = String::from("500");
        }
    };
    ret_val
}

#[component]
fn NavComponent(
    cx: Scope,
    href: String,
    text_to_show: String,
    svg_to_use: HtmlElement<Svg>,
) -> impl IntoView {
    view! {
        cx,
        <a class="flex items-center my-2 px-4 py-2 text-gray-700 transition-colors duration-300 transform rounded-md dark:bg-gray-800 dark:text-gray-200" href={href}>
        {svg_to_use}
        <span class="mx-4 font-medium">{text_to_show}</span>
        </a>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // Svg icon for the home menu point
    let home_svg = view! {cx,
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
    </svg>};

    // Svg icon for the home menu point
    let data_svg = view! {cx,
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
    <path d="M22.485,10.975,12,17.267,1.515,10.975A1,1,0,1,0,.486,12.69l11,6.6a1,1,0,0,0,1.03,0l11-6.6a1,1,0,1,0-1.029-1.715Z"/>
    <path d="M22.485,15.543,12,21.834,1.515,15.543A1,1,0,1,0,.486,17.258l11,6.6a1,1,0,0,0,1.03,0l11-6.6a1,1,0,1,0-1.029-1.715Z"/>
    <path d="M12,14.773a2.976,2.976,0,0,1-1.531-.425L.485,8.357a1,1,0,0,1,0-1.714L10.469.652a2.973,2.973,0,0,1,3.062,0l9.984,5.991a1,1,0,0,1,0,1.714l-9.984,5.991A2.976,2.976,0,0,1,12,14.773ZM2.944,7.5,11.5,12.633a.974.974,0,0,0,1,0L21.056,7.5,12.5,2.367a.974.974,0,0,0-1,0h0Z"/>
    </svg>};

    view! { cx,
        <Router>

        <main>
        <div class="">

        <aside class="flex flex-col w-64 float-left h-screen px-4 py-8 overflow-y-auto bg-white border-r rtl:border-r-0 rtl:border-l dark:bg-gray-900 dark:border-gray-700 sticky top-0">

        <div class="relative mt-6">
            <span class="mx-2 font-medium text-white">"DATA CONCENTRATOR UI"</span>

        </div>

        <div class="flex flex-col justify-between flex-1 mt-6 ">
                <nav>

                    <NavComponent href="/".to_string() text_to_show="Home".to_string() svg_to_use=home_svg />

                    <NavComponent href="/dmap".to_string() text_to_show="Data Map".to_string() svg_to_use=data_svg />
                    <hr class="my-6 border-gray-200 dark:border-gray-600" />


                </nav>

        </div>
    </aside>
        <Routes>

            <Route path="/" view=|cx| view! { cx,
                <h3>"Home"</h3>
            }/>
            <Route
                path="/dmap"
                view=DataNodeList
            >
                // if no id specified, fall back
                <Route path=":id" view=DataNodeInfo>
                    <Route path="" view=DataNodeDesc />
                    <Route path="dnodes" view=DataNodeData/>
                    <Route path="newdnode" view=NewDnodeComp/>
                </Route>
                // if no id specified, fall back
                <Route path="" view=|cx| view! { cx,
                    <div class="select-dnode">
                        "Select a data node to view the information."
                    </div>
                }/>
                    </Route>
                </Routes>
                </div>

            </main>

            </Router>



        }
}

#[component]
fn DataNodeList(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-wrap sticky top-0 ">
          <section class="relative w-[100%] sticky top-0 ">
            <nav class="flex bg-gray-900 text-white sticky top-0">
              <div class="px-5 xl:px-12 py-3 flex w-full items-center">
                <ul class="md:flex px-4 font-semibold font-heading space-x-12">
                <li><A  class="hover:text-gray-200" href="rapi">"rAPI"</A></li>
                <li><A  class="hover:text-gray-200" href="mbtcp">"Modbus TCP"</A></li>
                <li><A  class="hover:text-gray-200" href="mqtt">"MQTT"</A></li>
                </ul>
                </div>
            </nav>
          </section>
          <Outlet/>
        </div>
    }
}

#[component]
fn DataNodeInfo(cx: Scope) -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let _name = move || match id().as_str() {
        "rapi" => "REST API",
        "mbtcp" => "Modbus TCP",
        "mqtt" => "MQTT",
        _ => "Data protocol not found!",
    };

    view! { cx,
            <div class="dnode-info w-[100%] sticky top-12 ">
            <div class="flex flex-wrap sticky top-12 ">
              <section class="relative w-[100%]  sticky top-12 ">
                <nav class="flex bg-gray-600 text-white sticky top-12">
                  <div class="px-5 xl:px-12 py-3 flex w-full items-center">
                    <ul class="md:flex px-4 font-semibold font-heading space-x-12">
                    <li><A  class="hover:text-gray-200" href="">"Info"</A></li>
                    <li><A  class="hover:text-gray-200" href="newdnode">"New"</A></li>
                    <li><A  class="hover:text-gray-200" href="dnodes">"Data nodes"</A></li>
                    </ul>
                    </div>
                </nav>
              </section>
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
    let (count, set_count) = create_signal(cx, 0);
    let async_data = create_resource(cx, move || count.get(), crt_new_uid);

    let async_result = move || async_data.read(cx).unwrap_or_else(|| "Loading...".into());

    view! { cx,
        <div class="new_node m-5 max-w-sm rounded overflow-hidden shadow-lg">
        <button
        class="bg-blue-600 hover:bg-blue-900 text-white font-bold py-2 px-4 rounded"
        on:click= move |_| {
            set_count.update(|n| *n += 1);
        }
        class:btn_disabled=move || { count.get() > 0 }
    >
        "Generate new UID"
          </button>
            <Show
            when=move || { count.get() > 0 }
            fallback=|_cx| view! { _cx, <p class="text-gray-700 text-base"> "Generate new uid to make a new datanode!"</p> }
          >
            <NewRapiForm uid=async_result() scount=set_count/>
          </Show>
        </div>
    }
}

#[component]
fn NewRapiForm(cx: Scope, uid: String, scount: WriteSignal<i32>) -> impl IntoView {
    let input_element_name: NodeRef<Input> = create_node_ref(cx);
    let input_element_default_value: NodeRef<Input> = create_node_ref(cx);

    let select_element_rw: NodeRef<Select> = create_node_ref(cx);

    let uid_tmp = uid.clone();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value_name = input_element_name.get().expect("<input> to exist").value();
        let value_default_value = input_element_default_value
            .get()
            .expect("<input> to exist")
            .value();

        let value_rw = select_element_rw.get().expect("<select> to exist").value();
        let mut map = HashMap::new();
        map.insert("node_val", value_default_value);
        map.insert("node_uid", uid_tmp.clone());
        map.insert("node_rw_direction", value_rw);
        map.insert("node_name", value_name);

        spawn_local(async move {
            let resp = post_data("http://127.0.0.1:8000/u", map).await;
            if resp == "200".to_string() {
                scount.set(0);
            } else {
                log!("Error on server side, needs better handler later!");
            }
        });
    };

    view! { cx,
            <div class="new_node_form">

                <form class="w-full max-w-sm p-4"
                 on:submit=on_submit>
                 <div class="md:flex md:items-center mb-6">

                 <div class="md:w-1/3">
                 <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="uid">
                 "Generated uid: "
                </label>
                 </div>
                <div class="md:w-2/3">
                <span
                class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500"
                >
                {uid}
                </span>
                </div>
                </div>

                <FormInputCust node_ref_cust=input_element_name
                label_text="Data node name: ".to_string()
                id_name="name_input".to_string() />

                <FormInputCust node_ref_cust=input_element_default_value
                label_text="Data node default value: ".to_string()
                id_name="value_input".to_string() />

                <FormSelectCust node_ref_cust=select_element_rw
                label_text="Data node read/write: ".to_string()
                id_name="read_write_select".to_string() />

                <FormSubmitButton />
            </form>
            </div>
    }
}

#[component]
fn FormSubmitButton(cx: Scope) -> impl IntoView {
    view! {cx,
        <input
        class="shadow bg-green-500 hover:bg-green-400 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded m-5"
        type="submit" value="Submit"/>
    }
}

#[component]
fn FormInputCust(
    cx: Scope,
    node_ref_cust: NodeRef<Input>,
    id_name: String,
    label_text: String,
) -> impl IntoView {
    view! {cx,
        <div class="md:flex md:items-center mb-6">

          <div class="md:w-1/3">
          <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for={&id_name}>
          {label_text}
          </label>
        </div>
        <div class="md:w-2/3">
        <input type="text"
        id={&id_name}
        class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500"
        node_ref=node_ref_cust
    />
    </div>
    </div>}
}

#[component]
fn FormSelectCust(
    cx: Scope,
    node_ref_cust: NodeRef<Select>,
    id_name: String,
    label_text: String,
) -> impl IntoView {
    view! {cx,
        <div class="md:flex md:items-center mb-6">

          <div class="md:w-1/3">
          <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for={&id_name}>
          {label_text}
          </label>
        </div>
        <div class="md:w-2/3">
        <select
        id={&id_name}
        class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500"
        node_ref=node_ref_cust
        >
        <option value="rw">rw</option>
        <option value="r">r</option>
        <option value="w">w</option>
        </select>
    </div>
    </div>}
}

#[component]
fn NewMbtcpNode(cx: Scope) -> impl IntoView {
    let input_element_ip: NodeRef<Input> = create_node_ref(cx);
    let input_element_port: NodeRef<Input> = create_node_ref(cx);

    let select_element_rw: NodeRef<Select> = create_node_ref(cx);

    let input_element_lock_to_uid: NodeRef<Input> = create_node_ref(cx);
    let input_element_register: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value_ip = input_element_ip.get().expect("<input> to exist").value();
        let value_port = input_element_port.get().expect("<input> to exist").value();
        let value_lock_to_uid = input_element_lock_to_uid
            .get()
            .expect("<input> to exist")
            .value();
        let value_register = input_element_register
            .get()
            .expect("<input> to exist")
            .value();

        let value_rw = select_element_rw.get().expect("<select> to exist").value();
        let mut map = HashMap::new();

        map.insert("mb_lock_to_uid", value_lock_to_uid);
        map.insert("mb_ip", value_ip);
        map.insert("mb_port", value_port);
        map.insert("mb_rw", value_rw);
        map.insert("mb_register", value_register);

        spawn_local(async move {
            let resp = post_data("http://127.0.0.1:8000/cmbtcp", map).await;
            if resp == "200".to_string() {
            } else {
                log!("Error on server side, needs better handler later!");
            }
        });
    };

    view! { cx,
        <div class="m-5 max-w-sm rounded overflow-hidden shadow-lg">
        <div class="new_node_form ">
            <form class="w-full max-w-sm p-4"
            on:submit=on_submit>

            <FormInputCust node_ref_cust=input_element_lock_to_uid
            label_text="Mb lock to uid: ".to_string()
            id_name="mb_lock_to_id".to_string() />

            <FormInputCust node_ref_cust=input_element_ip
            label_text="Mb IP: ".to_string()
            id_name="mb_ip".to_string() />


            <FormInputCust node_ref_cust=input_element_port
            label_text="Mb port: ".to_string()
            id_name="mb_port".to_string() />



            <FormInputCust node_ref_cust=input_element_register
            label_text="Mb register: ".to_string()
            id_name="mb_register".to_string() />

            <FormSelectCust node_ref_cust=select_element_rw
            label_text="Mb read/write: ".to_string()
            id_name="mb_read_write_select".to_string() />

            <FormSubmitButton />


        </form>
        </div>
        </div>

    }
}

#[component]
fn NewMqttNode(cx: Scope) -> impl IntoView {
    let input_element_ip: NodeRef<Input> = create_node_ref(cx);
    let input_element_topic: NodeRef<Input> = create_node_ref(cx);

    let select_element_rw: NodeRef<Select> = create_node_ref(cx);

    let input_element_lock_to_uid: NodeRef<Input> = create_node_ref(cx);
    let input_element_topic_modif: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value_ip = input_element_ip.get().expect("<input> to exist").value();
        let value_topic = input_element_topic.get().expect("<input> to exist").value();
        let value_lock_to_uid = input_element_lock_to_uid
            .get()
            .expect("<input> to exist")
            .value();
        let value_topic_modif = input_element_topic_modif
            .get()
            .expect("<input> to exist")
            .value();

        let value_rw = select_element_rw.get().expect("<select> to exist").value();
        let mut map = HashMap::new();
        map.insert("mqtt_lock_to_uid", value_lock_to_uid);
        map.insert("mqtt_ip", value_ip);
        map.insert("mqtt_topic", value_topic);
        map.insert("mqtt_rw", value_rw);
        map.insert("mqtt_topic_modif", value_topic_modif);

        spawn_local(async move {
            let resp = post_data("http://127.0.0.1:8000/cmqtt", map).await;
            if resp == "200".to_string() {
            } else {
                log!("Error on server side, needs better handler later!");
            }
        });
    };

    view! { cx,
        <div class="new_node_form">
            <form on:submit=on_submit>
            "Mqtt lock to uid: "
            <input type="text"
            node_ref=input_element_lock_to_uid
        />
            <br/>

            "Mqtt IP: "
            <input type="text"
                node_ref=input_element_ip
            />

            <br/>

            "Mb topic: "
            <input type="text"
                node_ref=input_element_topic
            />

            <br/>

            "Mb topic modif: "
            <input type="text"
                node_ref=input_element_topic_modif
            />

            <br/>

            "Mb read/write: "
            <select
                node_ref=select_element_rw
            >
            <option value="rw">rw</option>
            <option value="r">r</option>
            <option value="w">w</option>
            </select>

            <br/>
            <input type="submit" value="Submit"/>
        </form>
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
