use leptos::prelude::*;

#[derive(Clone)]
pub struct ExperienceItem {
    pub company: String,
    pub position: String,
    pub duration: String,
    pub location: String,
    pub description: String,
    pub achievements: Vec<String>,
    pub technologies: Vec<String>,
    pub company_logo: Option<String>,
}

#[component]
#[must_use]
#[allow(clippy::too_many_lines)]
#[allow(clippy::must_use_candidate)]
pub fn Experience() -> impl IntoView {
    let experiences = vec![
        ExperienceItem {
            company: "S-Pankki Oy".to_string(),
            position: "Senior QA Engineer".to_string(),
            duration: "2024 - Present".to_string(),
            location: "Helsinki, Finland".to_string(),
            description: "Lead test automation activities. Responsible for planning and implementing automated test cases and, in addition, implement testing libraries and CI/CD pipelines.".to_string(),
            achievements: vec![],
            technologies: vec![
                "Python".to_string(),
                "Robot Framework".to_string(),
                "Selenium".to_string(),
                "Docker".to_string(),
                "Jenkins".to_string(),
                "GitLab CI/CD".to_string(),
                "Azure".to_string(),
                "Jira".to_string(),

            ],
            company_logo: None
        },
        ExperienceItem {
            company: "Vaisala Oyj".to_string(),
            position: "Software Engineer".to_string(),
            duration: "2022 - 2024".to_string(),
            location: "Vantaa, Finland".to_string(),
            description: "Built weather monitoring system for the airport using Python.".to_string(),
            technologies: vec![
                "Python".to_string(),
                "FastAPI".to_string(),
                "React".to_string(),
                "TypeScript".to_string(),
                "PostgreSQL".to_string(),
                "GitLab CI/CD".to_string(),
                "Docker".to_string(),
                "RabbitMQ".to_string(),
                "Playwright".to_string(),

            ],
            company_logo: None,
            achievements: vec![],
        },
        ExperienceItem {
            company: "KONE Oyj".to_string(),
            position: "Software Test Automation Specialist - Embedded Software And IoT Systems".to_string(),
            duration: "2016 - 2022".to_string(),
            location: "Hyvinkää, Finland".to_string(),
            description: "Implementing test automation solutions for KONE embedded and IoT systems.".to_string(),
            technologies: vec![
                "Python".to_string(),
                "Robot Framework".to_string(),
                "Jenkins".to_string(),
                "Docker".to_string(),
                "Qt".to_string(),
            ],
            company_logo: None,
            achievements: vec![],
        },
        ExperienceItem {
            company: "Vacon Oyj".to_string(),
            position: "Development Engineer".to_string(),
            duration: "2008 - 2015".to_string(),
            location: "Vaasa, Finland".to_string(),
            description: "Hardware engineer for Vacon frequency inverters".to_string(),
            technologies: vec![
                "Matlab".to_string(),
            ],
            company_logo: None,
            achievements: vec![],
        },
    ];

    view! {
        <div class="space-y-8">
            {experiences
                .into_iter()
                .map(|exp| {
                    view! {
                        <div class="relative">
                            // Timeline connector
                            <div class="absolute left-0 top-0 bottom-0 w-px bg-gradient-to-b from-emerald-500 to-green-500 ml-6"></div>

                            // Timeline dot
                            <div class="absolute left-0 top-6 w-3 h-3 bg-emerald-500 rounded-full ml-5 ring-4 ring-white shadow-lg"></div>

                            // Content card
                            <div class="ml-16 bg-gradient-to-br from-white to-emerald-50 rounded-lg shadow-md border border-emerald-100 hover:shadow-lg transition-all duration-300 hover:scale-[1.01]">
                                <div class="p-6">
                                    // Header with company info
                                    <div class="flex items-start justify-between mb-4">
                                        <div class="flex-1">
                                            <div class="flex items-center mb-2">
                                                {if let Some(logo) = &exp.company_logo {
                                                    view! {
                                                        <img
                                                            src=logo.clone()
                                                            alt=format!("{} logo", exp.company)
                                                            class="w-8 h-8 rounded mr-3 object-cover"
                                                        />
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {
                                                        <div class="w-8 h-8 bg gradient-to-br from-emerald-500 to-green-600 rounded mr-3 flex items-center justify-center">
                                                            <span class="text-white text-xs font-bold">
                                                                {exp.company.chars().next().unwrap_or('C')}
                                                            </span>
                                                        </div>
                                                    }
                                                        .into_any()
                                                }} <div>
                                                    <h3 class="text-xl font-bold text-gray-900">
                                                        {exp.position}
                                                    </h3>
                                                    <p class="text-emerald-700 font-semibold">{exp.company}</p>
                                                </div>
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
                                                    <span class="font-medium">{exp.duration}</span>
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
                                                    <span>{exp.location}</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    // Job description
                                    <p class="text-gray-700 mb-4 leading-relaxed">
                                        {exp.description}
                                    </p>

                                    // Technologies used
                                    <div>
                                        <h4 class="text-sm font-semibold text-gray-800 mb-2 flex items-center">
                                            <svg
                                                class="w-4 h-4 mr-1 text-emerald-600"
                                                fill="currentColor"
                                                viewBox="0 0 20 20"
                                            >
                                                <path
                                                    fill-rule="evenodd"
                                                    d="M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z"
                                                    clip-rule="evenodd"
                                                />
                                            </svg>
                                            "Technologies & Tools"
                                        </h4>
                                        <div class="flex flex-wrap gap-2">
                                            {exp
                                                .technologies
                                                .into_iter()
                                                .map(|tech| {
                                                    view! {
                                                        <span class="inline-flex items-center px-2.5 py-1 rounded-full text-xs font-medium bg-emerald-100 text-emerald-800 border border-emerald-200">
                                                            {tech}
                                                        </span>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
