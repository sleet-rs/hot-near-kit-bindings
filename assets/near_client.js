import { nearClient } from "https://esm.sh/@sleet-js/fastintear-custom-client-setup-with-other-custom-functions";
// ========================
// near_auth_status
export function near_auth_status() {
  return nearClient().authStatus();
}
// near_request_sign_in
export function near_request_sign_in() {
  return nearClient().requestSignIn();
}
// near_account_id
export function near_account_id() {
  return nearClient().accountId();
}
// near_sign_out
export function near_sign_out() {
  return nearClient().signOut();
}
// ========================
