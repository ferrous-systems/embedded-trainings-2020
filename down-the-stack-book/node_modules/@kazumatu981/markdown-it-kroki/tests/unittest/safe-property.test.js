const { expect } = require('chai');
const { safeProperty } = require('../../lib/safe-property');

describe('# [unit-test]: safe-property.js', () => {
    [
        {
            testCaseDescription: "standard test - string",
            testCase: {
                test: {
                    property1: "hello"
                },
                name: "property1",
                type: "string",
                defaultValue: undefined
            },
            expected: "hello"
        },
        {
            testCaseDescription: "standard test - boolean",
            testCase: {
                test: {
                    property1: true
                },
                name: "property1",
                type: "boolean",
                defaultValue: undefined
            },
            expected: true
        },
        {
            testCaseDescription: "standard test - boolean on null",
            testCase: {
                test: {
                    property1: null
                },
                name: "property1",
                type: "boolean",
                defaultValue: false
            },
            expected: false
        },
        {
            testCaseDescription: "on null",
            testCase: {
                test: {
                    property1: null
                },
                name: "property1",
                type: "string",
                defaultValue: "hello"
            },
            expected: "hello"
        },
        {
            testCaseDescription: "on empty string",
            testCase: {
                test: {
                    property1: ''
                },
                name: "property1",
                type: "string",
                defaultValue: "hello"
            },
            expected: "hello"
        },
        {
            testCaseDescription: "on undefined",
            testCase: {
                test: {
                    property1: undefined
                },
                name: "property1",
                type: "string",
                defaultValue: "hello"
            },
            expected: "hello"
        },
        {
            testCaseDescription: "on not mutch type",
            testCase: {
                test: {
                    property1: 1
                },
                name: "property1",
                type: "string",
                defaultValue: "hello"
            },
            expected: "hello"
        },
        {
            testCaseDescription: "on object is null",
            testCase: {
                test: null,
                name: "property1",
                type: "string",
                defaultValue: "hello"
            },
            expected: "hello"
        },
        {
            testCaseDescription: "on object is undefined",
            testCase: {
                test: undefined,
                name: "property1",
                type: "string",
                defaultValue: "hello"
            },
            expected: "hello"
        },
    ].forEach((testItem) => {
        it(`* ${testItem.testCaseDescription}`, () => {
            const actual = safeProperty(
                testItem.testCase.test,
                testItem.testCase.name,
                testItem.testCase.type,
                testItem.testCase.defaultValue);
            expect(actual).to.equal(testItem.expected);
        });
    })
})
