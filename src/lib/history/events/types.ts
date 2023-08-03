export interface InitialConnection {
  issuer: Issuer;
}

export interface CredentialOffer {
  issuer: Issuer;
  credentials: string[]; // ids
}

export interface Login {
  verifier: Verifier;
  credentials: string[]; // ids
}

interface Issuer {
  id: string;
  domain: string;
  did: string;
}

interface Verifier {
  id: string;
  domain: string;
  did: string;
}
