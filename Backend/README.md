```text
 ______  __  __   __  ______  ______  ______  ______  __  __    
/\  ___\/\ \/\ "-.\ \/\__  _\/\  == \/\  __ \/\  ___\/\ \/ /    
\ \  __\\ \ \ \ \-.  \/_/\ \/\ \  __<\ \  __ \ \ \___\ \  _"-.  
 \ \_\   \ \_\ \_\\"\_\ \ \_\ \ \_\ \_\ \_\ \_\ \_____\ \_\ \_\ 
  \/_/    \/_/\/_/ \/_/  \/_/  \/_/ /_/\/_/\/_/\/_____/\/_/\/_/
```
# FinTrack Backend

## [Online Documentation](https://fintrak-solutions.github.io/Backend/)

---
## 🌐 **API**

### 🔐 **Authentication**
| **API**                                    | **Status**  | **Time Finished**  | **Link to Docs**                            |
|--------------------------------------------|-------------|---------------------|--------------------------------------------|
| `/signup`                                  | ✅ Complete | 2024-12-07 2:00pm   | [View Docs](authentication/#signup-post)   |

---

### 📘 **Account Management**
| **API**                                    | **Status**  | **Time Finished**  | **Link to Docs**                                      |
|--------------------------------------------|-------------|---------------------|-----------------------------------------------------|
| `/account_create`                          | ✅ Complete | 2024-12-07 3:10pm   | [View Docs](account/#create-new-account-for-user-post)|
| `/account_summary?email=<>`                | ✅ Complete | 2024-12-07 3:40pm   | [View Docs](account/#get-account-overview-for-user-get)|
| `/delete_account?email=<>&account_name=<>` | ✅ Complete | 2024-12-07 4:20pm   | [View Docs](account/#delete-an-account-for-user-delete)|
| `/account_detail`                          | ❌ In Progress | TBD               | [View Docs](account/#get-account-detailed-view-for-user-get) |

---

### 📦 **Category Management**
| **API**                                    | **Status**   | **Time Finished**  | **Link to Docs**                                        |
|--------------------------------------------|--------------|---------------------|-------------------------------------------------------|
| `/category_create`                         | ✅ Complete  | 2024-12-07          | [View Docs](category/#create-new-category-post)         |
| `/category_summary?email=<>`               | ✅ Complete  | 2024-12-07          | [View Docs](category/#get-category-overview-for-user-get)|
| `/delete_category?email=<>&category_nickname=<>` | ✅ Complete | 2024-12-07         | [View Docs](category/#delete-a-category-for-user-delete)|
| `/category_update?email=<>&field=<field_to_update>&category_nickname=<>&new_value=<>`| ✅ Complete | 2024-12-07         | [View Docs](category/#update-a-category-for-user-update) |

---

### 💸 **Transaction Management**
| **API**                                    | **Status**    | **Time Finished**  | **Link to Docs**                          |
|--------------------------------------------|---------------|---------------------|------------------------------------------|
| `/add_trans` | ✅ Complete | 2024-12-07 [View Docs](https://github.com/FinTrak-Solutions/Backend/blob/main/Documentation/docs/transaction.md#create-new-transaction-post) |
| `/delete_trans?trans_id=<>` | ✅ Complete | 2024-12- 07 [View Docs](https://github.com/FinTrak-Solutions/Backend/blob/main/Documentation/docs/transaction.md#delete-transaction-delete) |
| `/category_trans?category_name=<>&email=<>` | ✅ Complete | 2024-12-11 [View Docs](https://github.com/FinTrak-Solutions/Backend/blob/main/Documentation/docs/transaction.md#get-category-transactions-get) |
| `/account_trans?account_name=<>&email=<>` | ✅ Complete | 2024-12-11 [View Docs](https://github.com/FinTrak-Solutions/Backend/blob/main/Documentation/docs/transaction.md#get-account-transactions-get) |

---

## 📮 **Postman API Testing**
To explore and test the API endpoints, you can check out the Postman API documentation [here](https://web.postman.co/workspace/46a5447a-bfb7-47fa-8a8b-0da03a25416e/collection/40276125-9521e786-da55-44fd-9b33-98f4b67d293e) (localhost version).

---
## ⚡ **Quick Start**

### 🔥 **Clone the project**
```bash
git clone https://github.com/FinTrak-Solutions/Backend.git
```

---

### 📦 **Install Virtual Environment**
```bash
cd Backend
virtualenv venv
source venv/bin/activate
pip3 install mkdocs
```

---

### 📁 **Go to the Documentation Directory**
```bash
cd Documentation
```

---

### 📝 **Modify `.md` files in `docs/`**
```bash
# Check and update locally
mkdocs serve
# Modify .md files in the docs/ folder
```

---

### 🚀 **Build and Deploy**
```bash
# Build and deploy
mkdocs build
mkdocs gh-deploy
```

---

## Create new database using `PostgreSQL`
```shell
psql -U postgres

# PostgreSQL prompt
CREATE DATABASE financial_tracker_db;
# Verify
\l 
# Exit
\q
```