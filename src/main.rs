use dioxus::prelude::*;
use dioxus_site::{
    views::{Contact, ContactFormOnly},
    App,
};
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

fn main() {
    // Launch the full app (for regular SPA mode)
    dioxus::launch(App);
}

// Standalone Contact component wrapper for WASM mounting
#[component]
fn ContactApp() -> Element {
    rsx! {
        div {
            class: "contact-form-container",
            style: "width: 100%;",
            ContactFormOnly {}
        }
    }
}

// Export function to mount Contact component to a specific DOM element
#[wasm_bindgen]
pub fn mount_contact_component() {
    console_error_panic_hook::set_once();

    console::log_1(&"🚀 Starting Dioxus Contact component mount...".into());

    let window = window().expect("should have a window");
    let document = window.document().expect("should have a document");

    // Find the contact form placeholder
    if let Some(placeholder) = document.get_element_by_id("contact-form-placeholder") {
        console::log_1(&"✅ Found contact form placeholder".into());

        // Clear the placeholder and create a mount point
        placeholder.set_inner_html(r#"<div id="dioxus-contact-root"></div>"#);

        // Get the mount point
        if let Some(_root_element) = document.get_element_by_id("dioxus-contact-root") {
            console::log_1(&"✅ Created Dioxus mount point".into());

            // Use Dioxus web-specific mounting
            let config = dioxus_web::Config::new().rootname("dioxus-contact-root");

            dioxus_web::launch::launch_cfg(ContactApp, config);

            console::log_1(&"✅ Dioxus Contact component mounted successfully".into());
        } else {
            console::error_1(&"❌ Failed to create Dioxus mount point".into());
        }
    } else {
        console::error_1(&"❌ Contact form placeholder not found".into());
    }
}

// Alternative approach using direct element mounting
#[wasm_bindgen]
pub fn mount_contact_to_element(element_id: &str) {
    console_error_panic_hook::set_once();

    console::log_1(&format!("🚀 Mounting Dioxus Contact to element: {}", element_id).into());

    let config = dioxus_web::Config::new().rootname(element_id);

    dioxus_web::launch::launch_cfg(ContactApp, config);

    console::log_1(&"✅ Contact component mounted to specified element".into());
}

// Utility function for initializing WASM
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
    console::log_1(&"🦀 Dioxus WASM module initialized".into());
}
