#!/usr/bin/env python3
"""
Script to add SPA (Single Page Application) routing support to Dioxus generated HTML.
This fixes direct URL access issues when deployed to GitHub Pages.
"""

import os
import sys


def add_spa_routing(html_file_path):
    """Add SPA routing script to the HTML file."""

    # SPA redirect script to inject
    spa_script = """    <!-- Start Single Page Apps for GitHub Pages -->
    <script type="text/javascript">
      // Single Page Apps for GitHub Pages
      // MIT License
      // https://github.com/rafgraph/spa-github-pages
      (function(l) {
        if (l.search[1] === '/' ) {
          var decoded = l.search.slice(1).split('&').map(function(s) {
            return s.replace(/~and~/g, '&')
          }).join('?');
          window.history.replaceState(null, null,
              l.pathname.slice(0, -1) + decoded + l.hash
          );
        }
      }(window.location))
    </script>
    <!-- End Single Page Apps for GitHub Pages -->"""

    try:
        # Read the HTML file
        with open(html_file_path, "r", encoding="utf-8") as f:
            content = f.read()

        # Check if SPA script is already present
        if "Single Page Apps for GitHub Pages" in content:
            print(f"✅ SPA routing script already present in {html_file_path}")
            return True

        # Insert the script before </head>
        if "</head>" in content:
            content = content.replace("</head>", f"{spa_script}\n</head>")
        else:
            print(f"❌ Could not find </head> tag in {html_file_path}")
            return False

        # Write the modified content back
        with open(html_file_path, "w", encoding="utf-8") as f:
            f.write(content)

        print(f"✅ Added SPA routing script to {html_file_path}")
        return True

    except Exception as e:
        print(f"❌ Error processing {html_file_path}: {e}")
        return False


def main():
    """Main function."""
    if len(sys.argv) != 2:
        print("Usage: python add-spa-routing.py <html-file-path>")
        sys.exit(1)

    html_file = sys.argv[1]

    if not os.path.exists(html_file):
        print(f"❌ File not found: {html_file}")
        sys.exit(1)

    if add_spa_routing(html_file):
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main()
