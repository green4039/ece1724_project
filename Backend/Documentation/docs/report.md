# Report and Analysis
## Report Overview `GET`
#### API
```
/report_overview?email=<>
```
#### Response:
- Email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Json<Vec<String>>`: Empty
- Successfully extracted:
    - `STATUS_CODE`: `OK (200)`
    - `Json<Vec<String>>`: Example output
```
[
    "Category Summary:",
    "clothes : 1370.34",
    "food : 3751.419999999999",
    "Account Summary:",
    "td_credit: 5121.759999999999"
]
```

## Report Details `GET`
#### API
```
/report_details?email=<>
```
#### Response:
- Email not found:
    - `STATUS_CODE`: `BAD_REQUEST (400)`
    - `Json<Vec<CategorySummary>>`: Empty
- Successfully extraced:
    - `STATUS_CODE`: `OK (200)`
    - `Json<Vec<CategorySummary>>`: Each CategorySummary is in the following format:
    ```
    pub struct CategorySummary {
        pub nickname: String,
        pub budget: f64,
        pub budget_freq: String,
        pub overbudget: bool,
        pub amount: f64,
        pub transaction_idz: Vec<i32>,
        pub cat_trans: Vec<String>,
    }
    ```
    - An example response is provided below:
    ```
[
    {
        "nickname": "food",
        "budget": 100.0,
        "budget_freq": "weekly",
        "overbudget": true,
        "total": 10000003752.42,
        "transaction_idz": [
            1,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            14,
            15,
            16,
            17,
            18,
            19,
            20,
            21,
            22
        ],
        "cat_trans": [
            "2024-12-08 05:07:18.906680 UTC, 456.78, winterlicious",
            "2024-12-08 05:31:26.645759 UTC, 100, EMT",
            "2024-12-08 05:31:29.750300 UTC, 100, EMT",
            "2024-12-08 05:31:33.449547 UTC, 100, EMT",
            "2024-12-08 05:31:34.869775 UTC, 100, EMT",
            "2024-12-08 05:32:39.826673 UTC, 200, EMT",
            "2024-12-08 05:56:55.727 UTC, 200, EMT",
            "2024-12-08 05:56:57.048726 UTC, 200, EMT",
            "2024-12-08 05:56:57.971029 UTC, 200, EMT",
            "2024-12-11 20:49:07.598004 UTC, 4, grocery",
            "2024-12-11 20:55:42.214059 UTC, 354, grocery",
            "2024-12-11 20:55:43.662435 UTC, 354, grocery",
            "2024-12-13 03:28:03.679604 UTC, 456.78, winterlicious",
            "2024-12-13 03:28:05.358450 UTC, 456.78, winterlicious",
            "2024-12-13 03:28:06.424495 UTC, 456.78, winterlicious",
            "2024-12-14 01:07:02.168398 UTC, 12.3, uber eats",
            "2024-12-14 02:31:17.177553 UTC, 10000000000, typo for ubereats; checking updates",
            "2024-12-14 13:15:12.190082 UTC, 1, whatever"
        ]
    },
    {
        "nickname": "clothes",
        "budget": 12345.678,
        "budget_freq": "monthly",
        "overbudget": false,
        "total": 1370.34,
        "transaction_idz": [
            11,
            12,
            13
        ],
        "cat_trans": [
            "2024-12-11 20:39:56.714375 UTC, 456.78, Uniqlo",
            "2024-12-11 20:39:58.473895 UTC, 456.78, Uniqlo",
            "2024-12-11 20:40:00.615091 UTC, 456.78, Uniqlo"
        ]
    }
]
    ```