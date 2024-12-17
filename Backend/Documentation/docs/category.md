# Category Management

## Create New Category `POST`
#### API
```
/category_create
```
#### Request
```json
{
    "email": "wick@example.com",
    "nickname": "TBD",
    "category_type": "weapon",
    "budget": 12345.678,
    "budget_freq": "daily"
}
```
#### Response:
- Successfully created: 
    - `STATUS_CODE`: `CREATED (201)` 
    - `Message`: "Successfully created `category_nickname`"
- Failed to create: 
    - No `email` found in `user` table
        - `STATUS_CODE`: `BAD_REQUEST (400)`
        - `Message`: "No user found for the provided email"
    - `category_nickname` already exists for current `user`
        - `STATUS_CODE`: `BAD_REQUEST (400)` 
        - `Message`: Failed to create new category: duplicate nicknames

## Get Category Overview for User `GET`
#### API
```
/category_summary?email=wick@example.com
```
#### Response:
- Email found: `STATUS_CODE`: `OK (200)`
```Json
[
    {
        "email": "wick@example.com",
        "nickname": "TBD",
        "category_type": "weapon",
        "budget": 12345.678,
        "budget_freq": "daily"
    },
    {
        "email": "wick@example.com",
        "nickname": "food",
        "category_type": "expense",
        "budget": 100.00,
        "budget_freq": "daily"
    }
]
```
- Email Not found
    - `STATUS_CODE`: `OK (200)`
    - return empty list in body
```json
[]
```

## Delete a category for User `DELETE`
#### API
```
/delete_category?email=<user_email_addr>&category_nickname=<nickname>
```
#### Response:
- Email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No user found for the provided email"
- Account not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Message`: "No such category found for the provided email"
- Account not found:
    - `STATUS_CODE`: `OK (200)`
    - `Message`: "Category successfully deleted"

## Update a category for User `UPDATE`
#### API
```
/category_update?email=<user_email>&field=<field_to_update>&category_nickname=<nickname>&new_value=<new_value>
```
- note: the `category_nickname` is the OLD name of the category.
#### Response:
- Email not found: 
    - `STATUS_CODE`: `BAD REQUEST (400)`
    - Message: "No user found for the provided email"
- No match to update:  
    - `STATUS_CODE`: `BAD REQUEST (400)`
    - Message: "Failed to update category: unable to match existing entry"
- Invalid field specified:
    - `STATUS_CODE`: `BAD REQUEST (400)`
    - Message: "Invalid field specified."
- Successfully updated:  `STATUS_CODE`: `OK (200)`