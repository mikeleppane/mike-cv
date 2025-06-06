use leptos::prelude::*;

#[derive(Clone)]
pub struct PersonalInfo {
    pub name: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub linkedin: String,
    pub blog: String,
    pub photo_url: String, // Add photo URL field
}

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn PersonalInfoComponent(info: PersonalInfo) -> impl IntoView {
    let photo_url = info.photo_url.clone();
    let email = info.email.clone();
    let phone = info.phone.clone();

    view! {
        <div class="text-center lg:text-left">
            <div class="flex flex-col lg:flex-row lg:items-center lg:justify-between">
                // Left side - Photo and basic info
                <div class="flex flex-col lg:flex-row lg:items-center mb-6 lg:mb-0">
                    // Professional photo with modern styling
                    <div class="mb-6 lg:mb-0 lg:mr-8">
                        <div class="relative">
                            <div class="w-32 h-32 lg:w-40 lg:h-40 mx-auto lg:mx-0 rounded-full overflow-hidden ring-4 ring-white/30 shadow-2xl">
                                <img
                                    src=photo_url
                                    alt=format!("{} - Profile Photo", info.name)
                                    class="w-full h-full object-cover object-center"
                                />
                            </div>
                            // Decorative gradient overlay
                            <div class="absolute -inset-2 bg-gradient-to-r from-blue-400 to-purple-500 rounded-full opacity-20 blur-sm -z-10"></div>
                        </div>
                    </div>

                    // Name and title
                    <div class="flex-1">
                        <h1 class="text-4xl lg:text-6xl font-bold mb-2 text-white drop-shadow-lg">
                            {info.name}
                        </h1>
                        <p class="text-lg lg:text-2xl text-blue-100 font-light mb-4">
                            {info.title}
                        </p>
                    </div>
                </div>

                // Right side - Contact information card
                <div class="bg-white/10 backdrop-blur-sm rounded-xl p-6 border border-white/20 shadow-xl">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-blue-100">
                        <div class="flex items-center space-x-3">
                            <svg
                                class="w-5 h-5 text-blue-300 flex-shrink-0"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" />
                                <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
                            </svg>
                            <a
                                href=format!("mailto:{}", email)
                                class="text-sm font-medium hover:text-white transition-colors"
                            >
                                {info.email}
                            </a>
                        </div>

                        <div class="flex items-center space-x-3">
                            <svg
                                class="w-5 h-5 text-blue-300 flex-shrink-0"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path d="M2 3a1 1 0 011-1h2.153a1 1 0 01.986.836l.74 4.435a1 1 0 01-.54 1.06l-1.548.773a11.037 11.037 0 006.105 6.105l.774-1.548a1 1 0 011.059-.54l4.435.74a1 1 0 01.836.986V17a1 1 0 01-1 1h-2C7.82 18 2 12.18 2 5V3z" />
                            </svg>
                            <a
                                href=format!("tel:{}", phone)
                                class="text-sm font-medium hover:text-white transition-colors"
                            >
                                {info.phone}
                            </a>
                        </div>

                        <div class="flex items-center space-x-3">
                            <svg
                                class="w-5 h-5 text-blue-300 flex-shrink-0"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                            <span class="text-sm font-medium">{info.address}</span>
                        </div>

                        <div class="flex items-center space-x-3">
                            <svg
                                class="w-5 h-5 text-blue-300 flex-shrink-0"
                                fill="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" />
                            </svg>
                            <a
                                href=info.linkedin.clone()
                                class="text-sm font-medium hover:text-white transition-colors"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "LinkedIn"
                            </a>
                        </div>

                        <div class="flex items-center space-x-3 md:col-span-2">
                            <svg
                                class="w-5 h-5 text-blue-300 flex-shrink-0"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M4.083 9h1.946c.089-1.546.383-2.97.837-4.118A6.004 6.004 0 004.083 9zM10 2a8 8 0 100 16 8 8 0 000-16zm0 2c-.076 0-.232.032-.465.262-.238.234-.497.623-.737 1.182-.389.907-.673 2.142-.766 3.556h3.936c-.093-1.414-.377-2.649-.766-3.556-.24-.56-.5-.948-.737-1.182C10.232 4.032 10.076 4 10 4zm3.971 5c-.089-1.546-.383-2.97-.837-4.118A6.004 6.004 0 0115.917 9h-1.946zm-2.003 2H8.032c.093 1.414.377 2.649.766 3.556.24.56.5.948.737 1.182.233.23.389.262.465.262.076 0 .232-.032.465-.262.238-.234.498-.623.737-1.182.389-.907.673-2.142.766-3.556zm1.166 4.118c.454-1.147.748-2.572.837-4.118h1.946a6.004 6.004 0 01-2.783 4.118zm-6.268 0C6.412 13.97 6.118 12.546 6.03 11H4.083a6.004 6.004 0 002.783 4.118z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                            <a
                                href=info.blog.clone()
                                class="text-sm font-medium hover:text-white transition-colors"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "Blog"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
