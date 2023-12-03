
const assert = require('node:assert');
const fs = require("fs");

console.log("Hello world!");

const example = `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`;

function is_digit(char) {
    return (char >= '0') && (char <= '9');
}

function is_symbol(char) {
    if (char === '.') {
        return false;
    }
    if (is_digit(char)) {
        return false;
    }
    return true;
}

function solve(input_str) {
    const lines = input_str.split(/\r?\n/);
    if (lines[lines.length-1].length == 0) {
        // console.log("Remove last empty line");
        lines.pop();
    }
    // console.log("lines.length = ", lines.length)
    let sum = 0;
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        // console.log("Processing line#", i, "of size", line.length)
        let j = 0;
        while(j<line.length) {
            if (!is_digit(line[j])) {
                j++;
                continue;
            }
            let current_num = 0;
            let left_boundary = j;
            while (is_digit(line[j])) {
                current_num = current_num*10 + (line[j]-'0');
                j++;
            }

            let found_symbol = false;

            // check above
            if (i>0) {
                for(let k=left_boundary-1; k<=j; k++) {
                    if (k >=0 && k < line.length && is_symbol(lines[i-1][k])) {
                        found_symbol = true;
                        break;
                    }
                }
            }

            // check below
            if (!found_symbol && i<lines.length-1) {
                for(let k=left_boundary-1; k<=j; k++) {
                    if (k >=0 && k < line.length && is_symbol(lines[i+1][k])) {
                        found_symbol = true;
                        break;
                    }
                }
            }

            // check left
            if (!found_symbol && left_boundary>0) {
                for(let k=i-1; k<=i+1; k++) {
                    if (k>=0 && k<lines.length && is_symbol(lines[k][left_boundary-1])) {
                        found_symbol = true;
                        break;
                    }
                }
            }

            // check right
            if (!found_symbol && j<line.length) {
                for(let k=i-1; k<=i+1; k++) {
                    if (k>=0 && k<lines.length && is_symbol(lines[k][j])) {
                        found_symbol = true;
                        break;
                    }
                }
            }

            if (found_symbol) {
                sum += current_num;
            }
        }
    }
    return sum;
}

assert.equal(4361, solve(example));

fs.readFile("input.txt", "utf8", function(err, data) {
    if (err) throw err;
    let answer = solve(data);
    assert.notEqual(481110, answer);
    console.log("Problems solution is ", answer);
});

