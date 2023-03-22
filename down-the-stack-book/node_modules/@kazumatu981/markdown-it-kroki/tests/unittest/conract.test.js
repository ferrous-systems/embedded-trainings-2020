const expect = require('chai').expect;
const contract = require('../../lib/contract');

describe('# [unit test]: contract.js', () => {
    describe('## toNonEmptyString()', () => {
        ['abc', ' ', '  abc  '].forEach((test) => {
            it(`* normal cases. { testcase: ${test} }`, () => {
                expect(() => {
                    contract.toNonEmptyString(test);
                }).not.to.throw();
            })
        });
        ['', null, undefined, 123, 0.123, true, { test: 123 }].forEach((test) => {
            it(`* abnormal cases. { testcase: ${test} }`, () => {
                expect(() => {
                    contract.toNonEmptyString(test);
                }).to.throw();
            })
        });
    });
    describe('## toTrue', () => {
        it('* normal case. {test case: true}', () => {
            expect(() => {
                contract.toTrue(true);
            }).not.to.throw();
        });
        ['', null, undefined, 123, 0.123, false, { test: 123 }].forEach((test) => {
            it(`* abnormal cases. { testcase: ${test} }`, () => {
                expect(() => {
                    contract.toTrue(test);
                }).to.throw();
            })
        });
    });
})