from fastapi import FastAPI
from solana.rpc.api import Client

app = FastAPI()
solana_client = Client("https://api.mainnet-beta.solana.com")

@app.get("/balance/{wallet_address}")
def get_balance(wallet_address: str):
    balance = solana_client.get_balance(wallet_address)
    return {"wallet_address": wallet_address, "balance": balance["result"]["value"] / 1e9}

@app.get("/")
def read_root():
    return {"message": "Welcome to Solivium AI Assistant"}
