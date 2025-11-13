Frontend: cto-parallel-test (React + Vite + shadcn/ui)

Summary
- React app scaffolded with Vite, Tailwind CSS, and shadcn/ui components.
- Pages: Home, Product List, Product Detail, Cart, Login, Register.
- Routing via react-router-dom; Header with cart badge; Footer included.
- API integration is parameterized via environment (no hardcoded endpoints).

Run Locally
- cd frontend
- cp .env.example .env and set VITE_API_BASE_URL to your backend (e.g., https://api.example.com)
- npm install
- npm start
- Open http://localhost:3000

Security Notes
- No secrets checked in. API base URL is provided through env.
- Path and URL handling: ids are validated; API paths use encodeURIComponent; prevents traversal and XSS in URLs.
- Forms: inputs trimmed; sensitive fields (password) never logged or stored.
- Dependencies are current; npm audit shows no runtime vulnerabilities at the time of build.

Configuration
- Dev server port: 3000 (npm start)
- Alias: @ -> src (configured in vite.config.js and jsconfig.json)
- shadcn/ui style: New York; components installed: button, card, badge, input, form, navigation-menu

Code Scanning
- Use GitHub code scanning on the PR once created:
  gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=<PR_NUMBER>"

