/**
 * placeholder for backend's `TrustVerification` type.
 */

export type TrustStatus = 'Verified' | 'Invalid' | 'Unknown';

export interface TrustVerification {
  status: TrustStatus;
  ecosystem_name: string | null;
  ecosystem_id: string | null;
  ecosystem_logo_uri: string | null;
  previously_trusted: boolean;
}
