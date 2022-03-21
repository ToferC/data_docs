# Data Docs

This repo is a learning and test project to transform the massive amount of unstructured word documents we create and use every day into a structured and useful data source that supports:

- [x] Web server using Actix-Web
- [x] Diesel accessing Postgresql DB
- [ ] Docker Containerization
- [ ] Authentication and permissions through Microsoft Active Directory
- [x] Markdown
- [ ] Publishing
- [ ] Search
- [ ] Proactive disclosure
- [ ] Analytics
- [ ] Editing and change history
- [ ] Translation
- [ ] Approvals
- [ ] Useful metadata and tagging
- [ ] API (probably GraphQL)

## Setup
* Clone the repository
* Create a database on Postgresql 13 or 14
* Create `.env` file with the following environmental variables:
    * COOKIE_SECRET_KEY
    * DATABASE_URL
    * SENDGRID_API_KEY
    * DEEPL_API_KEY
    * ADMIN_NAME
    * ADMIN_EMAIL
    * ADMIN_PASSWORD
* Install `diesel_cli`
* From repo root $ `diesel migration run`
* From repo root $ `cargo run`