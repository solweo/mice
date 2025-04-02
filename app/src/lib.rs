use leptos::{
    logging::log, 
    prelude::*, 
    task::spawn_local, 
    Params
};
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
    hooks::use_query,
    params::Params,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="Hi!"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}


#[derive(Params, PartialEq, Debug, Clone)]
struct ApiKey {
    key: String,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let api_key = use_query::<ApiKey>();

    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| {
        // let api_key = api_key.read().as_ref();
        if let Ok(api_key) = api_key.read().as_ref() {
            let key = api_key.key.clone();
            spawn_local(async {
                let _ = send_to_inbox(key).await;
            });
        }
        *count.write() += 1
    };

    view! {
        <h1>"Hi!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[server]
pub async fn send_to_inbox(api_key: String) -> Result<(), ServerFnError> {
    let required_key = std::env::var("MY_SECRET_API_KEY")?;
    if required_key == api_key {
        log!("Request was authenticated");
        Ok(())
    } else {
        Ok(())
    }
}