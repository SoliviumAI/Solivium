from fastapi.testclient import TestClient
from main import app

client = TestClient(app)

def test_get_balance():
    response = client.get("/balance/YourWalletAddress")
    assert response.status_code == 200
    assert "wallet_address" in response.json()
