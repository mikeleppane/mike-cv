use leptos::prelude::*;
use leptos_meta::{Link, MetaTags, Stylesheet, Title, provide_meta_context};

use crate::components::{
    certifications::Certifications,
    education::Education,
    experience::Experience,
    github::GitHub,
    languages::Languages,
    learning::ContinuousLearning,
    personal_info::{PersonalInfo, PersonalInfoComponent},
    skills::Skills,
    summary::Summary,
};

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-gradient-to-br from-slate-50 to-gray-100 min-h-screen">
                <App />
            </body>
        </html>
    }
}

#[component]
#[must_use]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/cosmic-rust.css" />
        <Title text="Mikko Leppänen - CV" />

        <Link rel="icon" type_="image/png" sizes="64x64" href="/images/mike-cv-logo-64x64.png" />

        <div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100">
            // Professional header with gradient background and logo
            <header class="bg-gradient-to-r from-blue-900 via-blue-800 to-indigo-900 text-white shadow-2xl">
                <div class="container mx-auto px-6 py-8">
                    // Header title section - text and image
                    <div class="flex items-center justify-center mb-6 space-x-6">
                        <img
                            src="/images/mike-cv-header.png"
                            alt="Mikko Leppänen CV Logo"
                            class="w-16 h-16 lg:w-20 lg:h-20 rounded-full shadow-lg border-4 border-white/20"
                            style="filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.1));"
                        />
                        <h1 class="text-4xl font-bold text-white tracking-wide">"RESUME"</h1>
                    </div>

                    // Personal info section
                    <div class="py-4">
                        <PersonalInfoComponent info=PersonalInfo {
                            name: "Mikko Leppänen".to_string(),
                            title: "Senior QA Engineer & Software Engineer".to_string(),
                            email: "mleppan23@gmail.com".to_string(),
                            phone: "+358 50 4087703".to_string(),
                            address: "Tuusula, Finland".to_string(),
                            linkedin: "https://www.linkedin.com/in/mikko-lepp%C3%A4nen-05bb621a/"
                                .to_string(),
                            blog: "https://medium.com/@mleppan23".to_string(),
                            photo_url: "/images/mikko-leppanen-profile.jpg".to_string(),
                        } />
                    </div>
                </div>
            </header>
            // Main content with professional layout
            <main class="container mx-auto px-6 py-8 max-w-6xl">
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    // Left column - Main content
                    <div class="lg:col-span-2 space-y-8">
                        // Professional Summary
                        <section class="bg-gradient-to-br from-white via-blue-50 to-indigo-50 rounded-xl shadow-lg p-8 border border-blue-200 hover:shadow-xl transition-all duration-300 hover:scale-[1.02]">
                            <h2 class="text-3xl font-bold text-gray-800 mb-6 border-b-2 border-blue-500 pb-2">
                                "Professional Summary"
                            </h2>
                            <Summary />
                        </section>

                        // Experience
                        <section class="bg-gradient-to-br from-white via-emerald-50 to-green-50 rounded-xl shadow-lg p-8 border border-emerald-200 hover:shadow-xl transition-all duration-300 hover:scale-[1.02]">
                            <h2 class="text-3xl font-bold text-gray-800 mb-6 border-b-2 border-emerald-500 pb-2">
                                "Experience"
                            </h2>
                            <Experience />
                        </section>

                        // Education
                        <section class="bg-gradient-to-br from-white via-purple-50 to-violet-50 rounded-xl shadow-lg p-8 border border-purple-200 hover:shadow-xl transition-all duration-300 hover:scale-[1.02]">
                            <h2 class="text-3xl font-bold text-gray-800 mb-6 border-b-2 border-purple-500 pb-2">
                                "Education"
                            </h2>
                            <Education />
                        </section>

                        // Continuous Learning
                        <section class="bg-gradient-to-br from-white via-amber-50 to-orange-50 rounded-xl shadow-lg p-8 border border-amber-200 hover:shadow-xl transition-all duration-300 hover:scale-[1.02]">
                            <h2 class="text-3xl font-bold text-gray-800 mb-6 border-b-2 border-amber-500 pb-2">
                                "Continuous Learning & Professional Development"
                            </h2>
                            <ContinuousLearning />
                        </section>
                    </div>

                    // Right column - Sidebar
                    <div class="space-y-6">
                        // Skills
                        <section class="bg-gradient-to-br from-white via-green-50 to-emerald-50 rounded-xl shadow-lg p-6 border border-green-200 hover:shadow-xl transition-all duration-300 hover:scale-[1.02]">
                            <h3 class="text-2xl font-bold text-gray-800 mb-4 border-b-2 border-green-500 pb-2">
                                "Skills"
                            </h3>
                            <Skills />
                        </section>

                        // Languages
                        <section class="bg-gradient-to-br from-white via-purple-50 to-fuchsia-50 rounded-xl shadow-lg p-6 border border-purple-200 hover:shadow-xl transition-all duration-300 hover:scale-[1.02]">
                            <h3 class="text-2xl font-bold text-gray-800 mb-4 border-b-2 border-purple-500 pb-2">
                                "Languages"
                            </h3>
                            <Languages />
                        </section>

                        // GitHub
                        <section class="bg-gradient-to-br from-white via-gray-50 to-slate-50 rounded-xl shadow-lg p-6 border border-gray-200 hover:shadow-xl transition-all duration-300 hover:scale-[1.02]">
                            <h3 class="text-2xl font-bold text-gray-800 mb-4 border-b-2 border-gray-800 pb-2">
                                "GitHub"
                            </h3>
                            <GitHub />
                        </section>

                        // Certifications
                        <section class="bg-gradient-to-br from-white via-yellow-50 to-amber-50 rounded-xl shadow-lg p-6 border border-yellow-200 hover:shadow-xl transition-all duration-300 hover:scale-[1.02]">
                            <h3 class="text-2xl font-bold text-gray-800 mb-4 border-b-2 border-yellow-500 pb-2">
                                "Certifications"
                            </h3>
                            <Certifications />
                        </section>
                    </div>
                </div>
            </main>

            // Footer with gradient
            <footer class="bg-gradient-to-r from-gray-800 via-gray-900 to-slate-900 text-white py-8 mt-16">
                <div class="container mx-auto px-6 text-center">
                    <p class="text-gray-300">
                        "© 2025 Mikko Leppänen. Built with Rust 🦀 & Leptos "
                        <img
                            src="/images/leptos-logo.png"
                            alt="Leptos"
                            class="inline-block w-6 h-6 mx-1"
                        /> "."
                    </p>
                </div>
            </footer>
        </div>
    }
}
