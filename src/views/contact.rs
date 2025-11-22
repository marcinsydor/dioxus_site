use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const CONTACT_CSS: Asset = asset!("/assets/styling/contact.css");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct FormData {
    name: String,
    email: String,
    subject: String,
    message: String,
    submitted_at: String,
}

#[derive(Debug, Clone, PartialEq)]
enum FormState {
    Editing,
    Submitted(FormData),
    Error(String),
}

#[component]
pub fn Contact() -> Element {
    // Form state management
    let mut form_state = use_signal(|| FormState::Editing);
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut subject = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());

    // Validation state
    let mut validation_errors = use_signal(|| Vec::<String>::new());

    // Calculate if form is valid
    let is_valid = use_memo(move || {
        !name().trim().is_empty()
            && !email().trim().is_empty()
            && !subject().trim().is_empty()
            && !message().trim().is_empty()
            && email().contains('@')
    });

    // Form submission handler
    let mut handle_submit = move |_| {
        let mut errors = Vec::new();

        // Validate form
        if name().trim().is_empty() {
            errors.push("Name is required".to_string());
        }
        if email().trim().is_empty() {
            errors.push("Email is required".to_string());
        } else if !email().contains('@') {
            errors.push("Please enter a valid email address".to_string());
        }
        if subject().trim().is_empty() {
            errors.push("Subject is required".to_string());
        }
        if message().trim().is_empty() {
            errors.push("Message is required".to_string());
        }

        if !errors.is_empty() {
            validation_errors.set(errors);
            form_state.set(FormState::Error("Please fix the errors below".to_string()));
            return;
        }

        // Simulate form processing
        let form_data = FormData {
            name: name().clone(),
            email: email().clone(),
            subject: subject().clone(),
            message: message().clone(),
            submitted_at: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        };

        // Save to localStorage (browser-only feature)
        #[cfg(feature = "web")]
        {
            if let Ok(json) = serde_json::to_string(&form_data) {
                let window = web_sys::window().unwrap();
                let storage = window.local_storage().unwrap().unwrap();
                let _ = storage.set_item("last_contact_submission", &json);
            }
        }

        validation_errors.set(Vec::new());
        form_state.set(FormState::Submitted(form_data));
    };

    let reset_form = move |_| {
        name.set(String::new());
        email.set(String::new());
        subject.set(String::new());
        message.set(String::new());
        validation_errors.set(Vec::new());
        form_state.set(FormState::Editing);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: CONTACT_CSS }

        div {
            class: "contact-container",

            header {
                class: "contact-header",
                h1 { class: "contact-title", "Contact Me" }
                p {
                    class: "contact-subtitle",
                    "Get in touch! This form demonstrates dynamic JavaScript/WASM functionality."
                }
            }

            div {
                class: "contact-content",

                // Contact Information
                div {
                    class: "contact-info",
                    h2 { "Contact Information" }
                    div {
                        class: "contact-methods",
                        div {
                            class: "contact-method",
                            span { class: "contact-icon", "📧" }
                            div {
                                h3 { "Email" }
                                a {
                                    href: "mailto:marcin.sydor@sky.uk",
                                    class: "contact-link",
                                    "marcin.sydor@sky.uk"
                                }
                            }
                        }
                        div {
                            class: "contact-method",
                            span { class: "contact-icon", "💼" }
                            div {
                                h3 { "LinkedIn" }
                                p { "Connect with me professionally" }
                            }
                        }
                        div {
                            class: "contact-method",
                            span { class: "contact-icon", "⚡" }
                            div {
                                h3 { "GitHub" }
                                a {
                                    href: "https://github.com/marcinsydor",
                                    target: "_blank",
                                    class: "contact-link",
                                    "@marcinsydor"
                                }
                            }
                        }
                    }
                }

                // Dynamic Form Section
                div {
                    class: "contact-form-section",
                    h2 { "Send a Message" }

                    div {
                        class: "js-functionality-notice",
                        p {
                            "🚀 This form demonstrates "
                            strong { "dynamic JavaScript/WASM functionality" }
                            " loaded in your browser!"
                        }

                        // Show WASM loading information
                        div {
                            class: "demo-info",
                            h4 { "📦 WASM Assets Loaded:" }
                            ul {
                                li { "✅ dioxus_site_bg.wasm - Rust code compiled to WebAssembly" }
                                li { "✅ dioxus_site.js - JavaScript glue code for WASM integration" }
                                li { "✅ Interactive form validation running in WASM" }
                                li { "✅ Real-time state management via Dioxus signals" }
                            }

                            p {
                                "Demo Status: "
                                match form_state() {
                                    FormState::Editing => rsx! {
                                        span { class: "status-editing", "WASM form ready for input" }
                                    },
                                    FormState::Submitted(_) => rsx! {
                                        span { class: "status-success", "Form processed by WASM!" }
                                    },
                                    FormState::Error(_) => rsx! {
                                        span { class: "status-error", "WASM validation active" }
                                    },
                                }
                            }
                            p {
                                "Form Valid (computed in WASM): "
                                if is_valid() {
                                    span { class: "status-valid", "✅ Yes" }
                                } else {
                                    span { class: "status-invalid", "❌ No" }
                                }
                            }
                        }
                    }

                    // Show submission result
                    match form_state() {
                        FormState::Submitted(data) => rsx! {
                            div {
                                class: "submission-result",
                                h3 { "✅ Form Submitted Successfully!" }
                                div {
                                    class: "submitted-data",
                                    h4 { "Submitted Data:" }
                                    div { class: "data-item",
                                        strong { "Name: " }
                                        span { "{data.name}" }
                                    }
                                    div { class: "data-item",
                                        strong { "Email: " }
                                        span { "{data.email}" }
                                    }
                                    div { class: "data-item",
                                        strong { "Subject: " }
                                        span { "{data.subject}" }
                                    }
                                    div { class: "data-item",
                                        strong { "Message: " }
                                        span { "{data.message}" }
                                    }
                                    div { class: "data-item",
                                        strong { "Submitted: " }
                                        span { "{data.submitted_at}" }
                                    }
                                }

                                div {
                                    class: "demo-features",
                                    h4 { "🎯 WASM Features Successfully Loaded:" }
                                    ul {
                                        li { "✅ Rust form validation compiled to WebAssembly (~800KB .wasm file)" }
                                        li { "✅ Real-time reactive state management in WASM" }
                                        li { "✅ Interactive form submission processed in WASM" }
                                        li { "✅ JSON serialization/deserialization in WASM" }
                                        li { "✅ Browser localStorage integration via WASM" }
                                        li { "✅ Conditional rendering powered by WASM" }
                                        li { "✅ Zero JavaScript - all logic runs in WebAssembly!" }
                                    }

                                    div {
                                        class: "wasm-info",
                                        p {
                                            "🦀 This page loaded a full Rust/WASM binary to handle the form, "
                                            "demonstrating how you can add interactive functionality to specific pages "
                                            "while keeping others as pure static HTML."
                                        }
                                    }
                                }

                                button {
                                    class: "btn btn-secondary",
                                    onclick: reset_form,
                                    "Send Another Message"
                                }
                            }
                        },
                        _ => rsx! {
                            // Contact Form
                            form {
                                class: "contact-form",
                                onsubmit: move |e| {
                                    e.prevent_default();
                                    handle_submit(());
                                },

                                // Show validation errors
                                if !validation_errors().is_empty() {
                                    div {
                                        class: "validation-errors",
                                        h4 { "Please fix the following errors:" }
                                        ul {
                                            for error in validation_errors() {
                                                li { "{error}" }
                                            }
                                        }
                                    }
                                }

                                div {
                                    class: "form-row",
                                    div {
                                        class: "form-group",
                                        label { "for": "name", "Name *" }
                                        input {
                                            "type": "text",
                                            id: "name",
                                            class: if name().trim().is_empty() && matches!(form_state(), FormState::Error(_)) {
                                                "form-input error"
                                            } else {
                                                "form-input"
                                            },
                                            placeholder: "Your full name",
                                            value: "{name}",
                                            oninput: move |e| name.set(e.value())
                                        }
                                    }
                                    div {
                                        class: "form-group",
                                        label { "for": "email", "Email *" }
                                        input {
                                            "type": "email",
                                            id: "email",
                                            class: if (email().trim().is_empty() || !email().contains('@')) && matches!(form_state(), FormState::Error(_)) {
                                                "form-input error"
                                            } else {
                                                "form-input"
                                            },
                                            placeholder: "your.email@example.com",
                                            value: "{email}",
                                            oninput: move |e| email.set(e.value())
                                        }
                                    }
                                }

                                div {
                                    class: "form-group",
                                    label { "for": "subject", "Subject *" }
                                    input {
                                        "type": "text",
                                        id: "subject",
                                        class: if subject().trim().is_empty() && matches!(form_state(), FormState::Error(_)) {
                                            "form-input error"
                                        } else {
                                            "form-input"
                                        },
                                        placeholder: "What's this about?",
                                        value: "{subject}",
                                        oninput: move |e| subject.set(e.value())
                                    }
                                }

                                div {
                                    class: "form-group",
                                    label { "for": "message", "Message *" }
                                    textarea {
                                        id: "message",
                                        class: if message().trim().is_empty() && matches!(form_state(), FormState::Error(_)) {
                                            "form-textarea error"
                                        } else {
                                            "form-textarea"
                                        },
                                        placeholder: "Tell me what's on your mind...",
                                        rows: "6",
                                        value: "{message}",
                                        oninput: move |e| message.set(e.value())
                                    }
                                }

                                div {
                                    class: "form-actions",
                                    button {
                                        "type": "submit",
                                        class: if is_valid() { "btn btn-primary" } else { "btn btn-primary disabled" },
                                        disabled: !is_valid(),
                                        "Send Message ✨"
                                    }
                                    button {
                                        "type": "button",
                                        class: "btn btn-secondary",
                                        onclick: reset_form,
                                        "Reset Form"
                                    }
                                }

                                div {
                                    class: "form-note",
                                    p {
                                        "* Required fields. This is a demo form - no actual email will be sent. "
                                        "Data is processed client-side and stored in browser localStorage."
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Tech details section
            div {
                class: "tech-details",
                h2 { "🔧 Technical Implementation" }
                div {
                    class: "tech-grid",
                    div {
                        class: "tech-item",
                        h3 { "🦀 WebAssembly Loaded" }
                        p { "This page loaded ~800KB of WASM containing compiled Rust code for form processing" }
                    }
                    div {
                        class: "tech-item",
                        h3 { "⚡ WASM State Management" }
                        p { "All form state, validation, and reactivity is handled inside the WebAssembly binary" }
                    }
                    div {
                        class: "tech-item",
                        h3 { "🏗️ Selective Enhancement" }
                        p { "Other pages are pure static HTML, this page loads WASM for interactivity" }
                    }
                    div {
                        class: "tech-item",
                        h3 { "📱 Asset Loading Demo" }
                        p { "Check browser DevTools Network tab to see the WASM and JS files loaded for this page" }
                    }
                }
            }
        }

        // Static fallback for non-JS environments
        noscript {
            div {
                class: "noscript-notice",
                h3 { "📄 Static Version" }
                p {
                    "JavaScript is disabled. You're viewing the static version of this page. "
                    "The form above won't be interactive, but you can still view the content and contact information."
                }
                p {
                    "To see the dynamic features (form validation, real-time updates, WASM functionality), "
                    "please enable JavaScript in your browser."
                }
            }
        }
    }
}

/// ContactFormOnly - renders just the form without the surrounding layout
/// This is used for hybrid pages where the layout is already in static HTML
#[component]
pub fn ContactFormOnly() -> Element {
    // Form state management
    let mut form_state = use_signal(|| FormState::Editing);
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut subject = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());

    // Validation state
    let mut validation_errors = use_signal(|| Vec::<String>::new());

    // Calculate if form is valid
    let is_valid = use_memo(move || {
        !name().trim().is_empty()
            && !email().trim().is_empty()
            && !subject().trim().is_empty()
            && !message().trim().is_empty()
            && email().contains('@')
    });

    // Form submission handler
    let mut handle_submit = move |_| {
        let mut errors = Vec::new();

        // Validate form
        if name().trim().is_empty() {
            errors.push("Name is required".to_string());
        }
        if email().trim().is_empty() {
            errors.push("Email is required".to_string());
        } else if !email().contains('@') {
            errors.push("Please enter a valid email address".to_string());
        }
        if subject().trim().is_empty() {
            errors.push("Subject is required".to_string());
        }
        if message().trim().is_empty() {
            errors.push("Message is required".to_string());
        }

        if !errors.is_empty() {
            validation_errors.set(errors);
            form_state.set(FormState::Error("Please fix the errors below".to_string()));
            return;
        }

        // Simulate form processing
        let form_data = FormData {
            name: name().clone(),
            email: email().clone(),
            subject: subject().clone(),
            message: message().clone(),
            submitted_at: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        };

        // Save to localStorage (browser-only feature)
        #[cfg(feature = "web")]
        {
            if let Ok(json) = serde_json::to_string(&form_data) {
                let window = web_sys::window().unwrap();
                let storage = window.local_storage().unwrap().unwrap();
                let _ = storage.set_item("last_contact_submission", &json);
            }
        }

        validation_errors.set(Vec::new());
        form_state.set(FormState::Submitted(form_data));
    };

    let reset_form = move |_| {
        name.set(String::new());
        email.set(String::new());
        subject.set(String::new());
        message.set(String::new());
        validation_errors.set(Vec::new());
        form_state.set(FormState::Editing);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: CONTACT_CSS }

        div {
            class: "js-functionality-notice",
            p {
                "🚀 This form demonstrates "
                strong { "dynamic JavaScript/WASM functionality" }
                " loaded in your browser!"
            }

            // Show WASM loading information
            div {
                class: "demo-info",
                h4 { "📦 WASM Assets Loaded:" }
                ul {
                    li { "✅ dioxus_site_bg.wasm - Rust code compiled to WebAssembly" }
                    li { "✅ dioxus_site.js - JavaScript glue code for WASM integration" }
                    li { "✅ Interactive form validation running in WASM" }
                    li { "✅ Real-time state management via Dioxus signals" }
                }

                p {
                    "Demo Status: "
                    match form_state() {
                        FormState::Editing => rsx! {
                            span { class: "status-editing", "WASM form ready for input" }
                        },
                        FormState::Submitted(_) => rsx! {
                            span { class: "status-success", "Form processed by WASM!" }
                        },
                        FormState::Error(_) => rsx! {
                            span { class: "status-error", "WASM validation active" }
                        },
                    }
                }
            }
            p {
                "🔄 Live Validation Status (computed in WASM): "
                if is_valid() {
                    span { class: "status-valid", "✅ Valid - Ready to submit!" }
                } else {
                    span { class: "status-invalid", "❌ Invalid - Please fill all fields" }
                }
            }
        }

        // Show submission result
        match form_state() {
            FormState::Submitted(data) => rsx! {
                div {
                    class: "submission-result",
                    h3 { "✅ Form Submitted Successfully!" }
                    div {
                        class: "submitted-data",
                        h4 { "Submitted Data:" }
                        div { class: "data-item",
                            strong { "Name: " }
                            span { "{data.name}" }
                        }
                        div { class: "data-item",
                            strong { "Email: " }
                            span { "{data.email}" }
                        }
                        div { class: "data-item",
                            strong { "Subject: " }
                            span { "{data.subject}" }
                        }
                        div { class: "data-item",
                            strong { "Message: " }
                            span { "{data.message}" }
                        }
                        div { class: "data-item",
                            strong { "Submitted: " }
                            span { "{data.submitted_at}" }
                        }
                    }

                    div {
                        class: "demo-features",
                        h4 { "🎯 WASM Features Successfully Loaded:" }
                        ul {
                            li { "✅ Rust form validation compiled to WebAssembly (~800KB .wasm file)" }
                            li { "✅ Real-time reactive state management in WASM" }
                            li { "✅ Interactive form submission processed in WASM" }
                            li { "✅ JSON serialization/deserialization in WASM" }
                            li { "✅ Browser localStorage integration via WASM" }
                            li { "✅ Conditional rendering powered by WASM" }
                            li { "✅ Zero JavaScript - all logic runs in WebAssembly!" }
                        }

                        div {
                            class: "wasm-info",
                            p {
                                "🦀 This page loaded a full Rust/WASM binary to handle the form, "
                                "demonstrating how you can add interactive functionality to specific pages "
                                "while keeping others as pure static HTML."
                            }
                        }
                    }

                    button {
                        class: "btn btn-secondary",
                        onclick: reset_form,
                        "Send Another Message"
                    }
                }
            },
            _ => rsx! {
                // Contact Form
                form {
                    class: "contact-form",
                    onsubmit: move |e| {
                        e.prevent_default();
                        handle_submit(());
                    },

                    // Show validation errors
                    if !validation_errors().is_empty() {
                        div {
                            class: "validation-errors",
                            h4 { "Please fix the following errors:" }
                            ul {
                                for error in validation_errors() {
                                    li { "{error}" }
                                }
                            }
                        }
                    }

                    div {
                        class: "form-row",
                        div {
                            class: "form-group",
                            label { "for": "name", "Name *" }
                            input {
                                r#type: "text",
                                id: "name",
                                class: "form-input",
                                placeholder: "Your full name",
                                value: "{name}",
                                oninput: move |e| name.set(e.value()),
                            }
                        }
                        div {
                            class: "form-group",
                            label { "for": "email", "Email *" }
                            input {
                                r#type: "email",
                                id: "email",
                                class: "form-input",
                                placeholder: "your.email@example.com",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                            }
                        }
                    }

                    div {
                        class: "form-group",
                        label { "for": "subject", "Subject *" }
                        input {
                            r#type: "text",
                            id: "subject",
                            class: "form-input",
                            placeholder: "What's this about?",
                            value: "{subject}",
                            oninput: move |e| subject.set(e.value()),
                        }
                    }

                    div {
                        class: "form-group",
                        label { "for": "message", "Message *" }
                        textarea {
                            id: "message",
                            class: "form-textarea",
                            placeholder: "Tell me what's on your mind...",
                            rows: "6",
                            value: "{message}",
                            oninput: move |e| message.set(e.value()),
                        }
                    }

                    div {
                        class: "form-actions",
                        button {
                            r#type: "submit",
                            class: "btn btn-primary",
                            disabled: !is_valid(),
                            "Send Message ✨"
                        }
                        button {
                            r#type: "button",
                            class: "btn btn-secondary",
                            onclick: reset_form,
                            "Reset Form"
                        }
                    }
                }
            }
        }
    }
}
