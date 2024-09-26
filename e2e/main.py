import unittest
import requests


class ProfileApiE2ETest(unittest.TestCase):
    HOST = "http://127.0.0.1:3000"
    PROFILE_ENDPOINT = "/api/v1/profiles"
    PROFILE_PRODUCT_REGISTRATION_ENDPOINT = "/api/v1/profiles/{}/product_registrations"

    @classmethod
    def setUpClass(cls):
        cls.maxDiff = None

    def test_profile_api(self):
        expected = {
            "page": 0,
            "items": [
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
        self.assertDictEqual(res.json(), expected)

    def test_profile_api_out_of_range_page(self):
        page = 10
        expected = {
            "page": page,
            "items": [],
        }

        res = requests.get(self.HOST + self.PROFILE_ENDPOINT, params={"page": page})
        self.assertEqual(res.status_code, 200)
        self.assertEqual(res.json(), expected)

    def test_product_registrations_api(self):
        expected = {
            "page": 0,
            "items": [
                {
                    "id": 1,
                    "purchase_date": 1673795045000,
                    "expiry_at": 1705331045000,
                    "product": {"sku": "ARIE4"},
                    "additional_product_registrations": [],
                },
                {
                    "id": 2,
                    "purchase_date": 1678449600000,
                    "expiry_at": None,
                    "product": {"sku": "ARCC4"},
                    "additional_product_registrations": [],
                },
            ],
        }

        res = requests.get(
            self.HOST + self.PROFILE_PRODUCT_REGISTRATION_ENDPOINT.format(1)
        )
        self.assertEqual(res.status_code, 200)
        self.assertDictEqual(res.json(), expected)


if __name__ == "__main__":
    unittest.main()
