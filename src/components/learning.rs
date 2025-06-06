use leptos::prelude::*;

#[derive(Clone)]
pub struct LearningPlatform {
    pub name: String,
    pub icon: String,
    pub url: Option<String>,
    pub description: String,
}

#[derive(Clone)]
pub struct LearningPath {
    pub title: String,
    pub platform: String,
    pub duration: String,
    pub completion_date: Option<String>,
    pub skills: Vec<String>,
    pub url: Option<String>,
}

#[derive(Clone)]
pub struct Conference {
    pub name: String,
    pub location: String,
    pub topics: Vec<String>,
}

#[component]
#[allow(clippy::too_many_lines)]
#[allow(clippy::must_use_candidate)]
pub fn ContinuousLearning() -> impl IntoView {
    let learning_platforms = vec![
        LearningPlatform {
            name: "O'Reilly Learning Platform".to_string(),
            icon: "📚".to_string(),
            url: Some("https://learning.oreilly.com/playlists/your-playlist-id/".to_string()),
            description: "Comprehensive tech learning platform with books, videos, and interactive content covering programming, DevOps, architecture, and emerging technologies".to_string(),
        },
        // Microsoft Learn
        LearningPlatform {
            name: "Microsoft Learn".to_string(),
            icon: "🖥️".to_string(),
            url: None,
            description: "Learning paths for Azure, DevOps and .NET & C#.".to_string(),
        },
        // A Cloud Guru
        LearningPlatform {
            name: "A Cloud Guru".to_string(),
            icon: "☁️".to_string(),
            url: None,
            description: "Cloud-focused learning platform specializing in AWS, Azure, and Google Cloud with hands-on labs and certification preparation".to_string(),
        },
        // Frontend Masters
        LearningPlatform {
            name: "Frontend Masters".to_string(),
            icon: "🎨".to_string(),
            url: None,
            description: "Advanced courses on JavaScript, TypeScript, and testing frameworks".to_string(),
        }
    ];

    let conferences = vec![
        Conference {
            name: "Euro Python".to_string(),
            location: "Virtual".to_string(),
            topics: vec!["Python".to_string()],
        },
        Conference {
            name: "PyCon US".to_string(),
            location: "Virtual".to_string(),
            topics: vec!["Python".to_string()],
        },
        Conference {
            name: "PyCon Italia".to_string(),
            location: "Virtual".to_string(),
            topics: vec!["Python".to_string()],
        },
        Conference {
            name: "RustConf".to_string(),
            location: "Virtual".to_string(),
            topics: vec!["Rust Programming Language".to_string()],
        },
        Conference {
            name: "Rust Nation UK".to_string(),
            location: "Virtual".to_string(),
            topics: vec!["Rust Programming Language".to_string()],
        },
        Conference {
            name: "RustLang".to_string(),
            location: "Virtual".to_string(),
            topics: vec!["Rust Programming Language".to_string()],
        },
        Conference {
            name: "EuroRust".to_string(),
            location: "Virtual".to_string(),
            topics: vec!["Rust Programming Language".to_string()],
        },
        Conference {
            name: "GOTO Conferences".to_string(),
            location: "Virtual".to_string(),
            topics: vec![
                "Software Architecture".to_string(),
                "DevOps".to_string(),
                "Agile".to_string(),
            ],
        },
        Conference {
            name: "Domain-Driven Design Europe".to_string(),
            location: "Virtual".to_string(),
            topics: vec![
                "Domain-Driven Design".to_string(),
                "Software Architecture".to_string(),
            ],
        },
    ];

    view! {
        <div class="space-y-8">
            // Learning Platforms Section
            <div>
                <h3 class="text-xl font-semibold text-gray-800 mb-4 flex items-center">
                    <span class="text-2xl mr-2">"🎓"</span>
                    "Learning Platforms"
                </h3>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {learning_platforms
                        .into_iter()
                        .map(|platform| {
                            view! {
                                <div class="bg-gradient-to-br from-white to-amber-50 rounded-lg p-4 border border-amber-200 hover:shadow-md transition-all duration-300 hover:scale-[1.02]">
                                    <div class="flex items-start justify-between mb-2">
                                        <div class="flex items-center">
                                            <span class="text-xl mr-2">{platform.icon}</span>
                                            <h4 class="font-semibold text-gray-800">{platform.name}</h4>
                                        </div>
                                    </div>
                                    <p class="text-sm text-gray-600 mb-3">{platform.description}</p>
                                    {if platform.url.is_some() {
                                        view! {
                                            <span>"View My O'Reilly Playlists:"</span>
                                            <a
                                                href="https://learning.oreilly.com/playlists/c6532a58-5bb4-4ca6-803b-25879d046ed4"
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="inline-flex items-center text-sm text-amber-700 hover:text-amber-800 font-medium"
                                            >
                                                "Elevate Your Skills: Become a Python Expert"
                                                <svg
                                                    class="w-3 h-3 ml-1"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                            </a>
                                            <a
                                                href="https://learning.oreilly.com/playlists/32e08149-c20b-4302-bb63-9be25b642e1c"
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="inline-flex items-center text-sm text-amber-700 hover:text-amber-800 font-medium"
                                            >
                                                "Elevate Your Skills: Become A Rust Professional"
                                                <svg
                                                    class="w-3 h-3 ml-1"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                            </a>
                                            <a
                                                href="https://learning.oreilly.com/playlists/d0920989-49cd-483d-98c8-ccbed8f601de"
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="inline-flex items-center text-sm text-amber-700 hover:text-amber-800 font-medium"
                                            >
                                                "Elevate Your Skills: Code Craftsmanship Essentials"
                                                <svg
                                                    class="w-3 h-3 ml-1"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                            </a>
                                            <a
                                                href="https://learning.oreilly.com/playlists/afbda73b-0cef-4858-bf1f-7ffd7ace7b25"
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="inline-flex items-center text-sm text-amber-700 hover:text-amber-800 font-medium"
                                            >
                                                "Elevate Your Skills: Docker & Kubernetes"
                                                <svg
                                                    class="w-3 h-3 ml-1"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                            </a>
                                            <a
                                                href="https://learning.oreilly.com/playlists/4bb46fb2-8b27-42ba-877e-f9d2f7d85879"
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="inline-flex items-center text-sm text-amber-700 hover:text-amber-800 font-medium"
                                            >
                                                "Elevate Your Skills: Event-Driven Architecture"
                                                <svg
                                                    class="w-3 h-3 ml-1"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                            </a>
                                            <a
                                                href="https://learning.oreilly.com/playlists/9960f764-8cd7-4ceb-a69b-6bbbdbdda115"
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="inline-flex items-center text-sm text-amber-700 hover:text-amber-800 font-medium"
                                            >
                                                "Elevate Your Skills: Modern DevOps & Delivery"
                                                <svg
                                                    class="w-3 h-3 ml-1"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                            </a>
                                            <a
                                                href="https://learning.oreilly.com/playlists/1704292b-a20a-4897-a975-687ad32c15a6"
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="inline-flex items-center text-sm text-amber-700 hover:text-amber-800 font-medium"
                                            >
                                                "Elevate Your Skills: Software Architectures and Patterns"
                                                <svg
                                                    class="w-3 h-3 ml-1"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                            </a>
                                        }
                                            .into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
            // Conferences & Events
            <div>
                <h3 class="text-xl font-semibold text-gray-800 mb-4 flex items-center">
                    <span class="text-2xl mr-2">"🎤"</span>
                    "Conferences & Events"
                </h3>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {conferences
                        .into_iter()
                        .map(|conf| {
                            view! {
                                <div class="bg-gradient-to-br from-white to-amber-50 rounded-lg p-4 border border-amber-200 hover:shadow-md transition-all duration-300">
                                    <h4 class="font-semibold text-gray-800 mb-2">{conf.name}</h4>
                                    <p class="text-sm text-gray-600 mb-2 flex items-center">
                                        <svg
                                            class="w-4 h-4 mr-1"
                                            fill="currentColor"
                                            viewBox="0 0 20 20"
                                        >
                                            <path
                                                fill-rule="evenodd"
                                                d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z"
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                        {conf.location}
                                    </p>
                                    <div class="flex flex-wrap gap-1">
                                        {conf
                                            .topics
                                            .into_iter()
                                            .map(|topic| {
                                                view! {
                                                    <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-amber-100 text-amber-800">
                                                        {topic}
                                                    </span>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
