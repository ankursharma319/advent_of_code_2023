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

function get_sum_contribution_of_gear(i, j, my_array) {
    let sum = 1;
    const my_set = new Set();
    let dirs = [[-1,-1], [-1,0], [-1,1], [0,-1], [0,1], [1,-1], [1,0], [1,1]];
    for (const [x,y] of dirs) {
        // console.log("x=", x, "y=", y);
        if (
            (i+y < my_array.length) &&
            (i+y >= 0) &&
            (j+x < my_array[0].length) &&
            (j+x >= 0) &&
            my_array[i+y][j+x]
        ) {
            let string = JSON.stringify(my_array[i+y][j+x]);
            if (!my_set.has(string)) {
                sum *= my_array[i+y][j+x].num;
            }
            my_set.add(string);
        }
    }
    // console.log("my_set=", my_set);
    if (my_set.size != 2) {
        return 0;
    }
    // console.log("sum contribution of gear at i=", i, ", j=", j, " is", sum);
    return sum;
}

function solve(input_str) {
    const lines = input_str.split(/\r?\n/);
    if (lines[lines.length-1].length == 0) {
        // console.log("Remove last empty line");
        lines.pop();
    }
    // console.log("lines.length = ", lines.length)

    let my_array = new Array(lines.length).fill().map(() => Array(lines[0].length).fill(null));
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
                my_array[i][j] = {"num": current_num, "start_row":i, "start_col":left_boundary}
                j++;
            }
            for (let k=left_boundary; k<j; k++) {
                my_array[i][k].num = current_num;
            }
        }
    }

    // console.log("my_array =", my_array);
    assert.equal(lines.length, my_array.length);
    assert.equal(lines[0].length, my_array[0].length);

    let sum = 0;
    for (let i = 0; i < my_array.length; i++) {
        let my_row = my_array[i];
        for (let j=0; j < my_row.length; j++) {
            if (lines[i][j] != '*') {
                continue;
            }
            // console.log("found gear at i=", i, "j=", j);
            sum += get_sum_contribution_of_gear(i, j, my_array);
        }
    }
    return sum;
}

assert.equal(467835, solve(example));

fs.readFile("input.txt", "utf8", function(err, data) {
    if (err) throw err;
    let answer = solve(data);
    console.log("Problems solution is ", answer);
});

