.PHONY: build build-web build-hybrid generate-static deploy publish clean help

# Build the Dioxus site for web (SPA mode)
build-web:
	@echo "🧹 Cleaning dx build artifacts..."
	rm -rf target/dx/dioxus_site
	@echo "🔨 Building Dioxus site for web..."
	dx build --release

# Generate static HTML files for all routes (True SSG)
generate-static:
	@echo "🧹 Cleaning static output..."
	rm -rf static_output
	@echo "🏗️  Generating static site..."
	cargo run --bin generate_static --features ssr
	@echo "✅ Static site generation complete!"

# Build hybrid site (static + interactive Contact page with WASM)
build-hybrid:
	@echo "🧹 Cleaning all build artifacts..."
	rm -rf static_output
	rm -rf target/dx/dioxus_site
	rm -rf docs/assets/dioxus_site*.js
	rm -rf docs/assets/dioxus_site*.wasm
	@echo "🔧 Building interactive Contact page with WASM..."
	dx build --release --features web
	@echo "🏗️  Generating static site (except contact)..."
	cargo run --bin generate_static --features ssr -- --skip-contact
	@echo "📦 Adding WASM assets to static output..."
	mkdir -p static_output/assets
	cp -r target/dx/dioxus_site/release/web/public/assets/* static_output/assets/
	@echo "🏗️  Generating hybrid Contact page with WASM..."
	cargo run --bin generate_hybrid_contact --features ssr
	@echo "✅ Hybrid build complete! Static pages + Interactive Contact with WASM"

# Build static site (default)
build: generate-static

# Clean and copy static files to docs folder (static build)
deploy: build
	@echo "🧹 Cleaning docs folder..."
	rm -rf docs
	mkdir -p docs
	@echo "📦 Copying static files to docs..."
	cp -r static_output/* docs/
	@echo "📄 Copying robots.txt..."
	cp assets/robots.txt docs/
	@echo "🌐 Copying CNAME..."
	cp CNAME docs/
	@echo "✅ True SSG deployment preparation complete!"
	@echo "📂 Static HTML files are ready in the docs/ folder"
	@echo ""
	@echo "Generated files:"
	@find docs -name "*.html" -exec echo "  📄 {}" \;

# Deploy hybrid build with WASM-enabled contact page
deploy-hybrid: build-hybrid
	@echo "🧹 Cleaning docs folder completely..."
	rm -rf docs
	mkdir -p docs
	@echo "🧹 Removing any old WASM/JS files..."
	find . -name "dioxus_site-*.js" -not -path "./target/dx/*" -not -path "./static_output/*" -delete 2>/dev/null || true
	find . -name "dioxus_site_bg-*.wasm" -not -path "./target/dx/*" -not -path "./static_output/*" -delete 2>/dev/null || true
	@echo "📦 Copying hybrid static files to docs..."
	cp -r static_output/* docs/
	@echo "📄 Copying robots.txt..."
	cp assets/robots.txt docs/
	@echo "🌐 Copying CNAME..."
	cp CNAME docs/
	@echo "✅ Hybrid deployment preparation complete!"
	@echo "📂 Hybrid site (static + WASM contact) ready in the docs/ folder"
	@echo ""
	@echo "Generated files:"
	@find docs -name "*.html" -exec echo "  📄 {}" \;
	@echo ""
	@echo "WASM files:"
	@find docs -name "*.wasm" -exec echo "  🦀 {}" \;
	@find docs -name "*dioxus_site*.js" -exec echo "  📄 {}" \;

# Build, deploy, and automatically commit and push to GitHub
publish: deploy
	@echo "🚀 Committing and pushing to GitHub..."
	git add docs/
	git commit -m "deploy static site $$(date '+%Y-%m-%d %H:%M:%S')"
	git push origin main
	@echo "✅ Published to GitHub!"

# Build hybrid, deploy, and automatically commit and push to GitHub
publish-hybrid: deploy-hybrid
	@echo "🚀 Committing and pushing hybrid site to GitHub..."
	git add docs/
	git commit -m "deploy hybrid site (static + WASM contact) $$(date '+%Y-%m-%d %H:%M:%S')"
	git push origin main
	@echo "✅ Hybrid site published to GitHub!"

# Clean all build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf target/
	rm -rf static_output/
	rm -rf docs
	@echo "🧹 Removing old WASM/JS files..."
	find . -name "dioxus_site-*.js" -delete 2>/dev/null || true
	find . -name "dioxus_site_bg-*.wasm" -delete 2>/dev/null || true
	mkdir -p docs
	@echo "✅ Clean complete!"

# Show available commands
help:
	@echo "Available commands:"
	@echo "  make build         - Generate static site (SSG) - default"
	@echo "  make build-web     - Build Dioxus site for web (SPA)"
	@echo "  make build-hybrid  - Build static site + interactive Contact page with WASM"
	@echo "  make generate-static - Generate static HTML files for all routes"
	@echo "  make deploy        - Build static site and prepare for GitHub Pages"
	@echo "  make deploy-hybrid - Build hybrid site and prepare for GitHub Pages"
	@echo "  make publish       - Build, deploy, commit and push to GitHub"
	@echo "  make publish-hybrid - Build hybrid, deploy, commit and push to GitHub"
	@echo "  make clean         - Clean all build artifacts"
	@echo "  make help          - Show this help message"
	@echo ""
	@echo "🏗️  True Static Site Generation (SSG):"
	@echo "  - Each route gets its own HTML file"
	@echo "  - No JavaScript required for basic navigation"
	@echo "  - Perfect for SEO and static hosting"
	@echo "  - Works with JavaScript disabled"
	@echo ""
	@echo "🚀 Hybrid Build (build-hybrid):"
	@echo "  - All pages are static HTML"
	@echo "  - Contact page gets interactive WASM functionality"
	@echo "  - Best of both worlds: performance + interactivity"
