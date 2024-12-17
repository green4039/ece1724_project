[x] Investigate and select the right packages
- https://crates.io/crates/rusqlite (for SQL)
- https://crates.io/crates/directories (for persistence)
- https://crates.io/crates/dirs (similar to the above)
[ ] Table schema design: to be checked with team
[x] Save and load schemas from file
[x] Convert the existing code to fit struct + method format
[ ] Functions to interact with local DB (Simple CRUD)
- [ ] Function to create tables (might be one time set up)
    - Persistence: read, modify, close, and then reload
- [ ] Function to read tables
- [ ] Function to update tables
- [ ] Function to delete table entries
- [ ] How to export for other modules in this project to use