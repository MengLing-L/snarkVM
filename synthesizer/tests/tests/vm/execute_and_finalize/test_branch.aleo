/*
randomness: 98372
cases:
  - function: run_test
    inputs: [1u8, 1u8]
  - function: run_test
    inputs: [0u8, 1u8]
  - function: run_test
    inputs: [0u8, 0u8]
*/

program test_branch.aleo;

function run_test:
    input r0 as u8.public;
    input r1 as u8.public;
    finalize r0 r1;

finalize run_test:
    input r0 as u8.public;
    input r1 as u8.public;
    branch.eq r0 0u8 to exit_one;
    assert.eq true false;
    position exit_one;
    branch.neq r1 1u8 to exit_two;
    assert.eq true false;
    position exit_two;

