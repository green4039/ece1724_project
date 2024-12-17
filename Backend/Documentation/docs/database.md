# Database Schema Documentation

## Introduction

This document outlines the updated database schema for the financial tracker application backend, utilizing Rust Rocket for REST API handling, Diesel for database interaction, and PostgreSQL as the database. The schema is designed to efficiently store and manage `user information`, `financial accounts`, `transactions`, `budgets`, and `categories`, ensuring data scalability and consistency.

---

## Table of Contents

1. [User Table](#user-table)
2. [Account Table](#account-table)
3. [Transaction Table](#transaction-table)
4. [Category Table](#category-table)
5. [Summary of Updates](#summary-of-updates)

---

## User Table

### Description

Stores user credentials and personal information. Each user is uniquely identified and can have multiple accounts, transactions, and categories associated with them.

### Schema

| Field Name  | Data Type         | Constraints       | Description                              |
|-------------|-------------------|-------------------|------------------------------------------|
| `id`        | `SERIAL`          | Primary Key       | Unique identifier for each user.         |
| `email`     | `TEXT`            | Unique, Not Null  | User's email address.                    |
| `password`  | `TEXT`            | Not Null          | Hashed password for security.            |
| `username`  | `TEXT`            | Not Null          | User's unique username.                  |

---

## Account Table

### Description

Represents financial accounts linked to users. Each account belongs to a user and holds financial transactions.

### Schema

| Field Name      | Data Type         | Constraints                         | Description                              |
|-----------------|-------------------|-------------------------------------|------------------------------------------|
| `account_id`    | `SERIAL`          | Primary Key                         | Unique identifier for each account.      |
| `email`         | `TEXT`            | Foreign Key (`User.email`), Not Null | Email of the account owner.              |
| `account_type`  | `TEXT`            | Not Null                            | Type of the account (e.g., `Credit`, `Debit`, `Savings`). |
| `account_name`  | `TEXT`            | Not Null                            | A descriptive name for the account.      |

---

## Transaction Table

### Description

Logs all financial transactions associated with accounts. Each transaction records details like amount, timestamp, and category.

### Schema

| Field Name    | Data Type         | Constraints                         | Description                              |
|---------------|-------------------|-------------------------------------|------------------------------------------|
| `trans_id`    | `SERIAL`          | Primary Key                         | Unique identifier for each transaction.  |
| `email`       | `TEXT`            | Foreign Key (`User.email`), Not Null | Email of the user linked to the transaction. |
| `category_id` | `INTEGER`         | Foreign Key (`Category.category_id`) | Category classification for the transaction. |
| `amount`      | `FLOAT8`          | Not Null                            | Transaction amount (positive or negative). |
| `notes`       | `TEXT` (nullable) | Optional                            | Additional notes for the transaction.    |

---

## Category Table

### Description

Provides a way to classify transactions into categories, allowing for detailed financial tracking and reporting.

### Schema

| Field Name      | Data Type         | Constraints                  | Description                              |
|-----------------|-------------------|------------------------------|------------------------------------------|
| `category_id`   | `SERIAL`          | Primary Key                  | Unique identifier for each category.     |
| `email`         | `TEXT`            | Foreign Key (`User.email`), Not Null | Email of the user who owns the category. |
| `nickname`      | `TEXT`            | Not Null                     | A descriptive name for the category.     |
| `category_type` | `TEXT`            | Not Null                     | Type of category (e.g., `Food`, `Bills`).|
| `budget`        | `FLOAT8`          | Optional                     | Budget associated with this category.    |
| `budget_freq`   | `TEXT`            | Optional                     | Budget frequency (`Daily`, `Weekly`, etc.).|

---

## Summary of Updates

1. **Primary Keys**: Added `id` fields as primary keys in tables where they were missing.
2. **Foreign Key Relationships**:
   - `email` links multiple tables to the `users` table for referential integrity.
   - `category_id` links the `transactions` table to the `categories` table.
3. **Field Updates**:
   - Added `account_name` to the `accounts` table for better account identification.
   - Ensured foreign key constraints for better data consistency.
4. **Enumerations and Constraints**:
   - Categories and budgets allow classification for better financial tracking.
   - Transaction amounts support both positive and negative values.
5. **Scalability and Performance**:
   - Schema supports detailed tracking and reporting, optimized for scalability and extensibility.

---
