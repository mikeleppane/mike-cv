use leptos::prelude::*;

#[component]
#[allow(clippy::too_many_lines)]
#[allow(clippy::must_use_candidate)]
pub fn Summary() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="prose prose-lg max-w-none text-gray-700">
                <p class="text-lg leading-relaxed font-medium text-gray-800 mb-4">
                    "Experienced Senior QA Engineer and Software Engineer with "
                    <span class="text-blue-600 font-semibold">"10 years"</span>
                    " of expertise in quality assurance, test automation, and software development.
                    Passionate about delivering high-quality software solutions and driving continuous improvement in development processes."
                </p>
                <p class="text-base leading-relaxed mb-4">
                    "I'm always looking for new opportunities and challenges.
                    I always strive for perseverance. My working morale is 
                    very high, and I'm committed to doing what I do. I 
                    always work relentlessly and hard to achieve my goals. I 
                    work thoroughly and have a high respect for deadlines.
                    I have a passion for learning. I believe in lifelong learning 
                    and staying updated with the latest technological 
                    advancements. Furthermore, I enjoy creating software 
                    that is clean, functional, and elegant."
                </p>
                <p class="text-base leading-relaxed">
                    " I strongly advocate Behavior-Driven Development BDD
                    and Domain-Driven Design DDD methodologies. BDD 
                    enables me to bridge the gap between business and 
                    technical teams, ensuring that software meets real user 
                    needs. I focus on modeling software around real 
                    business domains with DDD, resulting in maintainable 
                    and scalable solutions. These approaches allow me to 
                    deliver high-quality software that aligns closely with 
                    user needs and business goals. In addition, I strongly 
                    advocate the clean code paradigm, consistently writing 
                    code that is readable, simple, and easy to maintain. 
                    Clean code improves software quality and enhances 
                    team productivity and long-term project success."
                </p>
                <br />
                <p class="text-base leading-relaxed">
                    "My main programming language since 2008 has been
                    Python, which I have used extensively for software
                    development and test automation. I have also embraced
                    Rust in recent years, and I have been using it for about
                    four years. Learning Rust has deepened my
                    understanding of computer science concepts such as
                    memory management, concurrency, and system-level
                    programming. I enjoy leveraging its great type
                    system, safety, and performance features in my projects."
                </p>
                <br />
                <p class="text-base leading-relaxed">
                    "I've always been curious and jumped at the opportunity
                    to gain new, exciting experiences or take on a new
                    challenge. That is what motivates me and pushes me to
                    work harder.
                    Over fifteen years of experience in the industry have
                    taught me how to build relationships, communicate, and
                    get along with different types of people. I'm good at
                    working within a team and independently. I always share
                    my ideas and help other co-workers. As a worker, I'm
                    unprompted and systematic. My professional strengths
                    are in the fields of test automation and software
                    development."
                </p>
                <br />
                // add a nice quote section
                <div class="bg-gradient-to-r from-gray-50 to-gray-100 rounded-lg p-6 border-l-4 border-gray-400 my-6">
                    <div class="flex items-start">
                        <div class="flex-shrink-0">
                            <svg
                                class="w-8 h-8 text-gray-500"
                                fill="currentColor"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM6.293 6.707a1 1 0 010-1.414l3-3a1 1 0 011.414 0l3 3a1 1 0 11-1.414 1.414L10 4.414 7.707 6.707a1 1 0 01-1.414 0z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="ml-4 flex-1">
                            <blockquote class="text-lg italic text-gray-700 leading-relaxed mb-3">
                                "Software is not a product; it's a journey. The best software engineers are perpetual students."
                            </blockquote>
                            <cite class="text-base font-medium text-gray-600 not-italic">
                                "— Linus Torvalds"
                            </cite>
                        </div>
                    </div>
                </div>

                <div class="bg-gradient-to-r from-indigo-50 to-indigo-100 rounded-lg p-6 border-l-4 border-indigo-500 my-6">
                    <h4 class="text-lg font-semibold text-indigo-800 mb-4 flex items-center">
                        <svg
                            class="w-5 h-5 text-indigo-600 mr-2"
                            fill="currentColor"
                            viewBox="0 0 20 20"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M9.243 3.03a1 1 0 01.727 1.213L9.53 6h2.94l.56-2.243a1 1 0 111.94.486L14.53 6H17a1 1 0 110 2h-2.97l-1 4H15a1 1 0 110 2h-2.47l-.56 2.242a1 1 0 11-1.94-.485L10.47 14H7.53l-.56 2.242a1 1 0 11-1.94-.485L5.47 14H3a1 1 0 110-2h2.97l1-4H5a1 1 0 110-2h2.47l.56-2.243a1 1 0 011.213-.727zM9.03 8l-1 4h2.94l1-4H9.03z"
                                clip-rule="evenodd"
                            />
                        </svg>
                        "Core Technologies & Methodologies"
                    </h4>
                    <div class="flex flex-wrap gap-2">
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 border border-blue-200">
                            "Python"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-orange-100 text-orange-800 border border-orange-200">
                            "Rust"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 border border-blue-200">
                            "Azure"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800 border border-green-200">
                            "TDD"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800 border border-green-200">
                            "DDD"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800 border border-green-200">
                            "BDD"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-purple-100 text-purple-800 border border-purple-200">
                            "Software Architecture and Patterns"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-purple-100 text-purple-800 border border-purple-200">
                            "Cloud Native"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-indigo-100 text-indigo-800 border border-indigo-200">
                            "Clean Code"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-yellow-100 text-yellow-800 border border-yellow-200">
                            "Agile Development"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-red-100 text-red-800 border border-red-200">
                            "Testing"
                        </span>
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-pink-100 text-pink-800 border border-pink-200">
                            "QA"
                        </span>
                    </div>
                </div>

            </div>
        </div>
    }
}
