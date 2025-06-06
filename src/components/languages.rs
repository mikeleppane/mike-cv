use leptos::prelude::*;

#[derive(Clone)]
pub struct Language {
    pub name: String,
    pub proficiency: String,
    pub level: String, // "Native", "Fluent", "Advanced", "Intermediate", "Basic"
    pub flag_emoji: String,
    pub description: String,
}

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn Languages() -> impl IntoView {
    let languages = vec![
        Language {
            name: "Finnish".to_string(),
            proficiency: "Native".to_string(),
            level: "Native".to_string(),
            flag_emoji: "🇫🇮".to_string(),
            description: "Mother tongue with excellent written and verbal communication skills"
                .to_string(),
        },
        Language {
            name: "English".to_string(),
            proficiency: "Fluent".to_string(),
            level: "Advanced".to_string(),
            flag_emoji: "🇬🇧".to_string(),
            description: "Professional working proficiency in technical and business contexts"
                .to_string(),
        },
    ];

    view! {
        <div class="space-y-4">
            {languages
                .into_iter()
                .map(|lang| {
                    let (proficiency_bg, proficiency_text, proficiency_border, level_bg) = match lang
                        .proficiency
                        .as_str()
                    {
                        "Native" => {
                            ("bg-green-100", "text-green-800", "border-green-200", "bg-green-500")
                        }
                        "Fluent" => {
                            ("bg-blue-100", "text-blue-800", "border-blue-200", "bg-blue-500")
                        }
                        "Advanced" => {
                            (
                                "bg-purple-100",
                                "text-purple-800",
                                "border-purple-200",
                                "bg-purple-500",
                            )
                        }
                        "Intermediate" => {
                            (
                                "bg-yellow-100",
                                "text-yellow-800",
                                "border-yellow-200",
                                "bg-yellow-500",
                            )
                        }
                        _ => ("bg-gray-100", "text-gray-800", "border-gray-200", "bg-gray-500"),
                    };
                    let proficiency_stars = match lang.proficiency.as_str() {
                        "Native" => 5,
                        "Fluent" => 4,
                        "Advanced" => 3,
                        "Intermediate" => 2,
                        "Basic" => 1,
                        _ => 0,
                    };

                    view! {
                        <div class="bg-gradient-to-br from-white to-purple-50 rounded-lg p-4 border border-purple-200 hover:shadow-md transition-all duration-300 hover:scale-[1.01]">
                            <div class="flex items-start justify-between mb-3">
                                <div class="flex items-center space-x-3">
                                    <span class="text-2xl">{lang.flag_emoji}</span>
                                    <div>
                                        <h4 class="text-lg font-semibold text-gray-800">
                                            {lang.name}
                                        </h4>
                                        <div class="flex items-center space-x-2">
                                            <span class=format!(
                                                "px-2 py-1 rounded-full text-xs font-medium {proficiency_bg} {proficiency_text} {proficiency_border}",
                                            )>{lang.proficiency}</span>
                                            <span class="px-2 py-1 bg-gray-100 text-gray-700 rounded-full text-xs font-medium">
                                                {lang.level}
                                            </span>
                                        </div>
                                    </div>
                                </div>

                                // Proficiency level indicator
                                <div class="flex items-center space-x-1">
                                    {(1..=5)
                                        .map(|i| {
                                            let filled = i <= proficiency_stars;
                                            view! {
                                                <div class=format!(
                                                    "w-2 h-2 rounded-full {}",
                                                    if filled { level_bg } else { "bg-gray-300" },
                                                )></div>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            </div>

                            // Description
                            <p class="text-sm text-gray-600 leading-relaxed">{lang.description}</p>

                            // Progress bar
                            <div class="mt-3">
                                <div class="w-full bg-gray-200 rounded-full h-1.5">
                                    <div
                                        class=format!(
                                            "h-1.5 rounded-full transition-all duration-500 {}",
                                            level_bg,
                                        )
                                        style=format!("width: {}%", proficiency_stars * 20)
                                    ></div>
                                </div>
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
