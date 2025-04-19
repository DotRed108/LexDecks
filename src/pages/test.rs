use leptos::{html::Input, leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use server_fn::codec::GetUrl;
#[cfg(feature = "ssr")]
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Mutex,
};

use crate::{components::button::{Button, ButtonConfig, ButtonType}, utils_and_structs::{date_and_time::current_time_in_millis, outcomes::Outcome}};

#[component]
pub fn Test() -> impl IntoView {
    view! {
        <h2>"Some Simple Server Functions"</h2>
        <CookieTester/>
        <LatencyTest/>
        <SpawnLocal/>
        <WithAnAction/>
        <WithActionForm/>
        <h2>"Alternative Encodings"</h2>
        <ServerFnArgumentExample/>
    }
}

#[component]
fn CookieTester() -> impl IntoView {

    let action = Action::new(|_| cookie_test());

    let call_cookie_test = move |_| {
        action.dispatch(1);
    };
    
    view! {
        <Button on:click=call_cookie_test config=ButtonConfig {text: "Test Cookies".to_string(), ..Default::default()}/>
    }
}

#[server]
pub async fn cookie_test() -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use tower_cookies::{Cookie, Cookies};

    let cookies = extract::<Cookies>().await?;

    cookies.add(Cookie::new("hello", "what_da"));

    let hi = cookies.get("auth-token");
    println!("{:?}", hi);
    Ok(())
}

#[component]
pub fn LatencyTest() -> impl IntoView {
    #[server]
    pub async fn test_latency(fake_input: String) -> Result<Outcome, ServerFnError> {
        println!("hit the server");
        return Ok(Outcome::CacheFailed(fake_input));
    }
    let ping = RwSignal::new(0);
    let test_latency_action = ServerAction::<TestLatency>::new();
    let on_send = move |_| {
        ping.set(current_time_in_millis())
    };
    let on_recieve = move |outcome: Option<Result<Outcome, ServerFnError>>| {
        let default = 99999999;
        match outcome {
            Some(what) => match what {
                Ok(_uhh) => ping.set(current_time_in_millis() - ping.get_untracked()),
                Err(e) => {console_log(&e.to_string()); ping.set(default)},
            },
            None => ping.set(default),
        }
    };

    let response = test_latency_action.value();

    Effect::new(move |_| {
        on_recieve(response.get());
    });
    view! {
        <ActionForm action=test_latency_action>
            <label for="latency_test">{move || ping.get()}ms</label>
            <input style:display="hidden" id="latency_test" name="fake_input" required placeholder="Enter whatever you want"/>
            <Button on:click=on_send config=ButtonConfig {button_type: ButtonType::Submit, text: "Test Latency".into(), ..Default::default()}/>
        </ActionForm>
    }
}

/// A server function is really just an API call to your server. But it provides a plain async
/// function as a wrapper around that. This means you can call it like any other async code, just
/// by spawning a task with `spawn_local`.
///
/// In reality, you usually want to use a resource to load data from the server or an action to
/// mutate data on the server. But a simple `spawn_local` can make it more obvious what's going on.
#[component]
pub fn SpawnLocal() -> impl IntoView {
    /// A basic server function can be called like any other async function.
    ///
    /// You can define a server function at any scope. This one, for example, is only available
    /// inside the SpawnLocal component. **However**, note that all server functions are publicly
    /// available API endpoints: This scoping means you can only call this server function
    /// from inside this component, but it is still available at its URL to any caller, from within
    /// your app or elsewhere.
    #[server]
    pub async fn shouting_text(input: String) -> Result<String, ServerFnError> {
        // insert a simulated wait
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        Ok(input.to_ascii_uppercase())
    }

    let input_ref = NodeRef::<Input>::new();
    let (shout_result, set_shout_result) = signal("Click me".to_string());

    view! {
        <h3>Using <code>spawn_local</code></h3>
        <p>
            "You can call a server function by using " <code>"spawn_local"</code>
            " in an event listener. "
            "Clicking this button should alert with the uppercase version of the input."
        </p>
        <input node_ref=input_ref placeholder="Type something here."/>
        <button on:click=move |_| {
            let value = input_ref.get().unwrap().value();
            spawn_local(async move {
                let uppercase_text = shouting_text(value).await.unwrap_or_else(|e| e.to_string());
                set_shout_result.set(uppercase_text);
            });
        }>

            {shout_result}
        </button>
    }
}

/// Pretend this is a database and we're storing some rows in memory!
/// This exists only on the server.
#[cfg(feature = "ssr")]
static ROWS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Imagine this server function mutates some state on the server, like a database row.
/// Every third time, it will return an error.
///
/// This kind of mutation is often best handled by an Action.
/// Remember, if you're loading data, use a resource; if you're running an occasional action,
/// use an action.
#[server]
pub async fn add_row(text: String) -> Result<usize, ServerFnError> {
    static N: AtomicU8 = AtomicU8::new(0);

    // insert a simulated wait
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    let nth_run = N.fetch_add(1, Ordering::Relaxed);
    // this will print on the server, like any server function
    println!("Adding {text:?} to the database!");
    if nth_run % 3 == 2 {
        Err(ServerFnError::new("Oh no! Couldn't add to database!"))
    } else {
        let mut rows = ROWS.lock().unwrap();
        rows.push(text);
        Ok(rows.len())
    }
}

/// Simply returns the number of rows.
#[server]
pub async fn get_rows() -> Result<usize, ServerFnError> {
    // insert a simulated wait
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    Ok(ROWS.lock().unwrap().len())
}

/// An action abstracts over the process of spawning a future and setting a signal when it
/// resolves. Its .input() signal holds the most recent argument while it's still pending,
/// and its .value() signal holds the most recent result. Its .version() signal can be fed
/// into a resource, telling it to refetch whenever the action has successfully resolved.
///
/// This makes actions useful for mutations, i.e., some server function that invalidates
/// loaded previously loaded from another server function.
#[component]
pub fn WithAnAction() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();

    // a server action can be created by using the server function's type name as a generic
    // the type name defaults to the PascalCased function name
    let action = ServerAction::<AddRow>::new();

    // this resource will hold the total number of rows
    // passing it action.version() means it will refetch whenever the action resolves successfully
    let row_count =
        Resource::new(move || action.version().get(), |_| get_rows());

    view! {
        <h3>Using <code>Action::new</code></h3>
        <p>
            "Some server functions are conceptually \"mutations,\", which change something on the server. "
            "These often work well as actions."
        </p>
        <input node_ref=input_ref placeholder="Type something here."/>
        <button on:click=move |_| {
            let text = input_ref.get().unwrap().value();
            action.dispatch(text.into());
        }>

            Submit
        </button>
        <p>You submitted: {move || format!("{:?}", action.input().get())}</p>
        <p>The result was: {move || format!("{:?}", action.value().get())}</p>
        <Transition>
            <p>Total rows: {row_count}</p>
        </Transition>
    }
}

/// An <ActionForm/> lets you do the same thing as dispatching an action, but automates the
/// creation of the dispatched argument struct using a <form>. This means it also gracefully
/// degrades well when JS/WASM are not available.
///
/// Try turning off WASM in your browser. The form still works, and successfully displays the error
/// message if the server function returns an error. Otherwise, it loads the new resource data.
#[component]
pub fn WithActionForm() -> impl IntoView {
    let action = ServerAction::<AddRow>::new();
    let row_count =
        Resource::new(move || action.version().get(), |_| get_rows());

    view! {
        <h3>Using <code>"<ActionForm/>"</code></h3>
        <p>
            <code>"<ActionForm/>"</code>
            "lets you use an HTML "
            <code>"<form>"</code>
            "to call a server function in a way that gracefully degrades."
        </p>
        <ActionForm action>
            <input
                // the `name` of the input corresponds to the argument name
                name="text"
                placeholder="Type something here."
            />
            <button>Submit</button>
        </ActionForm>
        <p>You submitted: {move || format!("{:?}", action.input().get())}</p>
        <p>The result was: {move || format!("{:?}", action.value().get())}</p>
        <Transition>
            archive underaligned: need alignment 4 but have alignment 1
            <p>Total rows: {row_count}</p>
        </Transition>
    }
}

/// The plain `#[server]` macro gives sensible defaults for the settings needed to create a server
/// function, but those settings can also be customized. For example, you can set a specific unique
/// path rather than the hashed path, or you can choose a different combination of input and output
/// encodings.
///
/// Arguments to the server macro can be specified as named key-value pairs, like `name = value`.
#[server(
    // this server function will be exposed at /api2/custom_path
    prefix = "/api2",
    endpoint = "custom_path",
    // it will take its arguments as a URL-encoded GET request (useful for caching)
    input = GetUrl
)]
// You can use the `#[middleware]` macro to add appropriate middleware
// In this case, any `tower::Layer` that takes services of `Request<Body>` will work
// #[middleware(crate::utils_and_structs::middleware::LoggingLayer)]
pub async fn length_of_input(input: String) -> Result<usize, ServerFnError> {
    println!("2. Running server function.");
    // insert a simulated wait
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    Ok(input.len())
}

#[component]
pub fn ServerFnArgumentExample() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let (result, set_result) = signal(0);

    view! {
        <h3>Custom arguments to the <code>#[server]</code> " macro"</h3>
        <p>This example shows how to specify additional behavior, including:</p>
        <ul>
            <li>Specific server function <strong>paths</strong></li>
            <li>Mixing and matching input and output <strong>encodings</strong></li>
            <li>Adding custom <strong>middleware</strong> on a per-server-fn basis</li>
        </ul>
        <input node_ref=input_ref placeholder="Type something here."/>
        <button on:click=move |_| {
            let value = input_ref.get().unwrap().value();
            spawn_local(async move {
                let length = length_of_input(value).await.unwrap_or(0);
                set_result.set(length);
            });
        }>

            Click to see length
        </button>
        <p>Length is {result}</p>
    }
}
