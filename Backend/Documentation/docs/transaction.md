# Transaction Management

## Create New Transaction `POST`
#### API
```
/add_trans
```
#### Request
```json
{
    "email": "wick@example.com",
    "category_name": "food",
    "amount": 456.78,
    "notes": "winterlicious",
    "account_name": "td_debit"
}
```
#### Response:
- Email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No user found for the provided email"
- Account not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No category found for the provided email"
- Category not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No account found for the provided email"
- Successfully added:
    - `STATUS_CODE`: `CREATED (200)`
    - `Message`: new transaction ID as string.

## Delete Transaction `DELETE`
#### API
```
/delete_trans?trans_id=<>
```
#### Response:
- ID not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No transaction found for the provided ID"
- Transaction successfully deleted:
    - `STATUS_CODE`: `OK (200)`
    - `Message`: "No transaction found for the provided ID"
- Otherwise:
    - `STATUS_CODE`: `INTERNAL_SERVER_ERROR (500)`
    - `Message`: "Failed to delete the transaction"

## Get Category Transactions `GET`
#### API
```
/category_trans?category_name=<>&email=<>
```
#### Response:
- email or category name not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Json<Vec<Transaction>>`: Empty
- transactions successfully extracted:
    - `STATUS_CODE`: `OK (200)`
    - `Json<Vec<Transaction>>`

## Get Account Transactions `GET`
#### API
```
/account_trans?account_name=<>&email=<>
```
#### Response:
- account name or email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Json<Vec<Transaction>>`: Empty
- transactions successfully extracted:
    - `STATUS_CODE`: `OK (200)`
    - `Json<Vec<Transaction>>`