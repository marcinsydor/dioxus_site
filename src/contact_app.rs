//! Simple DOM-based Contact Form
//! This creates an interactive contact form using web APIs instead of full Dioxus mounting

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{
    console, window, Document, Element, Event, HtmlElement, HtmlInputElement, HtmlTextAreaElement,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FormData {
    name: String,
    email: String,
    subject: String,
    message: String,
    submitted_at: String,
}

#[derive(Debug, Clone)]
enum FormState {
    Editing,
    Submitted(FormData),
    Error(String),
}

/// Initialize the Contact form and replace the placeholder with interactive elements
pub fn mount_contact_form() -> Result<(), JsValue> {
    let window = window().ok_or("No global window exists")?;
    let document = window.document().ok_or("Should have a document")?;

    // Find the placeholder and replace it with the interactive form
    if let Some(placeholder) = document.get_element_by_id("contact-form-placeholder") {
        create_interactive_form(&document, &placeholder)?;
        console::log_1(&"✅ Interactive contact form created".into());
    }

    Ok(())
}

fn create_interactive_form(document: &Document, placeholder: &Element) -> Result<(), JsValue> {
    let form_html = r#"
    <form id="contact-form" class="contact-form">
        <div class="form-row">
            <div class="form-group">
                <label for="contact-name">Name *</label>
                <input type="text" id="contact-name" class="form-input" placeholder="Your full name" required />
                <div class="error-message" id="name-error"></div>
            </div>
            <div class="form-group">
                <label for="contact-email">Email *</label>
                <input type="email" id="contact-email" class="form-input" placeholder="your.email@example.com" required />
                <div class="error-message" id="email-error"></div>
            </div>
        </div>

        <div class="form-group">
            <label for="contact-subject">Subject *</label>
            <input type="text" id="contact-subject" class="form-input" placeholder="What's this about?" required />
            <div class="error-message" id="subject-error"></div>
        </div>

        <div class="form-group">
            <label for="contact-message">Message *</label>
            <textarea id="contact-message" class="form-textarea" placeholder="Tell me what's on your mind..." rows="6" required></textarea>
            <div class="error-message" id="message-error"></div>
        </div>

        <div class="form-actions">
            <button type="submit" id="submit-btn" class="btn btn-primary">Send Message ✨</button>
            <button type="button" id="reset-btn" class="btn btn-secondary">Reset Form</button>
        </div>

        <div class="form-status" id="form-status"></div>

        <div class="form-note">
            <p>🦀 <strong>Powered by WebAssembly:</strong> This form is running Rust code compiled to WASM!</p>
        </div>
    </form>

    <style>
        .error-message {
            color: #dc2626;
            font-size: 0.875rem;
            margin-top: 0.25rem;
            min-height: 1.25rem;
        }
        .form-status {
            margin-top: 1rem;
            padding: 1rem;
            border-radius: 0.5rem;
            text-align: center;
            font-weight: 500;
        }
        .form-status.success {
            background-color: #dcfce7;
            color: #166534;
            border: 1px solid #bbf7d0;
        }
        .form-status.error {
            background-color: #fef2f2;
            color: #dc2626;
            border: 1px solid #fecaca;
        }
        .form-input:focus, .form-textarea:focus {
            outline: 2px solid #3b82f6;
            outline-offset: 2px;
        }
        .form-input.error, .form-textarea.error {
            border-color: #dc2626;
        }
        .btn:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
    </style>
    "#;

    placeholder.set_inner_html(form_html);

    // Set up event listeners
    setup_form_listeners(document)?;

    Ok(())
}

fn setup_form_listeners(document: &Document) -> Result<(), JsValue> {
    let form = document
        .get_element_by_id("contact-form")
        .ok_or("Contact form not found")?;

    // Submit handler
    let submit_handler = Closure::wrap(Box::new(move |e: Event| {
        e.prevent_default();
        if let Err(err) = handle_form_submit() {
            console::error_1(&format!("Form submission error: {:?}", err).into());
        }
    }) as Box<dyn FnMut(_)>);

    form.add_event_listener_with_callback("submit", submit_handler.as_ref().unchecked_ref())?;
    submit_handler.forget();

    // Reset handler
    let reset_btn = document
        .get_element_by_id("reset-btn")
        .ok_or("Reset button not found")?;

    let reset_handler = Closure::wrap(Box::new(move |_e: Event| {
        if let Err(err) = handle_form_reset() {
            console::error_1(&format!("Form reset error: {:?}", err).into());
        }
    }) as Box<dyn FnMut(_)>);

    reset_btn.add_event_listener_with_callback("click", reset_handler.as_ref().unchecked_ref())?;
    reset_handler.forget();

    // Real-time validation
    setup_validation_listeners(document)?;

    Ok(())
}

fn setup_validation_listeners(document: &Document) -> Result<(), JsValue> {
    let inputs = [
        "contact-name",
        "contact-email",
        "contact-subject",
        "contact-message",
    ];

    for input_id in &inputs {
        if let Some(input) = document.get_element_by_id(input_id) {
            let input_id_owned = input_id.to_string();
            let validation_handler = Closure::wrap(Box::new(move |_e: Event| {
                if let Err(err) = validate_field(&input_id_owned) {
                    console::error_1(&format!("Validation error: {:?}", err).into());
                }
            }) as Box<dyn FnMut(_)>);

            input.add_event_listener_with_callback(
                "blur",
                validation_handler.as_ref().unchecked_ref(),
            )?;
            validation_handler.forget();
        }
    }

    Ok(())
}

fn validate_field(field_id: &str) -> Result<bool, JsValue> {
    let window = window().ok_or("No global window exists")?;
    let document = window.document().ok_or("Should have a document")?;

    let input = document
        .get_element_by_id(field_id)
        .ok_or(format!("Input {} not found", field_id))?;

    let value = if field_id == "contact-message" {
        let textarea: HtmlTextAreaElement = input.clone().dyn_into()?;
        textarea.value()
    } else {
        let html_input: HtmlInputElement = input.clone().dyn_into()?;
        html_input.value()
    };

    let error_id = field_id.replace("contact-", "") + "-error";
    let error_elem = document.get_element_by_id(&error_id);

    let is_valid = match field_id {
        "contact-name" => !value.trim().is_empty(),
        "contact-email" => !value.trim().is_empty() && value.contains('@'),
        "contact-subject" => !value.trim().is_empty(),
        "contact-message" => !value.trim().is_empty(),
        _ => true,
    };

    // Update UI
    if is_valid {
        if let Ok(html_elem) = input.clone().dyn_into::<HtmlElement>() {
            html_elem.class_list().remove_1("error")?;
        }
        if let Some(error_elem) = error_elem {
            error_elem.set_inner_html("");
        }
    } else {
        if let Ok(html_elem) = input.clone().dyn_into::<HtmlElement>() {
            html_elem.class_list().add_1("error")?;
        }
        if let Some(error_elem) = error_elem {
            let error_msg = match field_id {
                "contact-name" => "Name is required",
                "contact-email" => "Valid email is required",
                "contact-subject" => "Subject is required",
                "contact-message" => "Message is required",
                _ => "This field is required",
            };
            error_elem.set_inner_html(error_msg);
        }
    }

    Ok(is_valid)
}

fn handle_form_submit() -> Result<(), JsValue> {
    let window = window().ok_or("No global window exists")?;
    let document = window.document().ok_or("Should have a document")?;

    // Get form values
    let name = get_input_value(&document, "contact-name")?;
    let email = get_input_value(&document, "contact-email")?;
    let subject = get_input_value(&document, "contact-subject")?;
    let message = get_textarea_value(&document, "contact-message")?;

    // Validate all fields
    let name_valid = validate_field("contact-name")?;
    let email_valid = validate_field("contact-email")?;
    let subject_valid = validate_field("contact-subject")?;
    let message_valid = validate_field("contact-message")?;

    if !name_valid || !email_valid || !subject_valid || !message_valid {
        show_status(&document, "Please fix the errors above", "error")?;
        return Ok(());
    }

    // Disable submit button
    if let Some(submit_btn) = document.get_element_by_id("submit-btn") {
        submit_btn.set_inner_html("Sending...");
        if let Ok(btn) = submit_btn.dyn_into::<HtmlInputElement>() {
            btn.set_disabled(true);
        }
    }

    // Create form data
    let form_data = FormData {
        name,
        email,
        subject,
        message,
        submitted_at: js_sys::Date::new_0()
            .to_iso_string()
            .as_string()
            .unwrap_or_default(),
    };

    // Simulate form submission (in a real app, you'd send this to a server)
    console::log_1(&format!("📧 Form submitted: {:?}", form_data).into());

    // Show success message
    show_status(
        &document,
        &format!(
            "✅ Thank you, {}! Your message has been received. (This is a demo)",
            form_data.name
        ),
        "success",
    )?;

    // Re-enable submit button
    if let Some(submit_btn) = document.get_element_by_id("submit-btn") {
        submit_btn.set_inner_html("Send Message ✨");
        if let Ok(btn) = submit_btn.dyn_into::<HtmlInputElement>() {
            btn.set_disabled(false);
        }
    }

    Ok(())
}

fn handle_form_reset() -> Result<(), JsValue> {
    let window = window().ok_or("No global window exists")?;
    let document = window.document().ok_or("Should have a document")?;

    // Clear all form fields
    let inputs = ["contact-name", "contact-email", "contact-subject"];
    for input_id in &inputs {
        if let Some(input) = document.get_element_by_id(input_id) {
            if let Ok(html_input) = input.clone().dyn_into::<HtmlInputElement>() {
                html_input.set_value("");
            }
            if let Ok(html_elem) = input.clone().dyn_into::<HtmlElement>() {
                html_elem.class_list().remove_1("error").ok();
            }
        }
    }

    // Clear textarea
    if let Some(textarea) = document.get_element_by_id("contact-message") {
        if let Ok(html_textarea) = textarea.clone().dyn_into::<HtmlTextAreaElement>() {
            html_textarea.set_value("");
        }
        if let Ok(html_elem) = textarea.clone().dyn_into::<HtmlElement>() {
            html_elem.class_list().remove_1("error").ok();
        }
    }

    // Clear error messages
    let error_ids = [
        "name-error",
        "email-error",
        "subject-error",
        "message-error",
    ];
    for error_id in &error_ids {
        if let Some(error_elem) = document.get_element_by_id(error_id) {
            error_elem.set_inner_html("");
        }
    }

    // Clear status
    if let Some(status) = document.get_element_by_id("form-status") {
        status.set_inner_html("");
        if let Ok(html_elem) = status.clone().dyn_into::<HtmlElement>() {
            html_elem.class_list().remove_1("success").ok();
            html_elem.class_list().remove_1("error").ok();
        }
    }

    console::log_1(&"🔄 Form reset".into());
    Ok(())
}

fn get_input_value(document: &Document, id: &str) -> Result<String, JsValue> {
    let input = document
        .get_element_by_id(id)
        .ok_or(format!("Input {} not found", id))?;
    let html_input: HtmlInputElement = input.dyn_into()?;
    Ok(html_input.value())
}

fn get_textarea_value(document: &Document, id: &str) -> Result<String, JsValue> {
    let textarea = document
        .get_element_by_id(id)
        .ok_or(format!("Textarea {} not found", id))?;
    let html_textarea: HtmlTextAreaElement = textarea.dyn_into()?;
    Ok(html_textarea.value())
}

fn show_status(document: &Document, message: &str, status_type: &str) -> Result<(), JsValue> {
    if let Some(status_elem) = document.get_element_by_id("form-status") {
        status_elem.set_inner_html(message);
        if let Ok(html_elem) = status_elem.clone().dyn_into::<HtmlElement>() {
            html_elem.class_list().remove_1("success")?;
            html_elem.class_list().remove_1("error")?;
            html_elem.class_list().add_1(status_type)?;
        }
    }
    Ok(())
}

// Export the mount function for JavaScript to call
#[wasm_bindgen]
pub fn start_contact_app() {
    // Set up console error panic hook for better debugging
    console_error_panic_hook::set_once();

    // Initialize the contact form
    if let Err(err) = mount_contact_form() {
        console::error_1(&format!("❌ Failed to mount contact form: {:?}", err).into());
    } else {
        console::log_1(&"✅ Contact form initialized successfully".into());
    }
}
