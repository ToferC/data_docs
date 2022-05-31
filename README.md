# Data Docs

This repo is a learning and test project to transform the massive amount of unstructured word documents we create and use every day into a structured and useful data source that supports:

- [x] Web server using Actix-Web
- [x] Diesel accessing Postgresql DB
- [ ] Docker Containerization
- [ ] Authentication and permissions through Microsoft Active Directory
- [x] Markdown
- [x] Publishing
- [ ] Search
- [x] Proactive disclosure
- [ ] Analytics
- [x] Editing and change history
- [x] Translation
- [ ] Approvals
- [ ] Useful metadata and tagging
- [ ] API (probably GraphQL)

## Why do this?
The Government of Canada spends an incredible amount of time and resources on creating, revising, translating and redacting data in the form of documents. This work is resource intensive and high value, but despite decades of information management systems and practices, the overall value of this resource isn't apparent to the organizations. 

By treating documents as data and building a workflow and analytics flow around information and decision focused communications, we can improve the organizations capacity for knowledge sharing and translation, build connections across our organizations and reduce waste and duplication while turning this daily work into a source of data for the organization.


## Setup
* Clone the repository
* Create a database on Postgresql 13 or 14
* Create `.env` file with the following environmental variables:
    * COOKIE_SECRET_KEY (at least 32 characters - base key for document encryption. Should be rotated and pulled from system.)
    * DATABASE_URL
    * SENDGRID_API_KEY
    * DEEPL_API_KEY
    * ADMIN_NAME
    * ADMIN_EMAIL
    * ADMIN_PASSWORD
    * SECRET_KEY (at least 32 characters - base key for document encryption. Should be rotated and pulled from system.)
* Install `diesel_cli`
* From repo root $ `diesel migration run`
* From repo root $ `cargo run`