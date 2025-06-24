use crate::{components::*, pages::*};

use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_url,
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" dir="ltr" class="dark">
            <head>
                <title>Daimler Garay - Portfolio</title>
                <meta charset="utf-8"/>
                <meta name="description" content="Daimler Garay's portfolio website, created with Rust (Leptos)."/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <meta property="og:title" content="Daimler Garay – Portfolio"/>
                <meta property="og:description" content="Portfolio powered by Rust (Leptos)."/>
                <meta property="og:type" content="website"/>
                <link rel="icon" href="/favicon.ico" sizes="any"/>
                <link rel="manifest" href="/manifest.json"/>
                <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap"/>
                <link rel="stylesheet" href="/fontawesome/css/all.min.css"/>
                <link rel="stylesheet" id="leptos" href="/pkg/portfolio-rs.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options islands=true islands_router=true/>
            </head>
            <body
                class="font-inter antialiased bg-gradient-to-b from-slate-50 via-slate-100 to-slate-200 dark:from-slate-900 dark:via-slate-950 dark:to-black
                mx-auto px-4 sm:px-8 max-w-5xl min-h-screen transition-colors duration-300"
            >
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn Nav() -> impl IntoView {
    let url = use_url();
    let url_data = url.get();
    let current = url_data.path();

    view! {
        <nav class="sticky top-0 z-20 bg-white/80 dark:bg-slate-950/90 backdrop-blur border-b border-slate-200 dark:border-slate-800 shadow-sm">
            <div class="flex items-center justify-between h-16 max-w-5xl mx-auto px-4">
                <span class="text-xl font-bold tracking-tight text-slate-900 dark:text-white">
                    Portfolio
                </span>
                <div class="flex gap-6">
                    <NavLink
                        to="/"
                        active={current == "/"}
                        class="px-3 py-2 rounded-md text-sm font-medium transition hover:bg-slate-200/60 dark:hover:bg-slate-800/50"
                    >
                        "Home"
                    </NavLink>
                    <a
                        href="https://github.com/Daimler-Garay"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="px-3 py-2 rounded-md text-sm font-medium transition hover:bg-slate-200/60 dark:hover:bg-slate-800/50 flex items-center gap-1 text-slate-900 dark:text-white"
                        aria-label="Projects (GitHub)"
                    >
                        <i class="fab fa-github"></i> "Projects"
                    </a>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-slate-200 dark:border-slate-800 pt-8 mt-12 pb-4 text-slate-500 text-sm">
            <div class="flex flex-col items-center gap-2 text-center">
                <strong class="text-slate-800 dark:text-slate-100 mb-1">Contact & Socials</strong>
                <a
                    href="mailto:garay.daimlerchryslerfernandez@gmail.com"
                    class="inline-flex items-center gap-1 hover:underline"
                    title="garay.daimlerchryslerfernandez@gmail.com"
                >
                    <i class="fas fa-envelope"></i>
                    garay.daimlerchrysler...@gmail.com
                </a>
                <div class="flex gap-4 mt-1">
                    <a
                        href="https://github.com/Daimler-Garay"
                        class="inline-flex items-center gap-1 hover:text-primary transition-colors"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="GitHub"
                    >
                        <i class="fab fa-github"></i> GitHub
                    </a>
                    <span class="hidden sm:inline text-slate-400">|</span>
                    <a
                        href="https://www.linkedin.com/in/daimler-chrysler-406341243/"
                        class="inline-flex items-center gap-1 hover:text-primary transition-colors"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="LinkedIn"
                    >
                        <i class="fab fa-linkedin"></i> LinkedIn
                    </a>
                </div>
            </div>
            <div class="text-center pt-8 text-xs text-slate-400">
                2025 Daimler Garay. All rights reserved.
            </div>
        </footer>
    }
}

#[component]
pub fn FooterColumn(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 mb-8 md:mb-0">
            {children()}
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Nav />
            <main class="my-12">
                <Routes fallback=|| NotFound() transition=true>
                    <Route path=path!("/") view=|| view! { <Home/> }/>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
