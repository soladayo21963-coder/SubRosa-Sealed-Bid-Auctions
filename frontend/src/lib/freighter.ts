import { isConnected, requestAccess, getAddress } from "@stellar/freighter-api";

export async function connectWallet(): Promise<string | null> {
  if (!(await isConnected())) {
    return null;
  }

  const access = await requestAccess();
  if (!access) {
    return null;
  }

  const result = await getAddress();
  return result.address ?? null;
}
