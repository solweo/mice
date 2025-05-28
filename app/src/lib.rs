use leptos::{
    logging::log, 
    prelude::*, 
    task::spawn_local, 
    Params,
    html
};
use leptos_meta::{provide_meta_context, MetaTags, Title, Link, Meta, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
    hooks::use_query,
    params::Params,
};
use regex::Regex;
use serde::{Deserialize, Serialize};

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
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/mice-webapp.css"/>
        <Meta name="description" content="Note-taker for URLs"/>
        <Title text="Notes"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <link rel="preload" href="/triangular_lattice.svg" r#as="image" r#type="image/svg+xml"/>
        <link rel="preload" href="/stack.svg" r#as="image" r#type="image/svg+xml"/>
        
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

#[component]
fn HomePage() -> impl IntoView {
    let api_key = use_query::<ApiKey>();
    let (url, set_url) = signal("".to_string());

    let handle_send = move || {
        if let Ok(api_key) = api_key.read().as_ref() {
            let key = api_key.key.clone();
            let url = url.get();
            spawn_local(async {
                let _ = interop::send_to_inbox(key, url).await;
            });
        }
    };

    let input_element: NodeRef<html::Input> = NodeRef::new();
    Effect::new(move || {
        if let Some(input) = input_element.get() {
            input.focus().unwrap();
        }
    });

    let url_regex = Regex::new(r"(?i)^(https?|ftp):\/\/([a-z0-9-]+\.)+[a-z]{2,}(:\d+)?(\/\S*)?$").unwrap();
    let input_style = move || {
        if url_regex.is_match(&url.get()) {
            "color: blue; text-decoration: underline;"
        } else {
            ""
        }
    };

    use std::time::Duration;

    let hide_delay =  Duration::from_millis(300);
    let show_lock = RwSignal::new(true);
    let show_notes = RwSignal::new(false);
    let switch_show = move |_| {
        if show_lock.get() {
            show_lock.set(false);
            set_timeout(move || show_notes.set(true), hide_delay);
        } else {
            show_notes.set(false);
            set_timeout(move || show_lock.set(true), hide_delay);
        }
    };
    
    view! {
        <button
            on:click=switch_show
        >
            "Toggle"
        </button>
        
        <LockIcon/>
        <NotesIcon/>

        // `AnimatedShow` wrapper breaks translucent layer
        <AnimatedShow
            when=show_lock
            show_class="fade-in"
            hide_class="fade-out"
            hide_delay
        >
            <LockIcon/>
        </AnimatedShow>
        <AnimatedShow
            when=show_notes
            show_class="fade-in"
            hide_class="fade-out"
            hide_delay
        >
            <NotesIcon/>
        </AnimatedShow>

        <div><input type="url"
            placeholder="Paste your url note"
            style=input_style
            bind:value=(url, set_url)
            node_ref=input_element
            on:keydown= move |ev| if ev.key() == "Enter" && !url().is_empty() {
                handle_send();
                set_url(String::new());
            }
        /></div>
    }
}

#[component]
fn LockIcon() -> impl IntoView {
    view! {
        <div class="lock-icon">
            <div/>
            <div/>
            <div/>
        </div>
    }
}

#[component]
fn NotesIcon() -> impl IntoView {
    view! {
        <div class="notes-icon">
            <div/>
            <div/>
            <div/>
            <div/>
        </div>
    }
}