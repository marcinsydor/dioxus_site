//! Static site generator for Dioxus site
//! This binary generates static HTML files for all routes

use dioxus::prelude::*;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Starting static site generation...");

    // Create output directory
    let output_dir = Path::new("static_output");
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;
    fs::create_dir_all(output_dir.join("assets"))?;

    // Generate static pages
    generate_home_page(output_dir)?;
    generate_about_page(output_dir)?;
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
        content,
        "Home - Dioxus Site",
        "Welcome to my Dioxus-powered website",
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
        &content,
        "About - Dioxus Site",
        "Learn more about me and my work",
    );

    let about_dir = output_dir.join("about");
    fs::create_dir_all(&about_dir)?;
    fs::write(about_dir.join("index.html"), html)?;
    println!("✅ Generated: about/index.html");
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
            &content,
            &format!("Blog Post {} - Dioxus Site", id),
            &format!("Blog post number {}", id),
        );

        let blog_dir = output_dir.join("blog").join(id.to_string());
        fs::create_dir_all(&blog_dir)?;
        fs::write(blog_dir.join("index.html"), html)?;
        println!("✅ Generated: blog/{}/index.html", id);
    }

    Ok(())
}

fn create_html_document(body_content: &str, title: &str, description: &str) -> String {
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

fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
