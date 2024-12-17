# Account Management
## Create New Account for User `POST`
#### API
```
/account_create
```
#### Request:
```Json
{
    "email": "test@example.com",
    "type": "credit",
    "account_name": "td_credit"
}
```
#### Response:
- Successfully created: 
    - `STATUS_CODE`: `CREATED (201)` 
    - `Message`: "Successfully created `account_name`"
- Failed to create: 
    - No `email` found in `user` table
        - `STATUS_CODE`: `BAD_REQUEST (400)`
        - `Message`: "No user found for the provided email"
    - `account_name` already exists for current `user`
        - `STATUS_CODE`: `BAD_REQUEST (400)` 
        - `Message`: "Failed to create new account"
        
        

## Get Account Overview for User `GET`
#### API
```
/account_summary?email=wick@example.com
```
#### Response:
- Email found
    - `STATUS_CODE`: `OK (200)`
```Json
[
  {
    "account_id": 1,
    "email": "wick@example.com",
    "account_type": "credit",
    "account_name": "td_credit"
  },
  {
    "account_id": 2,
    "email": "wick@example.com",
    "account_type": "credit",
    "account_name": "bmo_credit"
  }
]
```
- Email Not found
    - `STATUS_CODE`: `OK (200)`
    - return empty list in body
```json
[]
```

## Delete an account for User `DELETE`
#### API
```
/delete_account?email=<>&account_name=<>
```
#### Response:
- Email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No user found for the provided email"
- Account not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No such account found for the provided email"
- Account found:
    - `STATUS_CODE`: `OK (200)`
    - `Message`: "Account successfully deleted"


## Get Account Detailed View for User `GET`
#### API
> **NOTE:** This interface will be handled by transaction details section.
