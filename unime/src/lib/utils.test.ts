import { hash } from './utils';

describe('hashing', () => {
  test('should return the expected hash digest', () => {
    expect(hash('foobar')).toEqual('c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2');
  });
});
