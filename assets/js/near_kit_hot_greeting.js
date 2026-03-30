import { greeting_get_greeting_fun, greeting_set_greeting_fun } from "https://esm.sh/@near-kit-tool-box/fun";
import { near_kit_client } from "https://esm.sh/@near-kit-tool-box/web";
// ================================================
const near = near_kit_client();
// ================================================
export async function greeting_get_greeting(greeting_contractId) {
  return await greeting_get_greeting_fun(near, greeting_contractId);
}
// ================================================
export async function greeting_set_greeting(greeting_contractId, greeting) {
  return await greeting_set_greeting_fun(near, greeting_contractId, greeting);
}
// ================================================
