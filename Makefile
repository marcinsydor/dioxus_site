.PHONY: build build-web generate-static deploy publish clean help

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

# Build static site (default)
build: generate-static

# Clean and copy static files to docs folder
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

# Build, deploy, and automatically commit and push to GitHub
publish: deploy
	@echo "🚀 Committing and pushing to GitHub..."
	git add docs/
	git commit -m "deploy static site $$(date '+%Y-%m-%d %H:%M:%S')"
	git push origin main
	@echo "✅ Published to GitHub!"

# Clean all build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf target/
	rm -rf static_output/
	rm -rf docs
	mkdir -p docs
	@echo "✅ Clean complete!"

# Show available commands
help:
	@echo "Available commands:"
	@echo "  make build         - Generate static site (SSG) - default"
	@echo "  make build-web     - Build Dioxus site for web (SPA)"
	@echo "  make generate-static - Generate static HTML files for all routes"
	@echo "  make deploy        - Build static site and prepare for GitHub Pages"
	@echo "  make publish       - Build, deploy, commit and push to GitHub"
	@echo "  make clean         - Clean all build artifacts"
	@echo "  make help          - Show this help message"
	@echo ""
	@echo "🏗️  True Static Site Generation (SSG):"
	@echo "  - Each route gets its own HTML file"
	@echo "  - No JavaScript required for basic navigation"
	@echo "  - Perfect for SEO and static hosting"
	@echo "  - Works with JavaScript disabled"
