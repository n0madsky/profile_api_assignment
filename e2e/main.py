import unittest
import requests


class ProfileApiE2ETest(unittest.TestCase):
    HOST = "http://127.0.0.1:3000"
    PROFILE_ENDPOINT = "/api/v1/profiles"

    def test_profile_api(self):
        expected = {
            "page": 0,
            "profiles": [
                {
                    "id": 1,
                    "email": "john.doe@example.com",
                    "firstname": "John",
                    "lastname": "Doe",
                    "product_registrations": [],
                },
                {
                    "id": 2,
                    "email": "jane.smith@example.com",
                    "firstname": "Jane",
                    "lastname": "Smith",
                    "product_registrations": [],
                },
            ],
        }

        res = requests.get(self.HOST + self.PROFILE_ENDPOINT)
        self.assertEqual(res.status_code, 200)
        self.assertEqual(res.json(), expected)

    def test_profile_api_out_of_range_page(self):
        page = 10
        expected = {
            "page": page,
            "profiles": [],
        }

        res = requests.get(self.HOST + self.PROFILE_ENDPOINT, params={"page": page})
        self.assertEqual(res.status_code, 200)
        self.assertEqual(res.json(), expected)


if __name__ == "__main__":
    unittest.main()
