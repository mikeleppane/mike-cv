// filepath: /mike-cv-leptos/mike-cv-leptos/src/components/header.rs
use leptos::prelude::*;

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-blue-600 text-white p-4">
            <h1 class="text-2xl">"Mikko Leppänen's CV"</h1>
            <nav>
                <ul class="flex space-x-4">
                    <li>
                        <a href="#summary" class="hover:underline">
                            "Summary"
                        </a>
                    </li>
                    <li>
                        <a href="#experience" class="hover:underline">
                            "Experience"
                        </a>
                    </li>
                    <li>
                        <a href="#skills" class="hover:underline">
                            "Skills"
                        </a>
                    </li>
                    <li>
                        <a href="#education" class="hover:underline">
                            "Education"
                        </a>
                    </li>
                    <li>
                        <a href="#languages" class="hover:underline">
                            "Languages"
                        </a>
                    </li>
                    <li>
                        <a href="#github" class="hover:underline">
                            "GitHub"
                        </a>
                    </li>
                    <li>
                        <a href="#continuous-learning" class="hover:underline">
                            "Continuous Learning"
                        </a>
                    </li>
                    <li>
                        <a href="#training-courses" class="hover:underline">
                            "Training/Courses"
                        </a>
                    </li>
                    <li>
                        <a href="#certifications" class="hover:underline">
                            "Certifications"
                        </a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
