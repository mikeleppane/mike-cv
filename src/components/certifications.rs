use leptos::prelude::*;

#[derive(Clone)]
pub struct Certification {
    pub name: String,
    pub issuer: String,
}

#[component]
#[allow(clippy::must_use_candidate)]
pub fn Certifications() -> impl IntoView {
    let certifications = vec![
        Certification {
            name: "AZ104 - Microsoft Azure Administrator AZ104 
Microsoft Azure Administrator"
                .to_string(),
            issuer: "Tieturi".to_string(),
        },
        Certification {
            name: "Microsoft Applied Skills: Configure secure access 
to your workloads using Azure networking"
                .to_string(),
            issuer: "Microsoft".to_string(),
        },
        Certification {
            name: "Microsoft enabled skills: Secure storage of Azure 
Files and Azure Blob storage"
                .to_string(),
            issuer: "Microsoft".to_string(),
        },
        Certification {
            name: "The Ultimate Kubernetes Administrator Course | 
CKA"
            .to_string(),
            issuer: " TechWorld with Nana".to_string(),
        },
        Certification {
            name: "DevOps with Kubernetes".to_string(),
            issuer: "University of Helsinki".to_string(),
        },
        Certification {
            name: "DevOps with Docker".to_string(),
            issuer: "University of Helsinki".to_string(),
        },
        Certification {
            name: "Full Stack Open  Full Stack Web Development: 
Core + extensions 12"
                .to_string(),
            issuer: "University of Helsinki".to_string(),
        },
    ];

    view! {
        <div class="space-y-4">
            {certifications
                .into_iter()
                .map(|cert| {
                    view! {
                        <div class="bg-gradient-to-br from-white to-yellow-50 rounded-lg p-4 border border-yellow-200 hover:shadow-md transition-all duration-300 hover:scale-[1.01]">
                            <div class="flex items-center space-x-3">
                                // Certificate icon
                                <div class="flex-shrink-0">
                                    <div class="w-12 h-12 bg-gradient-to-br from-yellow-400 to-amber-500 rounded-lg flex items-center justify-center shadow-sm">
                                        <svg
                                            class="w-6 h-6 text-white"
                                            fill="currentColor"
                                            viewBox="0 0 20 20"
                                        >
                                            <path
                                                fill-rule="evenodd"
                                                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                    </div>
                                </div>

                                // Content
                                <div class="flex-1">
                                    <h4 class="text-lg font-semibold text-gray-800 leading-tight">
                                        {cert.name}
                                    </h4>
                                    <p class="text-yellow-700 font-medium text-sm">{cert.issuer}</p>
                                </div>
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
