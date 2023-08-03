export interface InitialConnection {
  issuer: {
    id: string;
    domain: string;
    did: string;
  };
}

export interface CredentialOffer {
  issuer: {
    id: string;
  };
  credentials: string[]; // ids
}
