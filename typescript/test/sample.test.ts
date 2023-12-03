import { expect } from "chai";
import * as fs from "fs";

function extract_calibration_value(input: string) {
    const first = finds_first_number(input)
    const last = finds_last_number(input)
    return first + last
}

function finds_first_number(input: string) {
    const match = input.match(/[0-9]/);
    if (match) {
        return match[0]
    }
    return "0"
}

function finds_last_number(input: string) {
    const match = input.match(/[0-9]/g);
    if (match) {
        return match[match?.length - 1];
    }
    return "0"
}


describe("aoc day 1", () => {
    it("finds calibration value from start and end", () => {
        const input = "1abc2"
        const expected = "12"
        const output = extract_calibration_value(input)
        expect(output).is.eql(expected);
    });
    it("finds calibration value from middle", () => {
        const input = "pqr3stu8vwx"
        const expected = "38"
        const output = extract_calibration_value(input)
        expect(output).is.eql(expected);
    });
    it("finds first number in string", () => {
        const input = "pqr3stu8vwx"
        const expected = "3"
        const output = finds_first_number(input)
        expect(output).is.eql(expected);
    });

    it("finds last number in string", () => {
        const input = "pqr3stu8vwx"
        const expected = "8"
        const output = finds_last_number(input)
        expect(output).is.eql(expected);
    });
    it("finds calibration value with multiple middle values", () => {
        const input = "a1b2c3d4e5f"
        const expected = "15"
        const output = extract_calibration_value(input)
        expect(output).is.eql(expected);
    });
    it("finds calibration value with 1 value", () => {
        const input = "treb7uchet"
        const expected = "77"
        const output = extract_calibration_value(input)
        expect(output).is.eql(expected);
    });

    it("finds calibration value with empty string", () => {
        const input = ""
        const expected = "00"
        const output = extract_calibration_value(input)
        expect(output).is.eql(expected);
    });

    it("finds sum of calibration values", () => {
        const input = `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`;
        const expected = "142"
        const output = day_1(input)
        expect(output.toString()).is.eql(expected);
    });

    it("calculates day 1 result",() => {
        const input = fs.readFileSync("/home/mch/Work/advent-of-code-2023/data/day-1-input.txt").toString()
        const output = day_1(input)
        expect(output.toString()).is.eql("54877");
    }) 
});

function day_1(input: string) {
    const lines = input.split('\n');
    const calibration_values = lines.map(extract_calibration_value)
    let output = 0;
    for (const value of calibration_values) {
        output = output + Number.parseInt(value)
    } return output.toString()
}