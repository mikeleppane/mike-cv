use leptos::prelude::*;

#[derive(Clone)]
pub struct EducationItem {
    pub degree: String,
    pub field: String,
    pub institution: String,
    pub location: String,
    pub duration: String,
    pub gpa: Option<String>,
    pub description: String,
    pub highlights: Vec<String>,
    pub relevant_courses: Vec<String>,
}

#[component]
#[allow(clippy::too_many_lines)]
#[allow(clippy::must_use_candidate)]
pub fn Education() -> impl IntoView {
    let education_items = vec![EducationItem {
        degree: "Master of Science".to_string(),
        field: "Electical Engineering".to_string(),
        institution: "Tampere University of Technology".to_string(),
        location: "Tampere, Finland".to_string(),
        duration: "2003 - 2009".to_string(),
        gpa: None,
        description:
            "Major in computational electromagnetics with minors in mathematics and physics."
                .to_string(),
        highlights: vec![],
        relevant_courses: vec![],
    }];

    view! {
        <div class="space-y-6">
            {education_items
                .into_iter()
                .map(|edu| {
                    view! {
                        <div class="relative">
                            // Timeline design
                            <div class="absolute left-0 top-0 bottom-0 w-px bg-gradient-to-b from-purple-500 to-violet-500 ml-6"></div>

                            // Timeline dot with graduation cap icon
                            <div class="absolute left-0 top-6 w-12 h-12 bg-gradient-to-br from-purple-500 to-violet-600 rounded-full ml-0 ring-4 ring-white shadow-lg flex items-center justify-center">
                                <svg
                                    class="w-6 h-6 text-white"
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                >
                                    <path d="M10.394 2.08a1 1 0 00-.788 0l-7 3a1 1 0 000 1.84L5.25 8.051a.999.999 0 01.356-.257l4-1.714a1 1 0 11.788 1.838L7.667 9.088l1.94.831a1 1 0 00.787 0l7-3a1 1 0 000-1.838l-7-3zM3.31 9.397L5 10.12v4.102a8.969 8.969 0 00-1.05-.174 1 1 0 01-.89-.89 11.115 11.115 0 01.25-3.762zM9.3 16.573A9.026 9.026 0 007 14.935v-3.957l1.818.78a3 3 0 002.364 0l5.508-2.361a11.026 11.026 0 01.25 3.762 1 1 0 01-.89.89 8.968 8.968 0 00-5.75 2.524 9.026 9.026 0 01-1 .001z" />
                                </svg>
                            </div>

                            // Content card
                            <div class="ml-16 bg-gradient-to-br from-white to-purple-50 rounded-lg shadow-md border border-purple-100 hover:shadow-lg transition-all duration-300 hover:scale-[1.01]">
                                <div class="p-6">
                                    // Header with degree info
                                    <div class="mb-4">
                                        <div class="flex items-start justify-between mb-2">
                                            <div class="flex-1">
                                                <h3 class="text-xl font-bold text-gray-900 mb-1">
                                                    {format!("{} in {}", edu.degree, edu.field)}
                                                </h3>
                                                <p class="text-purple-700 font-semibold text-lg">
                                                    {edu.institution}
                                                </p>
                                            </div>

                                            {if let Some(gpa) = &edu.gpa {
                                                view! {
                                                    <div class="bg-purple-100 text-purple-800 px-3 py-1 rounded-full text-sm font-medium">
                                                        <span class="text-xs text-purple-600">"GPA: "</span>
                                                        {gpa.clone()}
                                                    </div>
                                                }
                                                    .into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }}
                                        </div>

                                        <div class="flex items-center text-sm text-gray-600 space-x-4 mb-3">
                                            <div class="flex items-center">
                                                <svg
                                                    class="w-4 h-4 mr-1 text-gray-500"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                                <span class="font-medium">{edu.duration}</span>
                                            </div>
                                            <div class="flex items-center">
                                                <svg
                                                    class="w-4 h-4 mr-1 text-gray-500"
                                                    fill="currentColor"
                                                    viewBox="0 0 20 20"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                                <span>{edu.location}</span>
                                            </div>
                                        </div>
                                    </div>

                                    // Description
                                    <p class="text-gray-700 mb-4 leading-relaxed">
                                        {edu.description}
                                    </p>
                                </div>
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
