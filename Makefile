.PHONY: build deploy publish clean help

# Build the Dioxus site for SSG (Static Site Generation)
build:
	@echo "🧹 Cleaning dx build artifacts..."
	rm -rf target/dx/dioxus_site
	@echo "🔨 Building Dioxus site for SSG..."
	dx build --release --ssg

# Clean and copy built files to docs folder
deploy: build
	@echo "🧹 Cleaning docs folder..."
	rm -rf docs
	mkdir -p docs
	@echo "📦 Copying built files to docs..."
	cp -r target/dx/dioxus_site/release/web/public/* docs/
	@echo "📄 Copying robots.txt..."
	cp assets/robots.txt docs/
	@echo "🌐 Copying CNAME..."
	cp CNAME docs/
	@echo "🔧 Setting up SPA routing for GitHub Pages..."
	cp assets/404.html docs/
	@echo "🧩 Adding SPA redirect script to index.html..."
	python3 scripts/add-spa-routing.py docs/index.html
	@echo "✅ Build and deploy preparation complete!"
	@echo "📂 Files are ready in the docs/ folder"

# Build, deploy, and automatically commit and push to GitHub
publish: deploy
	@echo "🚀 Committing and pushing to GitHub..."
	git add docs/
	git commit -m "deploy site $$(date '+%Y-%m-%d %H:%M:%S')"
	git push origin main
	@echo "✅ Published to GitHub!"

# Clean build artifacts and docs folder
clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf target/
	rm -rf docs
	mkdir -p docs
	@echo "✅ Clean complete!"

# Show available commands
help:
	@echo "Available commands:"
	@echo "  make build   - Build the Dioxus site for SSG"
	@echo "  make deploy  - Build and prepare files for GitHub Pages"
	@echo "  make publish - Build, deploy, commit and push to GitHub"
	@echo "  make clean   - Clean build artifacts and docs folder"
	@echo "  make help    - Show this help message"
