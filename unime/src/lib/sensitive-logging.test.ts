import { sanitizeStringify } from './sensitive-logging';

describe('sensitive-logging', () => {
  test('sensitive data should not be logged', () => {
    const sensitiveValue = 'should_not_log';

    const testee = {
      foo: 'bar',
      password: sensitiveValue,
    };

    const sanitized = sanitizeStringify(testee);
    const unaltered = JSON.stringify(testee);

    expect(sanitized).not.toContain(sensitiveValue);
    expect(unaltered).toContain(sensitiveValue);
  });

  test('sensitive data nested in object should not be logged', () => {
    const sensitiveValue1 = 'should_not_log_1';
    const sensitiveValue2 = 'should_not_log_2';
    const sensitiveValue3 = 'should_not_log_3';

    const testee = {
      foo: 'bar',
      password: sensitiveValue1,
      data: {
        secret: sensitiveValue2,
        foobar: {
          secret: sensitiveValue3,
        },
      },
    };

    const sanitized = sanitizeStringify(testee);
    const unaltered = JSON.stringify(testee);

    expect(sanitized).not.toContain(sensitiveValue1);
    expect(sanitized).not.toContain(sensitiveValue2);
    expect(sanitized).not.toContain(sensitiveValue3);

    expect(unaltered).toContain(sensitiveValue1);
    expect(unaltered).toContain(sensitiveValue2);
    expect(unaltered).toContain(sensitiveValue3);
  });
});
