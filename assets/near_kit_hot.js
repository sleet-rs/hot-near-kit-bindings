import {
  near_connect_client,
} from "https://esm.sh/@sleet-js/near-kit-custom-client-setup-with-other-custom-functions@0.0.8";
// ================================================

// near_auth_status - returns "SignedIn" or "NotSignedIn"
export async function near_auth_status() {
  const wallet = await near_connect_client().wallet();
  const accounts = await wallet.getAccounts();
  if (accounts && accounts.length > 0) {
    return "SignedIn";
  }
  return "NotSignedIn";
}

// near_request_sign_in - connects to wallet
export async function near_request_sign_in() {
  await near_connect_client().connect();
  return "Connected";
}

// near_account_id - returns the account ID or empty string
export async function near_account_id() {
  const wallet = await near_connect_client().wallet();
  const accounts = await wallet.getAccounts();
  if (accounts && accounts.length > 0) {
    return accounts[0].accountId;
  }
  return "";
}

// near_sign_out - signs out from wallet
export async function near_sign_out() {
  const wallet = await near_connect_client().wallet();
  wallet.signOut();
  return "Disconnected";
}

// ================================================
