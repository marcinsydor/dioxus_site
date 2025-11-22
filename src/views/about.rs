use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const ABOUT_DATA: &str = include_str!("../../assets/data/about.json");
const ABOUT_CSS: Asset = asset!("/assets/styling/about.css");

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
        document::Link { rel: "stylesheet", href: ABOUT_CSS }

        div {
            class: "about-container",

            // Header Section
            header {
                class: "about-header",
                h1 {
                    class: "about-name",
                    "{data.name}"
                }
                h2 {
                    class: "about-title",
                    "{data.title}"
                }
                p {
                    class: "about-location",
                    "📍 {data.location}"
                }
            }

            // Bio Section
            section {
                class: "about-bio-section",
                h3 {
                    class: "about-section-title",
                    "About Me"
                }
                p {
                    class: "about-bio-text",
                    "{data.bio}"
                }
            }

            // Skills Section
            section {
                class: "about-section",
                h3 {
                    class: "about-section-title",
                    "Skills"
                }
                div {
                    class: "skills-grid",
                    for skill in data.skills {
                        span {
                            key: "{skill}",
                            class: "skill-tag",
                            "{skill}"
                        }
                    }
                }
            }

            // Experience Section
            section {
                class: "about-section",
                h3 {
                    class: "about-section-title",
                    "Experience"
                }
                for exp in data.experience {
                    div {
                        key: "{exp.company}-{exp.position}",
                        class: "experience-card",
                        div {
                            class: "experience-header",
                            div {
                                h4 {
                                    class: "experience-position",
                                    "{exp.position}"
                                }
                                p {
                                    class: "experience-company",
                                    "{exp.company}"
                                }
                            }
                            span {
                                class: "experience-duration",
                                "{exp.duration}"
                            }
                        }
                        p {
                            class: "experience-description",
                            "{exp.description}"
                        }
                    }
                }
            }

            // Interests Section
            section {
                class: "about-section",
                h3 {
                    class: "about-section-title",
                    "Interests"
                }
                div {
                    class: "interests-grid",
                    for interest in data.interests {
                        div {
                            key: "{interest}",
                            class: "interest-item",
                            span { class: "interest-bullet", "•" }
                            "{interest}"
                        }
                    }
                }
            }

            // Contact Section
            section {
                class: "contact-section",
                h3 {
                    class: "about-section-title",
                    "Contact"
                }
                div {
                    class: "contact-grid",

                    div {
                        class: "contact-item",
                        span { class: "contact-icon", "📧" }
                        a {
                            href: "mailto:{data.contact.email}",
                            class: "contact-link",
                            "{data.contact.email}"
                        }
                    }

                    div {
                        class: "contact-item",
                        span { class: "contact-icon", "🌐" }
                        a {
                            href: "{data.contact.website}",
                            target: "_blank",
                            class: "contact-link",
                            "Website"
                        }
                    }

                    div {
                        class: "contact-item",
                        span { class: "contact-icon", "⚡" }
                        a {
                            href: "https://github.com/{data.contact.github}",
                            target: "_blank",
                            class: "contact-link",
                            "GitHub"
                        }
                    }
                }
            }

            // Footer
            footer {
                class: "about-footer",
                p { class: "footer-updated", "Last updated: {data.updated}" }
                p {
                    class: "footer-note",
                    "Generated statically with Dioxus SSG 🦀"
                }
            }
        }
    }
}
