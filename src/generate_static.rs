//! Static site generator for Dioxus site
//! This binary generates static HTML files for all routes

use dioxus::prelude::*;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Starting static site generation...");

    let args: Vec<String> = env::args().collect();
    let skip_contact = args.contains(&"--skip-contact".to_string());

    let output_dir = Path::new("static_output");

    // Clean and create output directory
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;

    // Generate all pages
    generate_home_page(output_dir)?;
    generate_about_page(output_dir)?;

    if !skip_contact {
        generate_contact_page(output_dir)?;
    } else {
        println!("⏭️  Skipping contact page generation");
    }

    generate_blog_pages(output_dir)?;

    // Copy assets
    copy_assets(output_dir)?;

    println!("✅ Static site generation complete!");
    println!("📂 Files generated in: {}", output_dir.display());

    Ok(())
}

fn generate_home_page(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔨 Generating: /");

    let content = r#"<div id="navbar">
        <a href="/">Home</a>
        <a href="/about">About</a>
        <a href="/contact">Contact</a>
        <a href="/blog/1">Blog</a>
    </div>
    <div class="container">
        <h1>Welcome to Dioxus Site</h1>
        <p>This is the home page of my Dioxus-powered website.</p>
        <nav>
            <ul>
                <li><a href="/about">Learn about me</a></li>
                <li><a href="/blog/1">Read my blog</a></li>
            </ul>
        </nav>
    </div>"#;

    let html = create_html_document(
        "Home - Dioxus Site",
        "Welcome to my Dioxus-powered website",
        content,
        None,
    );

    fs::write(output_dir.join("index.html"), html)?;
    println!("✅ Generated: index.html");
    Ok(())
}

fn generate_about_page(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔨 Generating: /about");

    // Read the about data
    let about_data = include_str!("../assets/data/about.json");
    let data: serde_json::Value = serde_json::from_str(about_data)?;

    let content = format!(
        r#"<div id="navbar">
        <a href="/">Home</a>
        <a href="/about">About</a>
        <a href="/contact">Contact</a>
        <a href="/blog/1">Blog</a>
    </div>
    <div class="about-container">
        <header class="about-header">
            <h1 class="about-name">{name}</h1>
            <h2 class="about-title">{title}</h2>
            <p class="about-location">📍 {location}</p>
        </header>

        <section class="about-bio-section">
            <h3 class="about-section-title">About Me</h3>
            <p class="about-bio-text">{bio}</p>
        </section>

        <section class="about-section">
            <h3 class="about-section-title">Skills</h3>
            <div class="skills-grid">
                {skills}
            </div>
        </section>

        <section class="about-section">
            <h3 class="about-section-title">Experience</h3>
            {experience}
        </section>

        <section class="about-section">
            <h3 class="about-section-title">Interests</h3>
            <div class="interests-grid">
                {interests}
            </div>
        </section>

        <section class="contact-section">
            <h3 class="about-section-title">Contact</h3>
            <div class="contact-grid">
                <div class="contact-item">
                    <span class="contact-icon">📧</span>
                    <a href="mailto:{email}" class="contact-link">{email}</a>
                </div>
                <div class="contact-item">
                    <span class="contact-icon">🌐</span>
                    <a href="{website}" target="_blank" class="contact-link">Website</a>
                </div>
                <div class="contact-item">
                    <span class="contact-icon">⚡</span>
                    <a href="https://github.com/{github}" target="_blank" class="contact-link">GitHub</a>
                </div>
            </div>
        </section>

        <footer class="about-footer">
            <p class="footer-updated">Last updated: {updated}</p>
            <p class="footer-note">Generated statically with Dioxus SSG 🦀</p>
        </footer>
    </div>"#,
        name = data["name"].as_str().unwrap_or(""),
        title = data["title"].as_str().unwrap_or(""),
        location = data["location"].as_str().unwrap_or(""),
        bio = data["bio"].as_str().unwrap_or(""),
        skills = data["skills"]
            .as_array()
            .map(|skills| skills
                .iter()
                .map(|skill| format!(
                    r#"<span class="skill-tag">{}</span>"#,
                    skill.as_str().unwrap_or("")
                ))
                .collect::<Vec<_>>()
                .join(""))
            .unwrap_or_default(),
        experience = data["experience"]
            .as_array()
            .map(|exp| exp
                .iter()
                .map(|e| format!(
                    r#"<div class="experience-card">
                    <div class="experience-header">
                        <div>
                            <h4 class="experience-position">{}</h4>
                            <p class="experience-company">{}</p>
                        </div>
                        <span class="experience-duration">{}</span>
                    </div>
                    <p class="experience-description">{}</p>
                </div>"#,
                    e["position"].as_str().unwrap_or(""),
                    e["company"].as_str().unwrap_or(""),
                    e["duration"].as_str().unwrap_or(""),
                    e["description"].as_str().unwrap_or("")
                ))
                .collect::<Vec<_>>()
                .join(""))
            .unwrap_or_default(),
        interests = data["interests"]
            .as_array()
            .map(|interests| interests
                .iter()
                .map(|interest| format!(
                    r#"<div class="interest-item">
                    <span class="interest-bullet">•</span>
                    {}</div>"#,
                    interest.as_str().unwrap_or("")
                ))
                .collect::<Vec<_>>()
                .join(""))
            .unwrap_or_default(),
        email = data["contact"]["email"].as_str().unwrap_or(""),
        website = data["contact"]["website"].as_str().unwrap_or(""),
        github = data["contact"]["github"].as_str().unwrap_or(""),
        updated = data["updated"].as_str().unwrap_or(""),
    );

    let html = create_html_document(
        "About - Dioxus Site",
        "Learn more about me and my work",
        &content,
        None,
    );

    let about_dir = output_dir.join("about");
    fs::create_dir_all(&about_dir)?;
    fs::write(about_dir.join("index.html"), html)?;
    println!("✅ Generated: about/index.html");
    Ok(())
}

fn generate_contact_page(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔨 Generating: /contact (static version)");
    generate_static_contact_page(output_dir)?;
    Ok(())
}

fn generate_static_contact_page(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"<div id="navbar">
        <a href="/">Home</a>
        <a href="/about">About</a>
        <a href="/contact">Contact</a>
        <a href="/blog/1">Blog</a>
    </div>
    <div class="contact-container">
        <header class="contact-header">
            <h1 class="contact-title">Contact Me</h1>
            <p class="contact-subtitle">Get in touch! This page will demonstrate dynamic JavaScript/WASM functionality.</p>
        </header>

        <div class="contact-content">
            <div class="contact-info">
                <h2>Contact Information</h2>
                <div class="contact-methods">
                    <div class="contact-method">
                        <span class="contact-icon">📧</span>
                        <div>
                            <h3>Email</h3>
                            <a href="mailto:marcin.sydor@sky.uk" class="contact-link">marcin.sydor@sky.uk</a>
                        </div>
                    </div>
                    <div class="contact-method">
                        <span class="contact-icon">💼</span>
                        <div>
                            <h3>LinkedIn</h3>
                            <p>Connect with me professionally</p>
                        </div>
                    </div>
                    <div class="contact-method">
                        <span class="contact-icon">⚡</span>
                        <div>
                            <h3>GitHub</h3>
                            <a href="https://github.com/marcinsydor" target="_blank" class="contact-link">@marcinsydor</a>
                        </div>
                    </div>
                </div>
            </div>

            <div class="contact-form-section">
                <h2>Send a Message</h2>

                <div class="js-functionality-notice">
                    <p>🚀 <strong>Dynamic Form Demo:</strong> This form will demonstrate JavaScript/WASM functionality when enhanced with dynamic features.</p>
                </div>

                <div class="static-form-notice">
                    <h3>📄 Static Version</h3>
                    <p>You're viewing the static HTML version. The form below is for display purposes.</p>
                    <p>When JavaScript is enabled, this becomes a fully interactive form with:</p>
                    <ul>
                        <li>Real-time validation</li>
                        <li>Dynamic state management</li>
                        <li>Client-side form processing</li>
                        <li>WASM-powered functionality</li>
                    </ul>
                </div>

                <form class="contact-form">
                    <div class="form-row">
                        <div class="form-group">
                            <label for="name">Name *</label>
                            <input type="text" id="name" class="form-input" placeholder="Your full name" />
                        </div>
                        <div class="form-group">
                            <label for="email">Email *</label>
                            <input type="email" id="email" class="form-input" placeholder="your.email@example.com" />
                        </div>
                    </div>

                    <div class="form-group">
                        <label for="subject">Subject *</label>
                        <input type="text" id="subject" class="form-input" placeholder="What's this about?" />
                    </div>

                    <div class="form-group">
                        <label for="message">Message *</label>
                        <textarea id="message" class="form-textarea" placeholder="Tell me what's on your mind..." rows="6"></textarea>
                    </div>

                    <div class="form-actions">
                        <button type="button" class="btn btn-primary disabled">Send Message ✨ (Demo)</button>
                        <button type="button" class="btn btn-secondary">Reset Form</button>
                    </div>

                    <div class="form-note">
                        <p>* This is a static demo form. Enable JavaScript to see dynamic functionality.</p>
                    </div>
                </form>
            </div>
        </div>

        <div class="tech-details">
            <h2>🔧 Technical Implementation</h2>
            <div class="tech-grid">
                <div class="tech-item">
                    <h3>🦀 WebAssembly</h3>
                    <p>Dynamic version uses Rust compiled to WASM for all interactive functionality</p>
                </div>
                <div class="tech-item">
                    <h3>⚡ Reactive State</h3>
                    <p>Real-time form validation and state management using Dioxus signals</p>
                </div>
                <div class="tech-item">
                    <h3>🏗️ Hybrid Architecture</h3>
                    <p>Static HTML foundation with dynamic JavaScript/WASM enhancement</p>
                </div>
                <div class="tech-item">
                    <h3>📱 Progressive Enhancement</h3>
                    <p>Works without JS, enhanced with dynamic features when available</p>
                </div>
            </div>
        </div>
    </div>"#;

    let html = create_html_document(
        "Contact - Dioxus Site",
        "Get in touch with me through this contact form",
        content,
        None,
    );

    let contact_dir = output_dir.join("contact");
    fs::create_dir_all(&contact_dir)?;
    fs::write(contact_dir.join("index.html"), html)?;
    println!("✅ Generated: contact/index.html");
    Ok(())
}

fn generate_blog_pages(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let blog_ids = vec![1, 2, 3];

    for id in blog_ids {
        println!("🔨 Generating: /blog/{}", id);

        let content = format!(
            r#"<div id="navbar">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/contact">Contact</a>
            <a href="/blog/1">Blog</a>
        </div>
        <div class="container">
            <h1>Blog Post {id}</h1>
            <div class="blog-content">
                <p>This is blog post number {id}.</p>
                <p>In a real application, this content would be loaded from a database or markdown files.</p>

                <h2>Sample Content</h2>
                <p>Here's some sample content for blog post {id}. This demonstrates how static site generation works with Dioxus.</p>

                <h3>Benefits of SSG</h3>
                <ul>
                    <li>Fast loading times</li>
                    <li>Great SEO</li>
                    <li>Works without JavaScript</li>
                    <li>Easy to deploy</li>
                </ul>

                <nav class="blog-nav">
                    <a href="/">← Back to Home</a>
                    {prev_next}
                </nav>
            </div>
        </div>"#,
            id = id,
            prev_next = if id > 1 && id < 3 {
                format!(
                    r#"<a href="/blog/{}">← Previous</a> <a href="/blog/{}">Next →</a>"#,
                    id - 1,
                    id + 1
                )
            } else if id > 1 {
                format!(r#"<a href="/blog/{}">← Previous</a>"#, id - 1)
            } else if id < 3 {
                format!(r#"<a href="/blog/{}">Next →</a>"#, id + 1)
            } else {
                String::new()
            }
        );

        let html = create_html_document(
            &format!("Blog Post {} - Dioxus Site", id),
            &format!("Blog post number {}", id),
            &content,
            None,
        );

        let blog_dir = output_dir.join("blog").join(id.to_string());
        fs::create_dir_all(&blog_dir)?;
        fs::write(blog_dir.join("index.html"), html)?;
        println!("✅ Generated: blog/{}/index.html", id);
    }

    Ok(())
}

fn create_html_document(
    title: &str,
    description: &str,
    body_content: &str,
    js_path: Option<&str>,
) -> String {
    let js_preload = if let Some(js) = js_path {
        format!(
            r#"<link rel="preload" as="script" href="{}" crossorigin>"#,
            js
        )
    } else {
        String::new()
    };

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>{title}</title>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta charset="UTF-8">
    <meta name="description" content="{description}">

    <!-- Styles -->
    <link rel="stylesheet" href="/assets/styling/main.css">
    <link rel="stylesheet" href="/assets/styling/navbar.css">
    <link rel="stylesheet" href="/assets/styling/about.css">
    <link rel="stylesheet" href="/assets/styling/contact.css">
    <link rel="stylesheet" href="/assets/styling/blog.css">
    <link rel="stylesheet" href="/assets/styling/echo.css">
    <link rel="stylesheet" href="/assets/tailwind.css">

    <!-- Favicon -->
    <link rel="icon" href="/assets/favicon.ico">

    <!-- Additional meta tags for SEO -->
    <meta property="og:title" content="{title}">
    <meta property="og:description" content="{description}">
    <meta property="og:type" content="website">
    <meta name="twitter:card" content="summary">
    <meta name="twitter:title" content="{title}">
    <meta name="twitter:description" content="{description}">
    {js_preload}
</head>
<body>
    <div id="main">{body_content}</div>

    <!-- Static site notice -->
    <noscript>
        <div style="position: fixed; bottom: 1rem; right: 1rem; padding: 0.5rem 1rem; background: #f0f9ff; border: 1px solid #0ea5e9; border-radius: 0.5rem; font-size: 0.875rem; max-width: 300px; z-index: 1000;">
            <p style="margin: 0; font-weight: bold; color: #0369a1;">📄 Static HTML</p>
            <p style="margin: 0.25rem 0 0 0; color: #0369a1;">This page works without JavaScript!</p>
        </div>
    </noscript>

    <style>
    /* Basic styling for static pages */
    body {{
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        margin: 0;
        padding: 0;
        line-height: 1.6;
        color: #333;
    }}

    .container {{
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem;
    }}

    #navbar {{
        background: #f8fafc;
        padding: 1rem;
        border-bottom: 1px solid #e2e8f0;
        margin-bottom: 2rem;
    }}

    #navbar a {{
        margin-right: 1rem;
        text-decoration: none;
        color: #2563eb;
        font-weight: 500;
        padding: 0.5rem 1rem;
        border-radius: 0.375rem;
        transition: background-color 0.2s;
    }}

    #navbar a:hover {{
        background: #dbeafe;
    }}

    .blog-nav {{
        margin-top: 2rem;
        padding-top: 2rem;
        border-top: 1px solid #e2e8f0;
    }}

    .blog-nav a {{
        margin-right: 1rem;
        text-decoration: none;
        color: #2563eb;
    }}

    .blog-nav a:hover {{
        text-decoration: underline;
    }}

    h1 {{
        color: #1f2937;
        margin-bottom: 1rem;
    }}

    h2 {{
        color: #374151;
        margin-top: 2rem;
        margin-bottom: 1rem;
    }}

    h3 {{
        color: #4b5563;
        margin-top: 1.5rem;
        margin-bottom: 0.5rem;
    }}

    ul {{
        padding-left: 1.5rem;
    }}

    li {{
        margin-bottom: 0.25rem;
    }}
    </style>
</body>
</html>"#,
        title = title,
        js_preload = js_preload,
        description = description,
        body_content = body_content
    )
}

fn copy_assets(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Copying assets...");

    let assets_src = Path::new("assets");
    let assets_dest = output_dir.join("assets");

    if assets_src.exists() {
        copy_dir_recursive(assets_src, &assets_dest)?;
    }

    println!("✅ Assets copied");
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

// Helper function to create HTML document with additional CSS
fn create_html_document_with_css(
    title: &str,
    description: &str,
    content: &str,
    js_path: Option<&str>,
    additional_css: Option<&str>,
) -> String {
    let js_import = js_path
        .map(|path| format!(r#"<script type="module" src="{}"></script>"#, path))
        .unwrap_or_default();

    let extra_css = additional_css.unwrap_or("");

    // Include base CSS styles
    let base_css = r#"
body {
    background-color: #0f1116;
    color: #ffffff;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    margin: 20px;
}

#hero {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

#links {
    width: 400px;
    text-align: left;
    font-size: x-large;
    color: white;
    display: flex;
    flex-direction: column;
}

#links a {
    color: white;
    text-decoration: none;
    margin-top: 20px;
    margin: 10px 0px;
    border: white 1px solid;
    border-radius: 5px;
    padding: 10px;
}

#links a:hover {
    background-color: #1f1f1f;
    cursor: pointer;
}

#header {
    max-width: 1200px;
}

/* Additional contact page styles */
.contact-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.contact-header {
    text-align: center;
    margin-bottom: 3rem;
}

.contact-title {
    font-size: 3rem;
    font-weight: bold;
    margin-bottom: 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}

.contact-subtitle {
    font-size: 1.25rem;
    color: #9ca3af;
    max-width: 600px;
    margin: 0 auto;
}

.contact-content {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 3rem;
    margin-bottom: 3rem;
}

@media (max-width: 768px) {
    .contact-content {
        grid-template-columns: 1fr;
        gap: 2rem;
    }
}

.contact-info h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: #f9fafb;
}

.contact-methods {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.contact-method {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    padding: 1.5rem;
    background: #1f2937;
    border-radius: 0.75rem;
    border: 1px solid #374151;
}

.contact-icon {
    font-size: 1.5rem;
    padding: 0.75rem;
    background: #3b82f6;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
}

.contact-method h3 {
    font-size: 1.125rem;
    font-weight: 600;
    margin: 0 0 0.25rem 0;
    color: #f9fafb;
}

.contact-method p {
    margin: 0;
    color: #9ca3af;
    font-size: 0.875rem;
}

.contact-link {
    color: #60a5fa !important;
    text-decoration: none;
}

.contact-link:hover {
    color: #93c5fd !important;
    text-decoration: underline;
}

.contact-form-section h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: #f9fafb;
}

.tech-details {
    padding: 2rem;
    background: #1f2937;
    border-radius: 1rem;
    border: 1px solid #374151;
}

.tech-details h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: #f9fafb;
    text-align: center;
}

.tech-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
}

@media (max-width: 768px) {
    .tech-grid {
        grid-template-columns: 1fr;
    }
}

.tech-item {
    padding: 1.5rem;
    background: #111827;
    border-radius: 0.75rem;
    border: 1px solid #374151;
}

.tech-item h3 {
    font-size: 1.125rem;
    font-weight: 600;
    margin: 0 0 0.75rem 0;
    color: #f9fafb;
}

.tech-item p {
    margin: 0;
    color: #9ca3af;
    font-size: 0.875rem;
    line-height: 1.5;
}
"#;

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <meta name="description" content="{}">
    <style>
        {}
        {}
    </style>
    {}
</head>
<body>
    {}
</body>
</html>"#,
        title, description, base_css, extra_css, js_import, content
    )
}

pub fn generate_hybrid_contact_page(
    output_dir: &Path,
    wasm_assets_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔨 Generating: /contact (hybrid with WASM)");

    // Since we just built WASM, there should be exactly one JS file in the fresh build
    let mut js_file = None;
    let mut wasm_file = None;

    println!(
        "🔍 Looking for WASM assets in: {}",
        wasm_assets_dir.display()
    );

    for entry in std::fs::read_dir(wasm_assets_dir)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();

        if file_name.starts_with("dioxus_site-") && file_name.ends_with(".js") {
            println!("📄 Found JS file: {}", file_name);
            // Verify this file has our exports
            let file_path = entry.path();
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                if content.contains("mount_contact_component") {
                    js_file = Some(format!("/assets/{}", file_name));
                    println!("✅ JS file contains mount_contact_component export");
                } else {
                    println!("⚠️  JS file does not contain mount_contact_component export");
                }
            }
        } else if file_name.starts_with("dioxus_site_bg-") && file_name.ends_with(".wasm") {
            wasm_file = Some(format!("/assets/{}", file_name));
            println!("🦀 Found WASM file: {}", file_name);
        }
    }

    let js_path = js_file.ok_or("JS file with mount_contact_component export not found")?;
    let wasm_path = wasm_file.ok_or("WASM file not found")?;

    println!("🎯 Using JS file: {}", js_path);
    println!("🎯 Using WASM file: {}", wasm_path);

    let content = format!(
        r#"<div id="navbar">
        <a href="/">Home</a>
        <a href="/about">About</a>
        <a href="/contact">Contact</a>
        <a href="/blog/1">Blog</a>
    </div>
    <div class="contact-container">
        <header class="contact-header">
            <h1 class="contact-title">Contact Me</h1>
            <p class="contact-subtitle">Get in touch! This form is powered by WebAssembly for interactive functionality.</p>
        </header>

        <div class="contact-content">
            <div class="contact-info">
                <h2>Contact Information</h2>
                <div class="contact-methods">
                    <div class="contact-method">
                        <span class="contact-icon">📧</span>
                        <div>
                            <h3>Email</h3>
                            <a href="mailto:marcin.sydor@sky.uk" class="contact-link">marcin.sydor@sky.uk</a>
                        </div>
                    </div>
                    <div class="contact-method">
                        <span class="contact-icon">💼</span>
                        <div>
                            <h3>LinkedIn</h3>
                            <p>Connect with me professionally</p>
                        </div>
                    </div>
                    <div class="contact-method">
                        <span class="contact-icon">⚡</span>
                        <div>
                            <h3>GitHub</h3>
                            <a href="https://github.com/marcinsydor" target="_blank" class="contact-link">@marcinsydor</a>
                        </div>
                    </div>
                </div>
            </div>

            <div class="contact-form-section">
                <h2>Send a Message</h2>

                <div class="wasm-loading-notice" style="padding: 1rem; margin-bottom: 1rem; background: #f0f9ff; border: 1px solid #0ea5e9; border-radius: 0.5rem; color: #0369a1;">
                    <p style="margin: 0;">🚀 <strong>Interactive WASM Form:</strong> Loading Dioxus Contact component...</p>
                    <div style="width: 100%; height: 4px; background: #e0f2fe; border-radius: 2px; margin-top: 0.5rem; overflow: hidden;">
                        <div style="height: 100%; background: #0ea5e9; animation: loading 2s infinite;"></div>
                    </div>
                </div>

                <!-- This will be replaced by the WASM contact app -->
                <div id="contact-form-placeholder" style="min-height: 400px; display: flex; align-items: center; justify-content: center; background: #f9fafb; border-radius: 0.5rem; border: 2px dashed #d1d5db;">
                    <div style="text-align: center; color: #6b7280;">
                        <div style="font-size: 2rem; margin-bottom: 0.5rem;">⏳</div>
                        <p style="margin: 0;">Initializing interactive Dioxus form...</p>
                    </div>
                </div>
            </div>
        </div>

        <div class="tech-details">
            <h2>🔧 Technical Implementation</h2>
            <div class="tech-grid">
                <div class="tech-item">
                    <h3>🦀 WebAssembly + Dioxus</h3>
                    <p>Interactive form powered by Rust compiled to WASM using the Dioxus framework</p>
                </div>
                <div class="tech-item">
                    <h3>⚡ Reactive State</h3>
                    <p>Real-time form validation and state management using Dioxus signals</p>
                </div>
                <div class="tech-item">
                    <h3>🏗️ Hybrid Architecture</h3>
                    <p>Server-rendered HTML enhanced with client-side WASM using Dioxus components</p>
                </div>
                <div class="tech-item">
                    <h3>📱 Progressive Enhancement</h3>
                    <p>Works with basic HTML, enhanced with dynamic features</p>
                </div>
            </div>
        </div>
    </div>

<script type="module">
    import {{ mount_contact_component, wasm_main }} from '{js_path}';

    async function loadWasmContactForm() {{
        try {{
            console.log('🚀 Loading WASM Contact Form...');

            // Wait for the auto-initialization to complete
            // The module auto-initializes on import, so we need to wait for it
            let retries = 0;
            const maxRetries = 50; // 5 seconds max (50 * 100ms)

            while (!globalThis.__dx_mainWasm && retries < maxRetries) {{
                await new Promise(resolve => setTimeout(resolve, 100));
                retries++;
            }}

            if (!globalThis.__dx_mainWasm) {{
                throw new Error('WASM module failed to initialize within timeout');
            }}

            console.log('✅ WASM module initialized successfully');

            // Initialize the Dioxus runtime
            wasm_main();

            // Wait a bit for WASM initialization
            await new Promise(resolve => setTimeout(resolve, 100));

            // Mount the Dioxus Contact component
            mount_contact_component();
            console.log('✅ Dioxus Contact component mounted');

            // Hide the loading notice
            const loadingNotice = document.querySelector('.wasm-loading-notice');
            if (loadingNotice) {{
                loadingNotice.style.display = 'none';
            }}

        }} catch (error) {{
            console.error('❌ Failed to load WASM Contact Form:', error);

            // Show fallback message
            const placeholder = document.getElementById('contact-form-placeholder');
            if (placeholder) {{
                placeholder.innerHTML = `
                    <div style="padding: 2rem; text-align: center; background: #fef2f2; border: 1px solid #fecaca; border-radius: 0.5rem; color: #dc2626;">
                        <h3>⚠️ Contact Form Loading Error</h3>
                        <p>The interactive contact form failed to load. Please try refreshing the page or contact me directly at <a href="mailto:marcin.sydor@sky.uk">marcin.sydor@sky.uk</a></p>
                        <p><small>Error: ${{error.message || 'WASM module failed to initialize'}}</small></p>
                    </div>
                `;
            }}

            // Hide the loading notice on error too
            const loadingNotice = document.querySelector('.wasm-loading-notice');
            if (loadingNotice) {{
                loadingNotice.style.display = 'none';
            }}
        }}
    }}

    // Load WASM when DOM is ready
    if (document.readyState === 'loading') {{
        document.addEventListener('DOMContentLoaded', loadWasmContactForm);
    }} else {{
        loadWasmContactForm();
    }}
</script>

<noscript>
    <div style="position: fixed; bottom: 1rem; right: 1rem; padding: 1rem; background: #fee; border: 1px solid #fcc; border-radius: 0.5rem; font-size: 0.875rem; max-width: 300px; z-index: 1000;">
        <p style="margin: 0; font-weight: bold; color: #c33;">⚠️ JavaScript Required</p>
        <p style="margin: 0.5rem 0 0 0; color: #c33;">This page requires JavaScript for interactive functionality.</p>
    </div>
</noscript>"#,
        js_path = js_path
    );

    // Add CSS for loading animation
    let additional_css = r#"
    @keyframes loading {
        0% { transform: translateX(-100%); }
        100% { transform: translateX(100%); }
    }

    .contact-form-container {
        animation: fadeIn 0.5s ease-in-out;
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(20px); }
        to { opacity: 1; transform: translateY(0); }
    }
    "#;

    let html_doc = create_html_document_with_css(
        "Contact - Dioxus Site",
        "Get in touch with me through this interactive contact form",
        &content,
        None, // Don't add script tag here - we import it manually in the inline script
        Some(additional_css),
    );

    let contact_dir = output_dir.join("contact");
    std::fs::create_dir_all(&contact_dir)?;
    let index_path = contact_dir.join("index.html");
    std::fs::write(&index_path, html_doc)?;

    println!("✅ Generated: contact/index.html (hybrid with WASM)");
    Ok(())
}
