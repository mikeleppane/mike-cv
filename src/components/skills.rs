use leptos::prelude::*;

#[derive(Clone)]
pub struct SkillCategory {
    pub name: String,
    pub icon: String,
    pub skills: Vec<String>,
    pub color: String,
}

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn Skills() -> impl IntoView {
    let skill_categories = vec![
        SkillCategory {
            name: "Programming Languages".to_string(),
            icon: "💻".to_string(),
            color: "blue".to_string(),
            skills: vec![
                "Python".to_string(),
                "Rust".to_string(),
                "TypeScript".to_string(),
                "Go".to_string(),
                "C++".to_string(),
                "MatLab".to_string(),
                "Mojo🔥".to_string(),
            ],
        },
        SkillCategory {
            name: "Testing & QA".to_string(),
            icon: "🧪".to_string(),
            color: "emerald".to_string(),
            skills: vec![
                "Selenium".to_string(),
                "Pytest".to_string(),
                "Robot Framework".to_string(),
                "Cypress".to_string(),
                "Jest".to_string(),
                "Playwright".to_string(),
            ],
        },
        SkillCategory {
            name: "Cloud & DevOps".to_string(),
            icon: "☁️".to_string(),
            color: "purple".to_string(),
            skills: vec![
                "Azure".to_string(),
                "Docker".to_string(),
                "Kubernetes".to_string(),
                "Jenkins".to_string(),
                "GitLab CI/CD".to_string(),
                "GitHub Actions".to_string(),
                "Bicep".to_string(),
                "Ansible".to_string(),
            ],
        },
        SkillCategory {
            name: "Web Technologies".to_string(),
            icon: "🌐".to_string(),
            color: "orange".to_string(),
            skills: vec![
                "React".to_string(),
                "Node.js".to_string(),
                "HTML/CSS".to_string(),
                "REST APIs".to_string(),
                "GraphQL".to_string(),
                "Tailwind CSS".to_string(),
            ],
        },
    ];

    view! {
        <div class="space-y-5">
            {skill_categories
                .into_iter()
                .map(|category| {
                    let (bg_class, border_class, text_class, tag_bg, tag_text) = match category
                        .color
                        .as_str()
                    {
                        "blue" => {
                            (
                                "bg-blue-50",
                                "border-blue-200",
                                "text-blue-700",
                                "bg-blue-100",
                                "text-blue-800",
                            )
                        }
                        "emerald" => {
                            (
                                "bg-emerald-50",
                                "border-emerald-200",
                                "text-emerald-700",
                                "bg-emerald-100",
                                "text-emerald-800",
                            )
                        }
                        "purple" => {
                            (
                                "bg-purple-50",
                                "border-purple-200",
                                "text-purple-700",
                                "bg-purple-100",
                                "text-purple-800",
                            )
                        }
                        "orange" => {
                            (
                                "bg-orange-50",
                                "border-orange-200",
                                "text-orange-700",
                                "bg-orange-100",
                                "text-orange-800",
                            )
                        }
                        _ => {
                            (
                                "bg-gray-50",
                                "border-gray-200",
                                "text-gray-700",
                                "bg-gray-100",
                                "text-gray-800",
                            )
                        }
                    };

                    view! {
                        <div class=format!("rounded-lg border p-4 {} {}", bg_class, border_class)>
                            <div class="flex items-center mb-3">
                                <span class="text-xl mr-2">{category.icon}</span>
                                <h4 class=format!(
                                    "text-base font-semibold {}",
                                    text_class,
                                )>{category.name}</h4>
                            </div>

                            <div class="flex flex-wrap gap-2">
                                {category
                                    .skills
                                    .into_iter()
                                    .map(|skill| {
                                        view! {
                                            <span class=format!(
                                                "inline-flex items-center px-2.5 py-1 rounded-full text-xs font-medium border {} {} hover:scale-105 transition-transform duration-200",
                                                tag_bg,
                                                tag_text,
                                            )>{skill}</span>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()} // Tools & Methodologies
            <div class="mt-5 p-4 bg-gradient-to-r from-gray-50 to-gray-100 rounded-lg border border-gray-200">
                <h4 class="text-base font-semibold text-gray-800 mb-3 flex items-center">
                    <span class="text-xl mr-2">"🛠️"</span>
                    "Tools & Methodologies"
                </h4>

                <div class="flex flex-wrap gap-2">
                    {vec![
                        "Software Architecture and Patterns",
                        "Cloud Native",
                        "Clean Code",
                        "Git",
                        "JIRA",
                        "Confluence",
                        "VS Code",
                        "Agile",
                        "Scrum",
                        "BDD",
                        "TDD",
                        "CI/CD",
                        "DDD",
                        "AI",
                    ]
                        .into_iter()
                        .map(|tool| {
                            view! {
                                <span class="inline-flex items-center px-2.5 py-1 rounded-full text-xs font-medium bg-gray-200 text-gray-800 hover:bg-gray-300 transition-colors duration-200">
                                    {tool}
                                </span>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
