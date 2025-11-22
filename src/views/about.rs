use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const ABOUT_DATA: &str = include_str!("../../assets/data/about.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct AboutData {
    name: String,
    title: String,
    location: String,
    bio: String,
    skills: Vec<String>,
    experience: Vec<Experience>,
    interests: Vec<String>,
    contact: Contact,
    updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Experience {
    company: String,
    position: String,
    duration: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Contact {
    email: String,
    website: String,
    github: String,
}

#[component]
pub fn About() -> Element {
    // Parse the JSON data at compile time
    let about_data = use_memo(move || {
        serde_json::from_str::<AboutData>(ABOUT_DATA).unwrap_or_else(|e| {
            eprintln!("Failed to parse about.json: {}", e);
            AboutData {
                name: "Error Loading Data".to_string(),
                title: "".to_string(),
                location: "".to_string(),
                bio: "Failed to load about information.".to_string(),
                skills: vec![],
                experience: vec![],
                interests: vec![],
                contact: Contact {
                    email: "".to_string(),
                    website: "".to_string(),
                    github: "".to_string(),
                },
                updated: "".to_string(),
            }
        })
    });

    let data = about_data();

    rsx! {
        div {
            class: "max-w-4xl mx-auto p-6 space-y-8",

            // Header Section
            header {
                class: "text-center border-b pb-8",
                h1 {
                    class: "text-4xl font-bold text-gray-800 mb-2",
                    "{data.name}"
                }
                h2 {
                    class: "text-xl text-gray-600 mb-2",
                    "{data.title}"
                }
                p {
                    class: "text-gray-500",
                    "📍 {data.location}"
                }
            }

            // Bio Section
            section {
                class: "bg-gray-50 rounded-lg p-6",
                h3 {
                    class: "text-2xl font-semibold text-gray-800 mb-4",
                    "About Me"
                }
                p {
                    class: "text-gray-700 leading-relaxed text-lg",
                    "{data.bio}"
                }
            }

            // Skills Section
            section {
                class: "space-y-4",
                h3 {
                    class: "text-2xl font-semibold text-gray-800 mb-4",
                    "Skills"
                }
                div {
                    class: "flex flex-wrap gap-2",
                    for skill in data.skills {
                        span {
                            key: "{skill}",
                            class: "px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium",
                            "{skill}"
                        }
                    }
                }
            }

            // Experience Section
            section {
                class: "space-y-4",
                h3 {
                    class: "text-2xl font-semibold text-gray-800 mb-4",
                    "Experience"
                }
                for exp in data.experience {
                    div {
                        key: "{exp.company}-{exp.position}",
                        class: "bg-white border border-gray-200 rounded-lg p-6 shadow-sm",
                        div {
                            class: "flex justify-between items-start mb-2",
                            div {
                                h4 {
                                    class: "text-xl font-semibold text-gray-800",
                                    "{exp.position}"
                                }
                                p {
                                    class: "text-gray-600 font-medium",
                                    "{exp.company}"
                                }
                            }
                            span {
                                class: "text-sm text-gray-500 bg-gray-100 px-2 py-1 rounded",
                                "{exp.duration}"
                            }
                        }
                        p {
                            class: "text-gray-700",
                            "{exp.description}"
                        }
                    }
                }
            }

            // Interests Section
            section {
                class: "space-y-4",
                h3 {
                    class: "text-2xl font-semibold text-gray-800 mb-4",
                    "Interests"
                }
                ul {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-2",
                    for interest in data.interests {
                        li {
                            key: "{interest}",
                            class: "flex items-center text-gray-700",
                            span { class: "mr-2", "•" }
                            "{interest}"
                        }
                    }
                }
            }

            // Contact Section
            section {
                class: "bg-gradient-to-r from-blue-50 to-purple-50 rounded-lg p-6",
                h3 {
                    class: "text-2xl font-semibold text-gray-800 mb-4",
                    "Contact"
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-4",

                    div {
                        class: "flex items-center space-x-2",
                        span { class: "text-2xl", "📧" }
                        a {
                            href: "mailto:{data.contact.email}",
                            class: "text-blue-600 hover:text-blue-800 underline",
                            "{data.contact.email}"
                        }
                    }

                    div {
                        class: "flex items-center space-x-2",
                        span { class: "text-2xl", "🌐" }
                        a {
                            href: "{data.contact.website}",
                            target: "_blank",
                            class: "text-blue-600 hover:text-blue-800 underline",
                            "Website"
                        }
                    }

                    div {
                        class: "flex items-center space-x-2",
                        span { class: "text-2xl", "⚡" }
                        a {
                            href: "https://github.com/{data.contact.github}",
                            target: "_blank",
                            class: "text-blue-600 hover:text-blue-800 underline",
                            "GitHub"
                        }
                    }
                }
            }

            // Footer
            footer {
                class: "text-center text-sm text-gray-500 pt-8 border-t",
                p { "Last updated: {data.updated}" }
                p {
                    class: "mt-1",
                    "Generated statically with Dioxus SSG 🦀"
                }
            }
        }
    }
}
