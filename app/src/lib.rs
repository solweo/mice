use interop::Authenticate;
use leptos::{
    logging::log, 
    prelude::*, 
    task::spawn_local, 
    Params,
    html
};
use leptos_meta::{provide_meta_context, MetaTags, Title, Link, Meta, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes}, hooks::{use_navigate, use_query}, params::Params, StaticSegment
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
    let key_from_query = {
        let api_key = use_query::<ApiKey>();
        move || { 
            api_key.read()
                .as_ref()
                .ok()
                .map(|api_key| api_key.key.clone()) 
        }
    };

    let (key_from_input, set_key_on_input) = signal(String::new());

    let authenticated = {
        let (key, set_key) = signal(String::new());

        Effect::new(move || {
            if let Some(key) = key_from_query() {
                set_key(key);
            }
        });

        Resource::new(key, |key| async move {
            interop::authenticate(key)
                .await
                .is_ok()
        })
    };

    use std::time::Duration;

    let hide_delay =  Duration::from_millis(300);
    let show_lock = RwSignal::new(false);
    let show_notes = RwSignal::new(false);

    Effect::new(move || {
        if let Some(true) = authenticated.get() {
            show_lock.set(false);
            set_timeout(move || if let Some(true) = authenticated.get_untracked() { show_notes.set(true) }, hide_delay);
        } 
        else {
            show_notes.set(false);
            set_timeout(move || if let Some(false) = authenticated.get_untracked() { show_lock.set(true) }, hide_delay);
        }
    });


    Effect::new(move || {
        log!("auth: {:?}", authenticated.get());
        log!("lock: {:?}", show_lock.get());
        log!("notes: {:?}", show_notes.get());
        log!("---------");
    });

    let (url, set_url) = signal(String::new());
    
    let handle_send = move || {
        if let Some(key) = key_from_query() {
            let url = url.get();
            spawn_local(async {
                let _ = interop::send_to_inbox(key, url).await;
            });
        }
    };
    
    let input_element: NodeRef<html::Input> = NodeRef::new();
    // Effect::new(move || {
    //     if let Some(input) = input_element.get() {
    //         input.focus().unwrap();
    //     }
    // });

    view! {
        
        // <LockIcon/>
        // <NotesIcon/>
        
        // `AnimatedShow` wrapper breaks translucent layer
        <AnimatedShow
            when=show_lock
            show_class="fade-in"
            hide_class="fade-out"
            hide_delay
        >
            <LockIcon/>
            <div><input type="url"
                placeholder="Enter passphrase"
                bind:value=(key_from_input, set_key_on_input)
                on:keydown= {
                    let navigate = use_navigate();
                    move |ev| if ev.key() == "Enter" && !key_from_input().is_empty() {
                        let url = format!("/?key={}", key_from_input());
                        set_key_on_input(String::new());
                        navigate(&url, Default::default());
                    }
                }
            /></div>
        </AnimatedShow>
        <AnimatedShow
            when=show_notes
            show_class="fade-in"
            hide_class="fade-out"
            hide_delay
        >
            <NotesIcon/>
            <div><input type="url"
                placeholder="Paste your url note"
                style={
                    let url_regex = Regex::new(r"(?i)^(https?|ftp):\/\/([a-z0-9-]+\.)+[a-z]{2,}(:\d+)?(\/\S*)?$").unwrap();
                    move || {
                        if url_regex.is_match(&url.get()) {
                            "color: blue; text-decoration: underline;"
                        } else {
                            ""
                        }
                    }
                }
                bind:value=(url, set_url)
                node_ref=input_element
                on:keydown= move |ev| if ev.key() == "Enter" && !url().is_empty() {
                    handle_send();
                    set_url(String::new());
                }
            /></div>

        </AnimatedShow>

        
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