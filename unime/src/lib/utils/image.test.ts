import { ensureLightIcon } from './image';

describe('logo', () => {
  test('should return undefined for no input', () => {
    expect(ensureLightIcon(undefined)).toEqual(undefined);
  });

  test('should return undefined for an unknown icon', () => {
    expect(ensureLightIcon('foobar')).toEqual(undefined);
  });

  test('should return `UserLight` for `User`', () => {
    expect(ensureLightIcon('User')).toEqual('UserLight');
  });

  test('should return `HouseLight` for `HouseLight`', () => {
    expect(ensureLightIcon('HouseLight')).toEqual('HouseLight');
  });
});
