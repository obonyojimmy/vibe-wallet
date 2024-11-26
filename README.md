# Vibe Wallet - Escrow Program

**Vibe Wallet** is a Solana-based escrow program that facilitates secure transactions between clients and service providers (e.g., escorts). The program ensures that funds are held in escrow until the transaction is verified, providing a secure and reliable payment mechanism.

---

## Features

1. **Secure Escrow Initialization**  
   Funds are securely transferred into an escrow PDA (Program Derived Address) account tied to a unique booking ID.

2. **Verification Mechanism**  
   Funds are released to the service provider only upon successful verification of a one-time verification code.

3. **Automatic Account Closure**  
   The escrow PDA is closed automatically once the funds are released.

4. **Commission Fee**  
   The program deducts a 5% commission on every transaction before releasing funds.

---

## Account Structures

### `EscrowAccount`

The account holding escrow transaction details.

| Field         | Type     | Description                                           |
|---------------|----------|-------------------------------------------------------|
| `client`      | `Pubkey` | The client initiating the escrow.                     |
| `escort`      | `Pubkey` | The service provider receiving the funds.             |
| `booking_id`  | `String` | A unique 32-character identifier for the booking.     |
| `verify_code` | `String` | A 6-character verification code for fund release.     |

---

## Program Instructions

### 1. `initialize_escrow`

Initializes an escrow transaction and transfers funds into a PDA account.

#### Parameters:
- `booking_id` (`String`): Unique identifier for the booking (max 32 characters).
- `verify_code` (`String`): Verification code for the transaction (max 6 characters).
- `amount` (`u64`): Amount of funds to transfer into escrow (in lamports).

#### Accounts:
- `escrow_account`: PDA account derived from `booking_id`, `client`, and `escort`.
- `client`: The funder of the escrow.
- `escort`: The recipient of the escrow.
- `system_program`: Solana's system program.

---

### 2. `release_escrow`

Releases funds from the escrow account to the service provider upon verification of the code. Deducts a 5% commission.

#### Parameters:
- `verify_code` (`String`): The verification code for fund release.

#### Accounts:
- `escrow_account`: The PDA holding the escrowed funds.
- `signer`: The service provider's account (must match `escort`).
- `system_program`: Solana's system program.

---

## Commission Logic

The program deducts a 5% commission from the escrowed amount before releasing the remaining funds to the service provider. The deducted commission is transferred to a pre-defined treasury wallet.

---

## Deployment

To deploy the program on Solana, follow these steps:

1. **Build and Deploy**  
   Run the following commands to build and deploy the program:
   ```bash
   anchor build
   anchor deploy
   ```

2. **Fetch Program ID**  
   Copy the generated program ID and update the `declare_id!` macro in the program.

3. **Generate PDA**  
   Use the seeds `[b"escrow", booking_id.as_bytes(), client.key.as_ref()]` to derive the PDA for escrow accounts.

---

## Usage Example

### Initialize Escrow
```bash
solana program invoke \
  --program-id <PROGRAM_ID> \
  --instruction '{"initialize_escrow": {"booking_id": "abc123", "verify_code": "123456", "amount": 1000000000}}' \
  --accounts '{"client": "<CLIENT_PUBLIC_KEY>", "escort": "<ESCORT_PUBLIC_KEY>", "system_program": "<SYSTEM_PROGRAM>"}'
```

### Release Escrow
```bash
solana program invoke \
  --program-id <PROGRAM_ID> \
  --instruction '{"release_escrow": {"verify_code": "123456"}}' \
  --accounts '{"escrow_account": "<PDA>", "signer": "<ESCORT_PUBLIC_KEY>", "system_program": "<SYSTEM_PROGRAM>"}'
```

---

## Testing

Unit tests are written using Anchor’s testing framework. To run tests:

```bash
anchor test
```

---

## Roadmap

- **Integration with Wallet Adapters**  
  Streamline interaction with wallets like Phantom and Solflare.

- **Escrow Cancellation**  
  Add functionality to allow cancellation of pending transactions.

- **Event Notifications**  
  Notify clients and service providers upon fund release or transaction updates.

---

## Contributions

Contributions, issues, and feature requests are welcome. Feel free to fork this repository and submit pull requests.

---

## License

This project is licensed under the [MIT License](LICENSE).