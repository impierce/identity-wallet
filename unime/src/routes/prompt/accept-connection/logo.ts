import type { Certification } from '$lib/dev/accept-connection.types';
import { hash } from '$lib/utils';

/**
 * The asset id for a certification's logo, or `undefined` when it has none.
 *
 * The backend downloads the logo to `assets/tmp/<hash(url)>`, so we re-hash the same URL to
 * find it. Which field carries that URL is still in flux, so both call sites go through here.
 */
export const certificationLogoId = (certification: Certification): string | undefined => {
  // `data` is `any` on the wire, so guard rather than trust the shape.
  const image = certification.credential.data?.credentialSubject?.image;
  return typeof image === 'string' ? hash(image) : undefined;
};
