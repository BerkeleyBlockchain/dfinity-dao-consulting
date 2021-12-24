export const toDate = (dateInt: bigint): Date =>
  new Date(Number(dateInt / 1000n));
